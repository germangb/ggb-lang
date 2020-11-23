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
        span,
        span::{Span, Spanned},
        Token, Tokens,
    },
};
#[doc(inline)]
#[cfg(todo_asm)]
pub use asm::Asm;
#[cfg(todo_asm)]
pub use asm::Opcode;
pub use context::{Context, ContextBuilder};
pub use expression::Expression;
pub use path::Path;
pub use r#static::*;
use std::iter::Peekable;
pub use types::Type;

#[macro_use]
mod macros;
#[cfg(todo_asm)]
pub mod asm;
mod context;
pub mod expression;
mod path;
mod r#static;
pub mod types;

/// Parse input source code.
pub fn parse(input: &str) -> Result<Ast<'_>, Error<'_>> {
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

impl<'a> Grammar<'a> for () {
    fn parse(_: &mut Context<'a>, _: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        Ok(())
    }
}

impl<'a, P: Grammar<'a>> Grammar<'a> for Box<P> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        P::parse(context, tokens).map(Self::new)
    }
}

impl<'a, P> Grammar<'a> for Vec<P> where Option<P>: Grammar<'a>
{
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let mut vec = Self::new();
        while let Some(item) = Grammar::parse(context, tokens)? {
            vec.push(item);
        }
        Ok(vec)
    }
}

parse! {
    /// Program statements.
    #[derive(Debug)]
    pub enum Statement<'a> {
        /// If statement.
        If(If<'a>),

        // IfElse statement.
        IfElse(IfElse<'a>),

        /// Scope (aka block).
        Scope(Scope<'a>),

        /// Panic statement.
        Panic(Panic<'a>),

        /// Module definition statement.
        Mod(Mod<'a>),

        /* /// Inline assembly statement.
        Asm(Asm<'a>), */

        /// Static statement (static symbol definition).
        Static(Static<'a>),

        /// Static const statement (const symbol definition).
        Const(Const<'a>),

        /// Let statement (stack symbol definition).
        Let(Let<'a>),

        /// For loop statement.
        For(For<'a>),

        /// Loop statement.
        Loop(Loop<'a>),

        /// Continue statement (flow control).
        Continue(Continue<'a>),

        /// Break statement (flow control).
        Break(Break<'a>),

        /// Void expression.
        Inline(Inline<'a>),

        /// Function declaration statement.
        Fn(Fn<'a>),

        /// Return statement.
        Return(Return<'a>),
    }
}

impl<'a> Grammar<'a> for Option<Statement<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        let statement = match tokens.peek() {
            Some(Err(_)) => return Err(tokens.next().unwrap().err().unwrap()),

            None | Some(Ok(Token::RightBracket(_))) | Some(Ok(Token::Eof(_))) => return Ok(None),

            Some(Ok(Token::If(_))) => {
                let if_ = Grammar::parse(context, tokens)?;

                if let Some(Ok(Token::Else(_))) = tokens.peek() {
                    Statement::IfElse(IfElse { if_,
                                               else_: Grammar::parse(context, tokens)? })
                } else {
                    Statement::If(if_)
                }
            }
            Some(Ok(Token::LeftBracket(_))) => Statement::Scope(Grammar::parse(context, tokens)?),
            Some(Ok(Token::BangBang(_))) => Statement::Panic(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Mod(_))) => Statement::Mod(Grammar::parse(context, tokens)?),
            #[cfg(todo_asm)]
            Some(Ok(Token::Asm(_))) => Statement::Asm(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Static(_))) => Statement::Static(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Const(_))) => Statement::Const(Grammar::parse(context, tokens)?),
            Some(Ok(Token::For(_))) => Statement::For(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Loop(_))) => Statement::Loop(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Let(_))) => Statement::Let(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Fn(_))) => Statement::Fn(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Continue(_))) => Statement::Continue(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Break(_))) => Statement::Break(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Return(_))) => Statement::Return(Grammar::parse(context, tokens)?),
            Some(Ok(_)) => Statement::Inline(Grammar::parse(context, tokens)?),
        };

        Ok(Some(statement))
    }
}

impl<'a> Grammar<'a> for Statement<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken { token })
        }
    }
}

span!(If { if_, right_bracket });
span!(Else { else_,
             right_bracket });
span!(IfElse { if_, else_ });
span!(Scope { left_bracket,
              right_bracket });
span!(Panic { bang_bang });
span!(Mod { mod_,
            right_bracket });
span!(Const { const_, expression });
span!(Let { let_, expression });
span!(For { for_,
            right_bracket });
span!(Loop { loop_,
             right_bracket });
span!(Continue { continue_ });
span!(Break { break_ });
span!(Inline { inner });
span!(Fn { fn_, right_bracket });
span!(FnReturn { colon, type_ });
span!(FnArg { left_par,
              right_par });
span!(Return { return_ });
span!(Struct { struct_,
               right_bracket });
span!(Union { union,
              right_bracket });
span!(Field { ident, type_ });
span!(FieldGroup { head, type_ });

parse! {
    #[derive(Debug)]
    pub struct Ast<'a> {
        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// EOF token.
        pub eof: lex::Eof<'a>,
    }
}

impl Spanned for Ast<'_> {
    fn span(&self) -> Span {
        let mut span = self.eof.span();
        if let Some(s) = self.inner.first() {
            span = span::union(&span, &s.span());
        }
        span
    }
}

parse! {
    #[derive(Debug)]
    pub struct Scope<'a> {
        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Panic<'a> {
        /// `!!` token.
        pub bang_bang: lex::BangBang<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Mod<'a> {
        /// `mod` token.
        pub mod_: lex::Mod<'a>,

        /// Identifier token.
        pub ident: lex::Ident<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Inline<'a> {
        /// Inner expression.
        pub inner: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Const<'a> {
        /// `const` tokens.
        pub const_: lex::Const<'a>,

        /// [`Field`](Field) tokens.
        pub field: Field<'a>,

        /// `=` token.
        pub assign: lex::Assign<'a>,

        /// Const expression tokens.
        pub expression: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Let<'a> {
        /// `let` token.
        pub let_: lex::Let<'a>,

        /// [`Field`](Field) tokens.
        pub field: Field<'a>,

        /// `=` token.
        pub assign: lex::Assign<'a>,

        /// Expression tokens.
        pub expression: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct IfElse<'a> {
        /// If block tokens.
        pub if_: If<'a>,

        /// Else block tokens.
        pub else_: Else<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct If<'a> {
        /// `if` token.
        pub if_: lex::If<'a>,

        /// If expression tokens.
        pub expression: Expression<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Else<'a> {
        /// `else` token.
        pub else_: lex::Else<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Range<'a> {
        /// Left expression tokens.
        pub left: Expression<'a>,

        /// `..` token.
        pub dot_dot: lex::DotDot<'a>,

        /// Optional `=` token.
        pub eq: Option<lex::Assign<'a>>,

        /// Optional `+` token.
        pub plus: Option<lex::Plus<'a>>,

        /// Right expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct For<'a> {
        /// `for` token.
        pub for_: lex::For<'a>,

        /// For field tokens.
        pub field: Field<'a>,

        /// `in` token.
        pub in_: lex::In<'a>,

        /// Range token.
        pub range: Range<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Loop<'a> {
        /// `loop` token.
        pub loop_: lex::Loop<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct FnReturn<'a> {
        /// `:` token.
        pub colon: lex::Colon<'a>,

        /// Return type tokens.
        pub type_: Type<'a>,
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

parse! {
    #[derive(Debug)]
    pub struct Fn<'a> {
        /// `fn` token.
        pub fn_: lex::Fn<'a>,

        /// Function identifier token.
        pub ident: lex::Ident<'a>,

        /// Function argument tokens.
        pub fn_arg: Option<FnArg<'a>>,

        /// Function return tokens.
        pub fn_return: Option<FnReturn<'a>>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner statements.
        pub inner: Vec<Statement<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct FnArg<'a> {
        /// `(` token.
        pub left_par: lex::LeftPar<'a>,

        /// Inner field(s) token(s).
        pub inner: Vec<Field<'a>>,

        /// `}` token.
        pub right_par: lex::RightPar<'a>,
    }
}

impl<'a> Grammar<'a> for Option<FnArg<'a>> {
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

parse! {
    #[derive(Debug)]
    pub struct Struct<'a> {
        /// `struct` token.
        pub struct_: lex::Struct<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner fields.
        pub fields: Vec<Field<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Union<'a> {
        /// `union` token.
        pub union: lex::Union<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner fields.
        pub fields: Vec<Field<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Field<'a> {
        /// Field identifier.
        pub ident: lex::Ident<'a>,

        /// `:` token.
        pub colon: lex::Colon<'a>,

        /// Type tokens.
        pub type_: Type<'a>,
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

parse! {
    #[derive(Debug)]
    pub struct FieldGroup<'a> {
        /// First identifier.
        pub head: lex::Ident<'a>,

        /// Rest of the identifiers.
        pub tail: Vec<lex::Ident<'a>>,

        /// `:` token.
        pub colon: lex::Colon<'a>,

        /// Field type tokens.
        pub type_: Type<'a>,
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

parse! {
    #[derive(Debug)]
    pub struct Continue<'a> {
        /// `continue` token.
        pub continue_: lex::Continue<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Break<'a> {
        /// `break` token.
        pub break_: lex::Break<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Return<'a> {
        /// `return` token.
        pub return_: lex::Return<'a>,

        /// Expression tokens.
        pub expression: Option<Expression<'a>>,
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Ast;

    fn parse_program(input: &str) -> Ast<'_> {
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
    #[ignore]
    fn struct_() {
        parse_program("struct Foo { }");
        parse_program("struct Foo { a : u8 }");
        parse_program("struct Foo { a : u8 b : u8 }");
        parse_program("struct Foo { a : u8 b : u8 c : struct { } }");
        parse_program("struct Foo { a : u8 b : u8 c : struct { a:[u8;42] } }");
    }

    #[test]
    #[ignore]
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
