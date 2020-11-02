//! Grammar definitions and syntactic analysis.
//!
//! # Remarks
//! - Note that the [`Pointer`] type syntax differs compared to Rust, C, or C++.
//! - Likewise, the [`AddressOf`] operator is also different.
//! - Expressions are currently lisp-based, but this will change later on.
//!
//! [`Pointer`]: ./struct.Pointer.html
//! [`AddressOf`]: ./expressions/struct.AddressOf.html
use crate::{
    error::Error,
    lex,
    lex::{
        span::{union, Span, Spanned},
        Ident, Token, Tokens,
    },
};
pub use context::{Context, ContextBuilder};
pub use expression::Expression;
use std::iter::Peekable;
pub use types::Type;

#[macro_use]
mod macros;
#[doc(hidden)]
pub mod asm;
mod context;
pub mod expression;
pub mod types;

/// Abstract syntax tree.
#[derive(Debug)]
pub struct Ast<'a> {
    pub inner: Vec<Statement<'a>>,
    pub eof: lex::Eof<'a>,
}

impl<'a> Grammar<'a> for Ast<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let inner = Grammar::parse(context, tokens)?;
        let eof = Grammar::parse(context, tokens)?;
        Ok(Self { inner, eof })
    }
}

impl Spanned for Ast<'_> {
    fn span(&self) -> Span {
        let mut span = self.eof.span();
        if let Some(s) = self.inner.first() {
            span = union(&span, &s.span());
        }
        span
    }
}

/// Parse input source code.
pub fn parse(input: &str) -> Result<Ast, Error> {
    let mut context = ContextBuilder::default().build();
    parse_with_context(input, &mut context)
}

/// Parse input source code with a context.
pub fn parse_with_context<'a>(input: &'a str,
                              context: &mut Context<'a>)
                              -> Result<Ast<'a>, Error<'a>> {
    let mut tokens = Tokens::new(input).peekable();
    Grammar::parse(context, &mut tokens)
}

/// Trait for parseable types.
pub trait Grammar<'a>: Sized {
    /// Parse stream of tokens into a type.
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>>;
}

// For parsing no-arguments (performs a no-op).
// Mainly used on the Struct statement, which behaves like a statement when it
// has an identifier, and like a type when it doesn't have.
impl<'a> Grammar<'a> for () {
    fn parse(_: &mut Context<'a>, _: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        Ok(())
    }
}

// Parsing of boxed types for self-referential types.
// Mostly used within expression syntax trees.
impl<'a, P: Grammar<'a>> Grammar<'a> for Box<P> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        P::parse(context, tokens).map(Box::new)
    }
}

// Parsing of boxed types for self-referential types.
// Mostly used within expression syntax trees.
impl<'a, P> Grammar<'a> for Vec<P> where Option<P>: Grammar<'a>
{
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let mut vec = Vec::new();
        while let Some(item) = Grammar::parse(context, tokens)? {
            vec.push(item);
        }
        Ok(vec)
    }
}

#[derive(Debug)]
pub struct Path<'a> {
    pub head: lex::Ident<'a>,
    pub tail: Vec<(lex::Square<'a>, lex::Ident<'a>)>,
}

impl<'a> From<lex::Ident<'a>> for Path<'a> {
    fn from(head: Ident<'a>) -> Self {
        Self { head,
               tail: Vec::new() }
    }
}

impl Spanned for Path<'_> {
    fn span(&self) -> Span {
        let mut span = self.head.span();
        if let Some((_, t)) = self.tail.last() {
            span = union(&span, &t.span());
        }
        span
    }
}

impl Path<'_> {
    /// Returns an ordered iterator over the separated items of type `T`.
    pub fn iter(&self) -> impl Iterator<Item = &lex::Ident> {
        let tail_iter = self.tail.iter().map(|(_, ref t)| t);
        Some(&self.head).into_iter().chain(tail_iter)
    }

    /// Returns the total number of items of type `T`.
    pub fn len(&self) -> usize {
        self.tail.len() + 1
    }
}

impl<'a> Grammar<'a> for Path<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let head = Grammar::parse(context, tokens)?;
        let mut tail = Vec::new();
        while let Some(sep) = Grammar::parse(context, tokens)? {
            let item = Grammar::parse(context, tokens)?;
            tail.push((sep, item))
        }
        Ok(Self { head, tail })
    }
}

parse_enum! {
    /// Program statements.
    #[derive(Debug)]
    pub enum Statement<'a> {
        If(If<'a>),
        IfElse(IfElse<'a>),
        Scope(Scope<'a>),
        Panic(Panic<'a>),
        Mod(Mod<'a>),
        Struct(Struct<'a, lex::Ident<'a>>),
        Union(Union<'a, lex::Ident<'a>>),
        Asm(Asm<'a>),
        Static(Static<'a>),
        Const(Const<'a>),
        For(For<'a>),
        Loop(Loop<'a>),
        Let(Let<'a>),
        Fn(Fn<'a>),
        Inline(Inline<'a>),
        Continue(Continue<'a>),
        Break(Break<'a>),
        Return(Return<'a>),
    }
}

impl<'a> Grammar<'a> for Option<Statement<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        match tokens.peek() {
            None => Ok(None),
            Some(Ok(Token::If(_))) => {
                let if_ = Grammar::parse(context, tokens)?;
                if let Some(Ok(Token::Else(_))) = tokens.peek() {
                    let else_ = Grammar::parse(context, tokens)?;
                    Ok(Some(Statement::IfElse(IfElse { if_, else_ })))
                } else {
                    Ok(Some(Statement::If(if_)))
                }
            }
            Some(Ok(Token::LeftBracket(_))) => {
                Ok(Some(Statement::Scope(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::BangBang(_))) => {
                Ok(Some(Statement::Panic(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Mod(_))) => Ok(Some(Statement::Mod(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Struct(_))) => {
                Ok(Some(Statement::Struct(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Union(_))) => {
                Ok(Some(Statement::Union(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Asm(_))) => Ok(Some(Statement::Asm(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Static(_))) => {
                Ok(Some(Statement::Static(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Const(_))) => {
                Ok(Some(Statement::Const(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::For(_))) => Ok(Some(Statement::For(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Loop(_))) => Ok(Some(Statement::Loop(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Let(_))) => Ok(Some(Statement::Let(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Fn(_))) => Ok(Some(Statement::Fn(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Continue(_))) => {
                Ok(Some(Statement::Continue(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Break(_))) => {
                Ok(Some(Statement::Break(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Return(_))) => {
                Ok(Some(Statement::Return(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::RightBracket(_))) | Some(Ok(Token::Eof(_))) => Ok(None),
            Some(Ok(_)) => Ok(Some(Statement::Inline(Grammar::parse(context, tokens)?))),
            // Token error
            Some(Err(_)) => Err(tokens.next().unwrap().err().expect("Expected token error")),
        }
    }
}

impl<'a> Grammar<'a> for Statement<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken { token,
                                         expected: None })
        }
    }
}

/// `{ <inner> }`
#[derive(Debug)]
pub struct Scope<'a> {
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<Statement<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl Spanned for Scope<'_> {
    fn span(&self) -> Span {
        union(&self.left_bracket.span(), &self.right_bracket.span())
    }
}

impl<'a> Grammar<'a> for Scope<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { left_bracket,
                  inner,
                  right_bracket })
    }
}

/// `!!`
#[derive(Debug)]
pub struct Panic<'a> {
    pub bang_bang: lex::BangBang<'a>,
}

impl<'a> Grammar<'a> for Panic<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let bang_bang = Grammar::parse(context, tokens)?;
        Ok(Self { bang_bang })
    }
}

impl Spanned for Panic<'_> {
    fn span(&self) -> Span {
        self.bang_bang.span()
    }
}

/// `asm { (<asm>)* }`
#[derive(Debug)]
pub struct Asm<'a> {
    pub asm: lex::Asm<'a>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<asm::Asm<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl Spanned for Asm<'_> {
    fn span(&self) -> Span {
        union(&self.asm.span(), &self.right_bracket.span())
    }
}

impl<'a> Grammar<'a> for Asm<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let asm = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { asm,
                  left_bracket,
                  inner,
                  right_bracket })
    }
}

/// `mod <ident> { <statements> }`
#[derive(Debug)]
pub struct Mod<'a> {
    pub mod_: lex::Mod<'a>,
    pub ident: lex::Ident<'a>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<Statement<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl Spanned for Mod<'_> {
    fn span(&self) -> Span {
        union(&self.mod_.span(), &self.right_bracket.span())
    }
}

impl<'a> Grammar<'a> for Mod<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let mod_ = Grammar::parse(context, tokens)?;
        let ident = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { mod_,
                  ident,
                  left_bracket,
                  inner,
                  right_bracket })
    }
}

/// `<expressions> ;`
#[derive(Debug)]
pub struct Inline<'a> {
    pub inner: Expression<'a>,
}

impl Spanned for Inline<'_> {
    fn span(&self) -> Span {
        self.inner.span()
    }
}

impl<'a> Grammar<'a> for Inline<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let inner = Grammar::parse(context, tokens)?;
        Ok(Self { inner })
    }
}

/// `@ <expressions>`
#[derive(Debug)]
pub struct StaticOffset<'a> {
    pub at: lex::At<'a>,
    pub expression: Expression<'a>,
}

impl<'a> Grammar<'a> for StaticOffset<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let at = Grammar::parse(context, tokens)?;
        let expression = Grammar::parse(context, tokens)?;
        Ok(Self { at, expression })
    }
}

impl<'a> Grammar<'a> for Option<StaticOffset<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::At(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

/// `static [<offset>] <field> ;`
#[derive(Debug)]
pub struct Static<'a> {
    pub static_: lex::Static<'a>,
    pub offset: Option<StaticOffset<'a>>,
    pub field: Field<'a>,
}

impl<'a> Grammar<'a> for Static<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let static_ = Grammar::parse(context, tokens)?;
        let offset = Grammar::parse(context, tokens)?;
        let field = Grammar::parse(context, tokens)?;
        Ok(Self { static_,
                  offset,
                  field })
    }
}

impl Spanned for Static<'_> {
    fn span(&self) -> Span {
        union(&self.static_.span(), &self.field.span())
    }
}

/// `const <field> = <expressions> ;`
#[derive(Debug)]
pub struct Const<'a> {
    pub const_: lex::Const<'a>,
    pub field: Field<'a>,
    pub assign: lex::Assign<'a>,
    pub expression: Expression<'a>,
}

impl<'a> Grammar<'a> for Const<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let const_ = Grammar::parse(context, tokens)?;
        let field = Grammar::parse(context, tokens)?;
        let assign = Grammar::parse(context, tokens)?;
        let expression = Grammar::parse(context, tokens)?;
        Ok(Self { const_,
                  field,
                  assign,
                  expression })
    }
}

impl Spanned for Const<'_> {
    fn span(&self) -> Span {
        union(&self.const_.span(), &self.expression.span())
    }
}

/// `let <field> = <expressions> ;`
#[derive(Debug)]
pub struct Let<'a> {
    pub let_: lex::Let<'a>,
    pub field: Field<'a>,
    pub assign: lex::Assign<'a>,
    pub expression: Expression<'a>,
}

impl<'a> Grammar<'a> for Let<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let let_ = Grammar::parse(context, tokens)?;
        let field = Grammar::parse(context, tokens)?;
        let assign = Grammar::parse(context, tokens)?;
        let expression = Grammar::parse(context, tokens)?;
        Ok(Let { let_,
                 field,
                 assign,
                 expression })
    }
}

impl Spanned for Let<'_> {
    fn span(&self) -> Span {
        union(&self.let_.span(), &self.expression.span())
    }
}

/// `if <expressions> { <inner> } else { <inner> }`
#[derive(Debug)]
pub struct IfElse<'a> {
    pub if_: If<'a>,
    pub else_: Else<'a>,
}

impl<'a> Grammar<'a> for IfElse<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let if_ = Grammar::parse(context, tokens)?;
        let else_ = Grammar::parse(context, tokens)?;
        Ok(Self { if_, else_ })
    }
}

impl Spanned for IfElse<'_> {
    fn span(&self) -> Span {
        union(&self.if_.span(), &self.else_.span())
    }
}

/// `if <expressions> { <inner> }`
#[derive(Debug)]
pub struct If<'a> {
    pub if_: lex::If<'a>,
    pub expression: Expression<'a>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<Statement<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl Spanned for If<'_> {
    fn span(&self) -> Span {
        union(&self.if_.span(), &self.right_bracket.span())
    }
}

impl<'a> Grammar<'a> for If<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let if_ = Grammar::parse(context, tokens)?;
        let expression = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { if_,
                  expression,
                  left_bracket,
                  inner,
                  right_bracket })
    }
}

/// `else { <inner> }`
#[derive(Debug)]
pub struct Else<'a> {
    pub else_: lex::Else<'a>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<Statement<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<'a> Grammar<'a> for Else<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let else_ = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { else_,
                  left_bracket,
                  inner,
                  right_bracket })
    }
}

impl Spanned for Else<'_> {
    fn span(&self) -> Span {
        union(&self.else_.span(), &self.right_bracket.span())
    }
}

/// `<left> .. [=] [+] <right>`
#[derive(Debug)]
pub struct Range<'a, L, R> {
    pub left: L,
    pub dot_dot: lex::DotDot<'a>,
    pub eq: Option<lex::Assign<'a>>,
    pub plus: Option<lex::Plus<'a>>,
    pub right: R,
}

impl<'a, L: Grammar<'a>, R: Grammar<'a>> Grammar<'a> for Range<'a, L, R> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let left = Grammar::parse(context, tokens)?;
        let dot_dot = Grammar::parse(context, tokens)?;
        let eq = Grammar::parse(context, tokens)?;
        let plus = Grammar::parse(context, tokens)?;
        let right = Grammar::parse(context, tokens)?;
        Ok(Self { left,
                  dot_dot,
                  eq,
                  plus,
                  right })
    }
}

/// `for <field> in <range> { (<statement>)* }`
#[derive(Debug)]
pub struct For<'a> {
    pub for_: lex::For<'a>,
    pub field: Field<'a>,
    pub in_: lex::In<'a>,
    pub range: Range<'a, Expression<'a>, Expression<'a>>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<Statement<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<'a> Grammar<'a> for For<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let for_ = Grammar::parse(context, tokens)?;
        let field = Grammar::parse(context, tokens)?;
        let in_ = Grammar::parse(context, tokens)?;
        let range = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { for_,
                  field,
                  in_,
                  range,
                  left_bracket,
                  inner,
                  right_bracket })
    }
}

impl Spanned for For<'_> {
    fn span(&self) -> Span {
        union(&self.for_.span(), &self.right_bracket.span())
    }
}

/// `loop { (<statement>)* }`
#[derive(Debug)]
pub struct Loop<'a> {
    pub loop_: lex::Loop<'a>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<Statement<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<'a> Grammar<'a> for Loop<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let loop_ = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { loop_,
                  left_bracket,
                  inner,
                  right_bracket })
    }
}

impl Spanned for Loop<'_> {
    fn span(&self) -> Span {
        union(&self.loop_.span(), &self.right_bracket.span())
    }
}

/// `:: <type>`
#[derive(Debug)]
pub struct FnReturn<'a> {
    pub colon: lex::Colon<'a>,
    pub type_: Type<'a>,
}

impl<'a> Grammar<'a> for FnReturn<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let colon = Grammar::parse(context, tokens)?;
        let type_ = Grammar::parse(context, tokens)?;
        Ok(Self { colon, type_ })
    }
}

impl Spanned for FnReturn<'_> {
    fn span(&self) -> Span {
        union(&self.colon.span(), &self.type_.span())
    }
}

impl<'a> Grammar<'a> for Option<FnReturn<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Colon(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

/// `fn <ident> [<args>] <type> { }`
#[derive(Debug)]
pub struct Fn<'a> {
    pub fn_: lex::Fn<'a>,
    pub ident: lex::Ident<'a>,
    pub fn_arg: Option<FnArg<'a, Vec<Field<'a>>>>,
    pub fn_return: Option<FnReturn<'a>>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: Vec<Statement<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl Spanned for Fn<'_> {
    fn span(&self) -> Span {
        union(&self.fn_.span(), &self.right_bracket.span())
    }
}

impl<'a> Grammar<'a> for Fn<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let fn_ = Grammar::parse(context, tokens)?;
        let ident: lex::Ident<'a> = Grammar::parse(context, tokens)?;
        let fn_arg = Grammar::parse(context, tokens)?;
        let fn_return = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { fn_,
                  ident,
                  fn_arg,
                  fn_return,
                  left_bracket,
                  inner,
                  right_bracket })
    }
}

#[derive(Debug)]
pub struct FnArg<'a, I> {
    pub left_par: lex::LeftPar<'a>,
    pub inner: I,
    pub right_par: lex::RightPar<'a>,
}

impl<'a, I: Grammar<'a>> Grammar<'a> for FnArg<'a, I> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let left_par = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_par = Grammar::parse(context, tokens)?;
        Ok(Self { left_par,
                  inner,
                  right_par })
    }
}

impl<I> Spanned for FnArg<'_, I> {
    fn span(&self) -> Span {
        union(&self.left_par.span(), &self.right_par.span())
    }
}

impl<'a, I: Grammar<'a>> Grammar<'a> for Option<FnArg<'a, I>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::LeftPar(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

/// `struct [<ident>] { <fields> }`
#[derive(Debug)]
pub struct Struct<'a, I> {
    pub struct_: lex::Struct<'a>,
    pub ident: I,
    pub left_bracket: lex::LeftBracket<'a>,
    pub fields: Vec<Field<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<'a> Grammar<'a> for Struct<'a, lex::Ident<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let struct_ = Grammar::parse(context, tokens)?;
        let ident: lex::Ident<'a> = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let fields = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { struct_,
                  ident,
                  left_bracket,
                  fields,
                  right_bracket })
    }
}

impl<'a> Grammar<'a> for Struct<'a, ()> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let struct_ = Grammar::parse(context, tokens)?;
        let ident = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let fields = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { struct_,
                  ident,
                  left_bracket,
                  fields,
                  right_bracket })
    }
}

impl<I> Spanned for Struct<'_, I> {
    fn span(&self) -> Span {
        union(&self.struct_.span(), &self.right_bracket.span())
    }
}

/// `union [<ident>] { <fields> }`
#[derive(Debug)]
pub struct Union<'a, I> {
    pub union: lex::Union<'a>,
    pub ident: I,
    pub left_bracket: lex::LeftBracket<'a>,
    pub fields: Vec<Field<'a>>,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<'a, I: Grammar<'a>> Grammar<'a> for Union<'a, I> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let union = Grammar::parse(context, tokens)?;
        let ident = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let fields = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self { union,
                  ident,
                  left_bracket,
                  fields,
                  right_bracket })
    }
}

impl<I> Spanned for Union<'_, I> {
    fn span(&self) -> Span {
        union(&self.union.span(), &self.right_bracket.span())
    }
}

/// `<ident> :: <type>`
#[derive(Debug)]
pub struct Field<'a> {
    pub ident: lex::Ident<'a>,
    pub colon: lex::Colon<'a>,
    pub type_: Type<'a>,
}

impl Spanned for Field<'_> {
    fn span(&self) -> Span {
        union(&self.ident.span(), &self.type_.span())
    }
}

impl<'a> Grammar<'a> for Field<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let ident = Grammar::parse(context, tokens)?;
        let colon = Grammar::parse(context, tokens)?;
        let type_ = Grammar::parse(context, tokens)?;
        Ok(Self { ident,
                  colon,
                  type_ })
    }
}

impl<'a> Grammar<'a> for Option<Field<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

/// `<ident> ( <ident>)* :: <type>`
#[derive(Debug)]
pub struct FieldGroup<'a> {
    pub head: lex::Ident<'a>,
    pub tail: Vec<lex::Ident<'a>>,
    pub square: lex::Square<'a>,
    pub type_: Type<'a>,
}

impl<'a> Grammar<'a> for FieldGroup<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let head = Grammar::parse(context, tokens)?;
        let tail = Grammar::parse(context, tokens)?;
        let square = Grammar::parse(context, tokens)?;
        let type_ = Grammar::parse(context, tokens)?;
        Ok(Self { head,
                  tail,
                  square,
                  type_ })
    }
}

impl Spanned for FieldGroup<'_> {
    fn span(&self) -> Span {
        union(&self.head.span(), &self.type_.span())
    }
}

impl<'a> Grammar<'a> for Option<FieldGroup<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        match tokens.peek() {
            Some(Ok(Token::Ident(_))) => Ok(Some(Grammar::parse(context, tokens)?)),
            _ => Ok(None),
        }
    }
}

/// `continue ;`
#[derive(Debug)]
pub struct Continue<'a> {
    pub continue_: lex::Continue<'a>,
}

impl Spanned for Continue<'_> {
    fn span(&self) -> Span {
        self.continue_.span()
    }
}

impl<'a> Grammar<'a> for Continue<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let continue_ = Grammar::parse(context, tokens)?;
        Ok(Self { continue_ })
    }
}

/// `break ;`
#[derive(Debug)]
pub struct Break<'a> {
    pub break_: lex::Break<'a>,
}

impl Spanned for Break<'_> {
    fn span(&self) -> Span {
        self.break_.span()
    }
}

impl<'a> Grammar<'a> for Break<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let break_ = Grammar::parse(context, tokens)?;
        Ok(Self { break_ })
    }
}

/// `return <expressions> ;`
#[derive(Debug)]
pub struct Return<'a> {
    pub return_: lex::Return<'a>,
    pub expression: Option<Expression<'a>>,
}

impl Spanned for Return<'_> {
    fn span(&self) -> Span {
        let mut span = self.return_.span();
        if let Some(expression) = &self.expression {
            span = union(&span, &expression.span());
        }
        span
    }
}

impl<'a> Grammar<'a> for Return<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let return_ = Grammar::parse(context, tokens)?;
        let expression = Grammar::parse(context, tokens)?;
        Ok(Self { return_,
                  expression })
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Ast;

    fn parse_program(input: &str) -> Ast {
        super::parse(input).unwrap()
    }

    #[test]
    fn if_else_() {
        parse_program("if 42 { }");
        parse_program("if 42 { } else { }");
    }

    #[test]
    fn scope() {
        parse_program("{}");
        parse_program("{{}{{}}}");
    }

    #[test]
    #[should_panic]
    fn scope_panic() {
        parse_program("{{}{{}}} }");
    }

    #[test]
    fn panic() {
        parse_program("!!");
    }

    #[test]
    fn mod_() {
        parse_program("mod foo { mod bar { } } mod baz { }");
    }

    #[test]
    fn struct_() {
        parse_program("struct Foo { }");
        parse_program("struct Foo { a : u8 }");
        parse_program("struct Foo { a : u8 b : u8 }");
        parse_program("struct Foo { a : u8 b : u8 c : struct { } }");
        parse_program("struct Foo { a : u8 b : u8 c : struct { a:[u8;42] } }");
    }

    #[test]
    fn union_() {
        parse_program("union Foo { }");
        parse_program("union Foo { a : u8 }");
        parse_program("union Foo { a : u8 b : u8 }");
        parse_program("union Foo { a : u8 b : u8 c : union { } }");
        parse_program("union Foo { a : u8 b : u8 c : union { a:[u8;42] } }");
    }

    #[test]
    #[ignore]
    fn asm() {
        parse_program("asm { }");
    }

    #[test]
    #[should_panic]
    fn asm_panic() {
        parse_program("asm { loop { } }");
    }

    #[test]
    fn static_() {
        parse_program("static FOO : [u8; 0x800]");
        parse_program("static@0x8000 FOO : [u8 0x800]");
        parse_program("static@0x8000 FOO : [u8 0x800]");
    }

    #[test]
    #[should_panic]
    fn static_panic() {
        parse_program("static FOO :: u8 = 0");
        parse_program("static FOO :: u8 = 0");
    }

    #[test]
    fn const_() {
        parse_program("const FOO : u8 = 42");
        parse_program("const FOO : u8 = 42");
    }

    #[test]
    #[should_panic]
    fn const_panic() {
        parse_program("const FOO, BAR : u8 = 0");
    }

    #[test]
    fn for_() {
        parse_program("for i:u8 in 0.. 42 { }");
        parse_program("for i:u8 in 0..=42 { }");
        parse_program("for i:u8 in 0..=+42 { }");
    }

    #[test]
    fn loop_() {
        parse_program("loop {}");
        parse_program("loop {} loop {}");
        parse_program("loop {loop{}} loop {}");
    }

    #[test]
    fn let_() {
        parse_program("let foo:u8 = 42");
        parse_program("let foo_bar:u8 = 0xffff");
    }

    #[test]
    #[should_panic]
    fn let_panic() {
        parse_program("let foo = 42;");
    }

    #[test]
    fn fn_() {
        parse_program("fn foo { }");
        parse_program("fn foo:u8 { }");
        parse_program("fn foo(bar:u8) { }");
        parse_program("fn foo(bar:u8):u8 { }");
        parse_program("fn foo(bar:u8 baz:u8):u8 { }");
    }

    #[test]
    #[ignore]
    fn inline_non_lisp() {
        parse_program("1");
        parse_program("foo");
        parse_program("foo()");
        parse_program("foo::bar(0, 1, 2)");
        parse_program("foo = 42");
        parse_program("foo += 42");
    }

    #[test]
    fn local_scope() {
        parse_program(
                      r#"
            static FOO:u8
            let bar:u8 = 0
            loop {
                let baz:u8 = 0
                {
                    FOO
                    bar
                    baz
                }
            }
            FOO
            bar
            fn fun {
                FOO
            }
        "#,
        );
    }

    #[test]
    fn mod_scopes() {
        parse_program(
                      r#"
            mod a {
                mod b {
                    mod c { static foo:u8 }
                    fn test_c { c::foo }
                    mod d { static foo:u8 }
                    fn test_d { d::foo }
                }
                fn test_b_c_and_b_d {
                    b::c::foo
                    b::d::foo
                }
            }
            a::b::c::foo
            a::b::d::foo
        "#,
        );
    }
}
