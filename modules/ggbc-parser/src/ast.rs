//! Grammar definitions and syntactic analysis.
//!
//! # Statements
//! - `S := { S }`
//! - `S := !!` (panic)
//! - `S := mod <ident> { S }`
//! - `S := struct <ident> { <fields> }`
//! - `S := union <ident> { <fields> }`
//! - `S := asm { <instructions> }`
//! - `S := static [@ <lit>] <ident> :: T ;`
//! - `S := const <ident> :: T = E ;`
//! - `S := for <ident> :: T in E .. [=] [+] E { S }`
//! - `S := loop { S }`
//! - `S := let <ident> :: T = E ;`
//! - `S := fn <ident> [( <fields> )] [T] { S }`
//! - `S := E ;`
//!
//! # Types
//! - `T := u8`
//! - `T := u16`
//! - `T := [ T ; E ]`
//! - `T := & T` (pointer)
//! - `T := struct { F }`
//! - `T := union { F }`
//!
//! # Expressions
//! - Expression grammars are **lisp-based by default**. This will change in
//!   favor of non-lisp ones.
//!
//! # Remarks
//! - Note that the [`Pointer`] type syntax differs compared to Rust, C, or C++.
//! - Likewise, the [`AddressOf`] operator is also different.
//!
//! [`Pointer`]: ./struct.Pointer.html
//! [`AddressOf`]: ./expressions/struct.AddressOf.html
//!
//! # Example
//!
//! ```
//! # use ggbc_parser::Ast;
//! # ggbc_parser::parse::<Ast>(r#"
//! mod std {
//!    // adds a layer of typing to an existing region of memory
//!    // here, VRAM starts at address 0x8000 ans is layed out like this:
//!    static@0x0000 MEM_MAP :: [u8; 0x10000];
//!    static@0x8000 VRAM :: struct {
//!        tile_data :: union {
//!            x8000 :: struct {                        data::[u8; 0x1000] },
//!            x8800 :: struct { _padding::[u8; 0x800], data::[u8; 0x1000] }
//!        },
//!        tile_map :: struct { x9800::[u8; 0x400],
//!                             x9c00::[i8; 0x400] }
//!    };
//! }
//!
//! // C-style for loop
//! for offset::u8 in 0..+16 {
//!     // equivalent statements:
//!     (= ([] std::MEM_MAP (+ 0x8000 offset)) 0xff);
//!     //(= ([] std::VRAM::tile_data::x8000 offset) 0xff);
//! }
//!
//! loop {}
//! # "#).unwrap();
//! ```
use crate::{
    error::Error,
    lex,
    lex::{Token, Tokens},
};
use std::iter::Peekable;

// re-exports
use crate::span::{union, Span, Spanned};
pub use context::{Context, ContextBuilder};
#[cfg(feature = "lisp")]
pub use expressions::lisp::Expression;
#[cfg(not(feature = "lisp"))]
pub use expressions::Expression;

#[macro_use]
mod macros;
mod context;
// FIXME
//  group all the "workaround" grammars into this module. Eventually it will be
//  removed once I get rid of the type system errors...
mod workarounds;

pub mod asm;
pub mod expressions;

parse! {
    /// `(<statement>)* EOF`
    pub struct Ast<'a> {
        pub inner: Vec<Statement<'a>>,
        pub eof: lex::Eof<'a>,
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

// User-facing parsing functions

pub fn parse<'a, P: Grammar<'a>>(input: &'a str) -> Result<P, Error<'a>> {
    let mut context = ContextBuilder::default().build();
    parse_with_context(input, &mut context)
}

pub fn parse_with_context<'a, P: Grammar<'a>>(
    input: &'a str,
    context: &mut Context<'a>,
) -> Result<P, Error<'a>> {
    let mut tokens = Tokens::new(input).peekable();
    P::parse(context, &mut tokens)
}

// Grammar definitions and some blanket implementations.

pub trait Grammar<'a>: Sized {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>>;
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
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        P::parse(context, tokens).map(Box::new)
    }
}

// Parsing of boxed types for self-referential types.
// Mostly used within expression syntax trees.
impl<'a, P> Grammar<'a> for Vec<P>
where
    Option<P>: Grammar<'a>,
{
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let mut vec = Vec::new();
        while let Some(item) = Grammar::parse(context, tokens)? {
            vec.push(item);
        }
        Ok(vec)
    }
}

parse_tuple!(A, B);
parse_tuple!(A, B, C);
parse_tuple!(A, B, C, D);
parse_tuple!(A, B, C, D, E);
parse_tuple!(A, B, C, D, E, F);
parse_tuple!(A, B, C, D, E, F, G);
parse_tuple!(A, B, C, D, E, F, G, H);
parse_tuple!(A, B, C, D, E, F, G, H, I);
parse_tuple!(A, B, C, D, E, F, G, H, I, J);
parse_tuple!(A, B, C, D, E, F, G, H, I, J, K);
parse_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
parse_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
parse_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
parse_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
parse_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
parse_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);

#[derive(Debug)]
pub struct Separated<T, S> {
    pub head: T,
    pub tail: Vec<(S, T)>,
}

impl<T: Spanned, S> Spanned for Separated<T, S> {
    fn span(&self) -> Span {
        let mut span = self.head.span();
        if let Some((_, t)) = self.tail.last() {
            span = union(&span, &t.span());
        }
        span
    }
}

impl<T, S> Separated<T, S> {
    /// Returns an ordered iterator over the separated items of type `T`.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let tail_iter = self.tail.iter().map(|(_, ref t)| t);
        Some(&self.head).into_iter().chain(tail_iter)
    }

    /// Returns the total number of items of type `T`.
    pub fn len(&self) -> usize {
        self.tail.len() + 1
    }
}

impl<'a, T, S> Grammar<'a> for Separated<T, S>
where
    T: Grammar<'a>,
    S: Grammar<'a>,
    Option<S>: Grammar<'a>,
{
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
    pub enum Type<'a> {
        U8(lex::U8<'a>),
        I8(lex::I8<'a>),
        Array(Array<'a, Box<Type<'a>>, Expression<'a>>),
        Struct(Box<Struct<'a, ()>>),
        Union(Box<Union<'a, ()>>),
        Pointer(Pointer<'a, Box<Type<'a>>>),
        Ident(lex::Ident<'a>),
        // TODO function type
    }
}

parse! {
    /// `& <type>`
    pub struct Pointer<'a, T>
    where
        T: Grammar<'a>,
    {
        pub ampersand: lex::Ampersand<'a>,
        pub type_: T,
    }
}

impl<T: Spanned> Spanned for Pointer<'_, T> {
    fn span(&self) -> Span {
        union(&self.ampersand.span(), &self.type_.span())
    }
}

parse! {
    /// `[ <type> ; <length> ]`
    pub struct Array<'a, T, E>
    where
        T: Grammar<'a>,
        E: Grammar<'a>,
    {
        pub left_square: lex::LeftSquare<'a>,
        pub type_: T,
        pub semi_colon: lex::SemiColon<'a>,
        pub len: E,
        pub right_square: lex::RightSquare<'a>,
    }
}

impl<T, E> Spanned for Array<'_, T, E> {
    fn span(&self) -> Span {
        union(&self.left_square.span(), &self.right_square.span())
    }
}

impl<'a> Grammar<'a> for Option<Type<'a>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            Some(Err(_)) => {
                let err = tokens.next().unwrap().err().unwrap();
                Err(err)
            }
            Some(Ok(Token::U8(_))) => Ok(Some(Type::U8(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::I8(_))) => Ok(Some(Type::I8(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::LeftSquare(_))) => {
                Ok(Some(Type::Array(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Struct(_))) => Ok(Some(Type::Struct(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Union(_))) => Ok(Some(Type::Union(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Ampersand(_))) => {
                Ok(Some(Type::Pointer(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Ident(_))) => Ok(Some(Type::Ident(Grammar::parse(context, tokens)?))),
            _ => Ok(None),
        }
    }
}

impl<'a> Grammar<'a> for Type<'a> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken {
                token,
                expected: None,
            })
        }
    }
}

parse_enum! {
    pub enum Statement<'a> {
        If(If<'a, Expression<'a>, Vec<Statement<'a>>>),
        IfElse(IfElse<'a, Expression<'a>, Vec<Statement<'a>>, Vec<Statement<'a>>>),
        Scope(Scope<'a, Vec<Statement<'a>>>),
        Panic(Panic<'a>),
        Mod(Mod<'a, Vec<Statement<'a>>>),
        Struct(Struct<'a, lex::Ident<'a>>),
        Union(Union<'a, lex::Ident<'a>>),
        Asm(Asm<'a, Vec<asm::Asm<'a>>>),
        Static(Static<'a, Type<'a>>),
        Const(Const<'a, Type<'a>, Expression<'a>>),
        For(For<'a, Type<'a>, Expression<'a>, Expression<'a>, Vec<Statement<'a>>>),
        Loop(Loop<'a, Vec<Statement<'a>>>),
        Let(Let<'a, Type<'a>, Expression<'a>>),
        Fn(Fn<'a, Vec<Statement<'a>>>),
        Inline(Inline<'a, Expression<'a>>),
        Continue(Continue<'a>),
        Break(Break<'a>),
    }
}

impl<'a> Grammar<'a> for Option<Statement<'a>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
            Some(Ok(Token::RightBracket(_))) | Some(Ok(Token::Eof(_))) => Ok(None),
            Some(Ok(_)) => Ok(Some(Statement::Inline(Grammar::parse(context, tokens)?))),
            // Token error
            Some(Err(_)) => {
                let err = tokens.next().unwrap().err().unwrap();
                Err(err)
            }
        }
    }
}

impl<'a> Grammar<'a> for Statement<'a> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken {
                token,
                expected: None,
            })
        }
    }
}

/// `{ <inner> }`
pub struct Scope<'a, I> {
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: I,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<I> Spanned for Scope<'_, I> {
    fn span(&self) -> Span {
        union(&self.left_bracket.span(), &self.right_bracket.span())
    }
}

impl<'a, I: Grammar<'a>> Grammar<'a> for Scope<'a, I> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let left_bracket = Grammar::parse(context, tokens)?;
        context.scope_begin()?;
        let inner = Grammar::parse(context, tokens)?;
        context.scope_end()?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self {
            left_bracket,
            inner,
            right_bracket,
        })
    }
}

parse! {
    /// `!!`
    pub struct Panic<'a> {
        pub bang_bang: lex::BangBang<'a>,
    }
}

impl Spanned for Panic<'_> {
    fn span(&self) -> Span {
        self.bang_bang.span()
    }
}

parse! {
    /// `asm { (<asm>)* }`
    pub struct Asm<'a, I>
    where
        I: Grammar<'a>,
    {
        pub asm: lex::Asm<'a>,
        pub left_bracker: lex::LeftBracket<'a>,
        pub inner: I,
        pub right_bracker: lex::RightBracket<'a>,
    }
}

impl<I> Spanned for Asm<'_, I> {
    fn span(&self) -> Span {
        union(&self.asm.span(), &self.right_bracker.span())
    }
}

/// `mod <ident> { <statements> }`
pub struct Mod<'a, I> {
    pub mod_: lex::Mod<'a>,
    pub ident: lex::Ident<'a>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: I,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<I> Spanned for Mod<'_, I> {
    fn span(&self) -> Span {
        union(&self.mod_.span(), &self.right_bracket.span())
    }
}

impl<'a, I: Grammar<'a>> Grammar<'a> for Mod<'a, I> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let mod_ = Grammar::parse(context, tokens)?;
        let ident: lex::Ident<'a> = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        context.mod_begin(ident.clone())?;
        let inner = Grammar::parse(context, tokens)?;
        context.mod_end()?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self {
            mod_,
            ident,
            left_bracket,
            inner,
            right_bracket,
        })
    }
}

parse! {
    /// `<expr> ;`
    pub struct Inline<'a, E>
    where
        E: Grammar<'a>,
    {
        pub inner: E,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

impl<E: Spanned> Spanned for Inline<'_, E> {
    fn span(&self) -> Span {
        union(&self.inner.span(), &self.semi_colon.span())
    }
}

parse! {
    /// `@ <expr>`
    pub struct StaticOffset<'a> {
        pub at: lex::At<'a>,
        pub lit: lex::Lit<'a>,
    }
}

impl<'a> Grammar<'a> for Option<StaticOffset<'a>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::At(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

parse! {
    /// `static [<offset>] <field> ;`
    pub struct Static<'a, T>
    where
        T: Grammar<'a>,
    {
        pub static_: lex::Static<'a>,
        pub offset: Option<StaticOffset<'a>>,
        pub field: GlobalField<'a, T>,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

impl<T> Spanned for Static<'_, T> {
    fn span(&self) -> Span {
        union(&self.static_.span(), &self.semi_colon.span())
    }
}

parse! {
    /// `const <field> = <expr> ;`
    pub struct Const<'a, T, E>
    where
        T: Grammar<'a>,
        E: Grammar<'a>,
    {
        pub const_: lex::Const<'a>,
        pub field: GlobalField<'a, T>,
        pub assign: lex::Assign<'a>,
        pub expr: E,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

impl<T, E> Spanned for Const<'_, T, E> {
    fn span(&self) -> Span {
        union(&self.const_.span(), &self.semi_colon.span())
    }
}

/// `let <field> = <expr> ;`
pub struct Let<'a, T, E> {
    pub let_: lex::Let<'a>,
    pub field: Field<'a, T>,
    pub assign: lex::Assign<'a>,
    pub expr: E,
    pub semi_colon: lex::SemiColon<'a>,
}

impl<'a, T, E> Grammar<'a> for Let<'a, T, E>
where
    T: Grammar<'a>,
    E: Grammar<'a>,
{
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let let_: lex::Let<'a> = Grammar::parse(context, tokens)?;
        context.let_begin(let_.clone())?;
        let field = Grammar::parse(context, tokens)?;
        let assign = Grammar::parse(context, tokens)?;
        let expr = Grammar::parse(context, tokens)?;
        let semi_colon = Grammar::parse(context, tokens)?;
        context.let_end()?;
        Ok(Let {
            let_,
            field,
            assign,
            expr,
            semi_colon,
        })
    }
}

impl<T, E> Spanned for Let<'_, T, E> {
    fn span(&self) -> Span {
        union(&self.let_.span(), &self.semi_colon.span())
    }
}

parse! {
    /// `if <expr> { <inner> } else { <inner> }`
    pub struct IfElse<'a, E, I0, I1>
    where
        E: Grammar<'a>,
        I0: Grammar<'a>,
        I1: Grammar<'a>,
    {
        pub if_: If<'a, E, I0>,
        pub else_: Else<'a, I1>,
    }
}

impl<E, I0, I1> Spanned for IfElse<'_, E, I0, I1> {
    fn span(&self) -> Span {
        union(&self.if_.span(), &self.else_.span())
    }
}

parse! {
    /// `if <expr> { <inner> }`
    pub struct If<'a, E, I>
    where
        E: Grammar<'a>,
        I: Grammar<'a>,
    {
        pub if_: lex::If<'a>,
        pub expr: E,
        pub scope: Scope<'a, I>,
    }
}

impl<E, I> Spanned for If<'_, E, I> {
    fn span(&self) -> Span {
        union(&self.if_.span(), &self.scope.span())
    }
}

parse! {
    /// `else { <inner> }`
    pub struct Else<'a, I>
    where
        I: Grammar<'a>,
    {
        pub else_: lex::Else<'a>,
        pub scope: Scope<'a, I>,
    }
}

impl<I> Spanned for Else<'_, I> {
    fn span(&self) -> Span {
        union(&self.else_.span(), &self.scope.span())
    }
}

parse! {
    /// `<left> .. [=] [+] <right>`
    pub struct Range<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub dot_dot: lex::DotDot<'a>,
        pub eq: Option<lex::Assign<'a>>,
        pub plus: Option<lex::Plus<'a>>,
        pub right: R,
    }
}

/// `for <field> in <range> { (<statement>)* }`
pub struct For<'a, T, L, R, I> {
    pub for_: lex::For<'a>,
    pub field: Field<'a, T>,
    pub in_: lex::In<'a>,
    pub range: Range<'a, L, R>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: I,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<T, L, R, I> Spanned for For<'_, T, L, R, I> {
    fn span(&self) -> Span {
        union(&self.for_.span(), &self.right_bracket.span())
    }
}

impl<'a, T, L, R, I> Grammar<'a> for For<'a, T, L, R, I>
where
    T: Grammar<'a>,
    L: Grammar<'a>,
    R: Grammar<'a>,
    I: Grammar<'a>,
{
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        context.scope_begin()?;
        let for_ = Grammar::parse(context, tokens)?;
        let field = Grammar::parse(context, tokens)?;
        let in_ = Grammar::parse(context, tokens)?;
        let range = Grammar::parse(context, tokens)?;
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        let right_bracket = Grammar::parse(context, tokens)?;
        context.scope_end()?;
        Ok(Self {
            for_,
            field,
            in_,
            range,
            left_bracket,
            inner,
            right_bracket,
        })
    }
}

/// `loop { (<statement>)* }`
pub struct Loop<'a, I> {
    pub loop_: lex::Loop<'a>,
    pub inner: Scope<'a, I>,
}

impl<I> Spanned for Loop<'_, I> {
    fn span(&self) -> Span {
        union(&self.loop_.span(), &self.inner.span())
    }
}

impl<'a, I: Grammar<'a>> Grammar<'a> for Loop<'a, I> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        context.loop_begin()?;
        let loop_ = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        context.loop_end()?;
        Ok(Self { loop_, inner })
    }
}

/// `fn <ident> [<args>] <type> { }`
pub struct Fn<'a, I> {
    pub fn_: lex::Fn<'a>,
    pub ident: lex::Ident<'a>,
    pub fn_args: Option<FnArgs<'a>>,
    pub type_: Option<Type<'a>>,
    pub left_bracket: lex::LeftBracket<'a>,
    pub inner: I,
    pub right_bracket: lex::RightBracket<'a>,
}

impl<I> Spanned for Fn<'_, I> {
    fn span(&self) -> Span {
        union(&self.fn_.span(), &self.right_bracket.span())
    }
}

impl<'a, I: Grammar<'a>> Grammar<'a> for Fn<'a, I> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let fn_ = Grammar::parse(context, tokens)?;
        let ident: lex::Ident<'a> = Grammar::parse(context, tokens)?;
        context.type_begin_global(ident.clone())?;
        context.type_end_global()?;
        context.function_begin()?;
        let fn_args = Grammar::parse(context, tokens)?;
        let type_ = Grammar::parse(context, tokens)?;
        if let Some(type_) = &type_ {
            // TODO check scope of returned type
        }
        let left_bracket = Grammar::parse(context, tokens)?;
        let inner = Grammar::parse(context, tokens)?;
        context.function_end()?;
        let right_bracket = Grammar::parse(context, tokens)?;
        Ok(Self {
            fn_,
            ident,
            fn_args,
            type_,
            left_bracket,
            inner,
            right_bracket,
        })
    }
}

parse! {
    pub struct FnArgs<'a> {
        pub left_par: lex::LeftPar<'a>,
        pub args: Separated<Field<'a, Type<'a>>, lex::Comma<'a>>,
        pub right_par: lex::RightPar<'a>,
    }
}

impl Spanned for FnArgs<'_> {
    fn span(&self) -> Span {
        union(&self.left_par.span(), &self.right_par.span())
    }
}

impl<'a> Grammar<'a> for Option<FnArgs<'a>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::LeftPar(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

parse! {
    /// `struct [<ident>] { <fields> }`
    pub struct Struct<'a, I>
    where
        I: Grammar<'a>,
    {
        pub struct_: lex::Struct<'a>,
        pub ident: I,
        pub left_bracket: lex::LeftBracket<'a>,
        pub fields: Option<Separated<Field<'a, Type<'a>>, lex::Comma<'a>>>,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

impl<I> Spanned for Struct<'_, I> {
    fn span(&self) -> Span {
        union(&self.struct_.span(), &self.right_bracket.span())
    }
}

parse! {
    /// `union [<ident>] { <fields> }`
    pub struct Union<'a, I>
    where
        I: Grammar<'a>,
    {
        pub union: lex::Union<'a>,
        pub ident: I,
        pub left_bracket: lex::LeftBracket<'a>,
        pub fields: Option<Separated<Field<'a, Type<'a>>, lex::Comma<'a>>>,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

impl<I> Spanned for Union<'_, I> {
    fn span(&self) -> Span {
        union(&self.union.span(), &self.right_bracket.span())
    }
}

/// `<ident> :: <type>`
pub struct Field<'a, T> {
    pub ident: lex::Ident<'a>,
    pub square: lex::Square<'a>,
    pub type_: T,
}

impl<T: Spanned> Spanned for Field<'_, T> {
    fn span(&self) -> Span {
        union(&self.ident.span(), &self.type_.span())
    }
}

impl<'a, T: Grammar<'a>> Grammar<'a> for Field<'a, T> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let ident: lex::Ident<'a> = Grammar::parse(context, tokens)?;
        let square = Grammar::parse(context, tokens)?;
        context.type_begin_local(ident.clone())?;
        let type_ = Grammar::parse(context, tokens)?;
        context.type_end_local()?;
        Ok(Self {
            ident,
            square,
            type_,
        })
    }
}

/// `<ident> :: <type>`
pub struct GlobalField<'a, T> {
    pub ident: lex::Ident<'a>,
    pub square: lex::Square<'a>,
    pub type_: T,
}

impl<T: Spanned> Spanned for GlobalField<'_, T> {
    fn span(&self) -> Span {
        union(&self.ident.span(), &self.type_.span())
    }
}

// FIXME code repetition
impl<'a, T: Grammar<'a>> Grammar<'a> for GlobalField<'a, T> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let ident: lex::Ident<'a> = Grammar::parse(context, tokens)?;
        let square = Grammar::parse(context, tokens)?;
        context.type_begin_global(ident.clone())?;
        let type_ = Grammar::parse(context, tokens)?;
        context.type_end_global()?;
        Ok(Self {
            ident,
            square,
            type_,
        })
    }
}

parse! {
    /// `<ident> (, <ident>)* :: <type>`
    pub struct FieldGroup<'a, T>
    where
        T: Grammar<'a>,
    {
        pub ident: Separated<lex::Ident<'a>, lex::Comma<'a>>,
        pub square: lex::Square<'a>,
        pub type_: T,
    }
}

impl<T: Spanned> Spanned for FieldGroup<'_, T> {
    fn span(&self) -> Span {
        union(&self.ident.span(), &self.type_.span())
    }
}

impl<'a, T> Grammar<'a> for Option<FieldGroup<'a, T>>
where
    T: Grammar<'a>,
{
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
    pub semi_colon: lex::SemiColon<'a>,
}

impl Spanned for Continue<'_> {
    fn span(&self) -> Span {
        union(&self.continue_.span(), &self.semi_colon.span())
    }
}

impl<'a> Grammar<'a> for Continue<'a> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let statement = Self {
            continue_: Grammar::parse(context, tokens)?,
            semi_colon: Grammar::parse(context, tokens)?,
        };
        if !context.is_loop() {
            Err(Error::InvalidContinue(statement))
        } else {
            Ok(statement)
        }
    }
}

/// `break ;`
#[derive(Debug)]
pub struct Break<'a> {
    pub break_: lex::Break<'a>,
    pub semi_colon: lex::SemiColon<'a>,
}

impl Spanned for Break<'_> {
    fn span(&self) -> Span {
        union(&self.break_.span(), &self.semi_colon.span())
    }
}

impl<'a> Grammar<'a> for Break<'a> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let statement = Self {
            break_: Grammar::parse(context, tokens)?,
            semi_colon: Grammar::parse(context, tokens)?,
        };
        if !context.is_loop() {
            Err(Error::InvalidBreak(statement))
        } else {
            Ok(statement)
        }
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
        parse_program("struct Foo { a :: u8 }");
        parse_program("struct Foo { a :: u8, b :: u8 }");
        parse_program("struct Foo { a :: u8, b :: u8 , c :: struct { } }");
        parse_program("struct Foo { a :: u8, b :: u8 , c :: struct { a::[u8;42] } }");
    }

    #[test]
    fn union_() {
        parse_program("union Foo { }");
        parse_program("union Foo { a :: u8 }");
        parse_program("union Foo { a :: u8, b :: u8 }");
        parse_program("union Foo { a :: u8, b :: u8 , c :: union { } }");
        parse_program("union Foo { a :: u8, b :: u8 , c :: union { a::[u8;42] } }");
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
        parse_program("static FOO :: [u8; 0x800];");
        parse_program("static@0x8000 FOO :: [u8; 0x800];");
        parse_program("static@0x8000 FOO :: [u8; 0x800];");
    }

    #[test]
    #[should_panic]
    fn static_panic() {
        parse_program("static FOO :: u8 = 0;");
        parse_program("static FOO :: u8 = 0;");
    }

    #[test]
    fn const_() {
        parse_program("const FOO :: u8 = 42;");
        parse_program("const FOO :: u8 = 42;");
    }

    #[test]
    #[should_panic]
    fn const_panic() {
        parse_program("const FOO, BAR :: u8 = 0;");
    }

    #[test]
    fn for_() {
        parse_program("for i::u8 in 0.. 42 { }");
        parse_program("for i::u8 in 0..=42 { }");
        parse_program("for i::u8 in 0..=+42 { }");
    }

    #[test]
    fn loop_() {
        parse_program("loop {}");
        parse_program("loop {} loop {}");
        parse_program("loop {loop{}} loop {}");
    }

    #[test]
    fn let_() {
        parse_program("let foo::u8 = 42;");
        parse_program("let foo_bar::u8 = 0xffff;");
    }

    #[test]
    #[should_panic]
    fn let_panic() {
        parse_program("let foo = 42;");
    }

    #[test]
    fn fn_() {
        parse_program("fn foo { }");
        parse_program("fn foo u8 { }");
        parse_program("fn foo(bar::u8) { }");
        parse_program("fn foo(bar::u8) u8 { }");
        parse_program("fn foo(bar::u8, baz::u8) u8 { }");
    }

    #[test]
    #[ignore]
    fn inline_non_lisp() {
        parse_program("1;");
        parse_program("foo;");
        parse_program("foo();");
        parse_program("foo::bar(0, 1, 2);");
        parse_program("foo = 42;");
        parse_program("foo += 42;");
    }

    #[test]
    fn local_scope() {
        parse_program(
            r#"
            static FOO::u8;
            let bar::u8 = 0;
            loop {
                let baz::u8 = 0;
                {
                    FOO;
                    bar;
                    baz;
                }
            }
            FOO;
            bar;
            fn fun {
                FOO;
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
                    mod c {}
                    c;
                    mod d {}
                    d;
                }
                b::c;
                b::d;
            }
            a::b::c;
            a::b::d;
        "#,
        );
    }
}
