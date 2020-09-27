use crate::{lex, lex::Tokens};
use std::iter::Peekable;

#[macro_use]
mod macros;
pub mod asm;
mod error;
pub mod expressions;

use crate::{
    ast::expressions::{Expression, ParseExpression},
    lex::Token,
};
pub use error::Error;
use std::{borrow::Cow, collections::HashSet};

pub type Program<'a> = Ast<'a, Vec<Statement<'a>>>;

// base trait for parse-able types.
pub trait Parse<'a>: Sized {
    /// Parse statement.
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error>;
}
/// Marker trait.
pub trait ParseFields<'a>: Parse<'a> {}
/// Marker trait.
pub trait ParseType<'a>: Parse<'a> {}
/// Marker trait.
pub trait ParseStatement<'a>: Parse<'a> {}

impl<'a, T: ParseType<'a>> ParseFields<'a> for Field<'a, T> {}
impl<'a> ParseFields<'a> for Vec<Field<'a, Type<'a>>> {}
impl<'a> ParseFields<'a> for () {}

impl<'a> ParseType<'a> for Type<'a> {}
impl<'a> ParseType<'a> for lex::U8<'a> {}
impl<'a> ParseType<'a> for lex::I8<'a> {}
impl<'a> ParseType<'a> for lex::U16<'a> {}
impl<'a> ParseType<'a> for lex::I16<'a> {}
impl<'a, T: ParseType<'a>, E: Parse<'a>> ParseType<'a> for ArrayType<'a, T, E> {}
impl<'a, F: ParseFields<'a>> ParseType<'a> for Struct<'a, (), F> {}
impl<'a, F: ParseFields<'a>> ParseType<'a> for Union<'a, (), F> {}

impl<'a> ParseStatement<'a> for Vec<Statement<'a>> {}
impl<'a> ParseStatement<'a> for Option<Statement<'a>> {}
impl<'a> ParseStatement<'a> for Statement<'a> {}
impl<'a> ParseStatement<'a> for Use<'a> {}
impl<'a, E: ParseStatement<'a>> ParseStatement<'a> for Mod<'a, E> {}
impl<'a, F: ParseFields<'a>> ParseStatement<'a> for Struct<'a, lex::Ident<'a>, F> {}
impl<'a, L: ParseExpression<'a>, R: ParseExpression<'a>> ParseStatement<'a> for Assign<'a, L, R> {}
impl<'a, E: ParseExpression<'a>> ParseStatement<'a> for InlineExpression<'a, E> {}
impl<'a, T: ParseType<'a>> ParseStatement<'a> for Static<'a, T> {}
impl<'a, T: ParseType<'a>, E: ParseExpression<'a>> ParseStatement<'a> for Const<'a, T, E> {}

// For parsing no-arguments (performs a no-op).
// Mainly used on the Struct statement, which behaves like a statement when it
// has an identifier, and like a type when it doesn't have.
impl<'a> Parse<'a> for () {
    fn parse(_: &mut Context, _: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        Ok(())
    }
}

// Parsing of boxed types for self-referential types.
// Mostly used within expression syntax trees.
impl<'a, P: Parse<'a>> Parse<'a> for Box<P> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        P::parse(context, tokens).map(Box::new)
    }
}

pub enum Statement<'a> {
    Use(Use<'a>),
    Mod(Mod<'a, Vec<Statement<'a>>>),
    Struct(Struct<'a, lex::Ident<'a>, Vec<Field<'a, Type<'a>>>>),
    Union(Union<'a, lex::Ident<'a>, Vec<Field<'a, Type<'a>>>>),
    InlineAsm(InlineAsm<'a>),
    Static(Static<'a, Type<'a>>),
    Const(Const<'a, Type<'a>, Expression<'a>>),
}

pub enum Type<'a> {
    I16(lex::I16<'a>),
    U16(lex::U16<'a>),
    I8(lex::I8<'a>),
    U8(lex::U8<'a>),
    ArrayType(ArrayType<'a, Box<Type<'a>>, Expression<'a>>),
    Struct(Struct<'a, (), Vec<Field<'a, Type<'a>>>>),
    Union(Union<'a, (), Vec<Field<'a, Type<'a>>>>),
    Ident(lex::Ident<'a>),
}

impl<'a> Parse<'a> for Option<Statement<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        match tokens.peek() {
            None => Ok(None),
            Some(Ok(Token::Use(_))) => Ok(Some(Statement::Use(Parse::parse(context, tokens)?))),
            Some(Ok(Token::Mod(_))) => Ok(Some(Statement::Mod(Parse::parse(context, tokens)?))),
            Some(Ok(Token::Struct(_))) => {
                Ok(Some(Statement::Struct(Parse::parse(context, tokens)?)))
            }
            Some(Ok(Token::Asm(_))) => {
                Ok(Some(Statement::InlineAsm(Parse::parse(context, tokens)?)))
            }
            Some(Ok(Token::Static(_))) => {
                Ok(Some(Statement::Static(Parse::parse(context, tokens)?)))
            }
            Some(Ok(Token::Const(_))) => Ok(Some(Statement::Const(Parse::parse(context, tokens)?))),
            Some(Ok(_)) => Ok(None),
            // Token error
            Some(Err(_)) => {
                let err = tokens.next().unwrap().err().unwrap();
                Err(Error::Lexer(err))
            }
        }
    }
}

impl<'a> Parse<'a> for Vec<Statement<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        let mut vec = Vec::new();
        while let Some(s) = Parse::parse(context, tokens)? {
            vec.push(s);
        }
        Ok(vec)
    }
}

impl<'a> Parse<'a> for Statement<'a> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        if let Some(statement) = Parse::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let _token = tokens.next().expect("Token please");
            Err(Error::UnexpectedToken)
        }
    }
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        match tokens.peek() {
            Some(Err(_)) => {
                let err = tokens.next().unwrap().err().unwrap();
                Err(Error::Lexer(err))
            }
            Some(Ok(Token::I16(_))) => Ok(Type::I16(Parse::parse(context, tokens)?)),
            Some(Ok(Token::U16(_))) => Ok(Type::U16(Parse::parse(context, tokens)?)),
            Some(Ok(Token::I8(_))) => Ok(Type::I8(Parse::parse(context, tokens)?)),
            Some(Ok(Token::U8(_))) => Ok(Type::U8(Parse::parse(context, tokens)?)),
            Some(Ok(Token::LeftSquare(_))) => Ok(Type::ArrayType(Parse::parse(context, tokens)?)),
            Some(Ok(Token::Struct(_))) => Ok(Type::Struct(Parse::parse(context, tokens)?)),
            Some(Ok(Token::Union(_))) => Ok(Type::Union(Parse::parse(context, tokens)?)),
            Some(Ok(Token::Ident(_))) => Ok(Type::Ident(Parse::parse(context, tokens)?)),
            Some(Ok(_)) => {
                // TODO error reporting
                let _ = tokens.next().unwrap();
                Err(Error::UnexpectedToken)
            }
            None => unimplemented!(),
        }
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

#[derive(Debug, Eq, PartialEq)]
pub struct Context<'a, 'b> {
    level: usize,
    parent: Option<&'a Self>,
    ident: HashSet<Cow<'b, str>>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new() -> Self {
        Self {
            level: 0,
            parent: None,
            ident: HashSet::new(),
        }
    }

    /// Nested level.
    pub fn level(&self) -> usize {
        self.level
    }

    /// Inserts identifier to the current scope.
    pub fn define<S: Into<Cow<'b, str>>>(&mut self, ident: S) {
        self.ident.insert(ident.into());
    }

    /// Remove identifier from current scope.
    /// Returns `true` if the item exists in the current scope.
    pub fn undefine<S: Into<Cow<'b, str>>>(&mut self, ident: S) -> bool {
        self.ident.remove(&ident.into())
    }

    /// Check if the current scope contains the given scope.
    pub fn contains<S: Into<Cow<'b, str>>>(&self, ident: S) -> bool {
        let ident = ident.into();
        let mut defined = self.ident.contains(&ident);
        if let (false, Some(parent)) = (defined, self.parent) {
            defined = parent.contains(ident);
        }
        defined
    }

    /// Create new scope.
    pub fn push(&'a self) -> Self {
        Self {
            level: self.level + 1,
            parent: Some(self),
            ident: HashSet::new(),
        }
    }

    /// Pop current scope.
    ///
    /// # Panics
    /// Panics if pop is called on the root scope.
    pub fn pop(self) {
        assert_ne!(0, self.level);
    }
}

impl Drop for Context<'_, '_> {
    fn drop(&mut self) {
        if self.level != 0 {
            panic!("You forgot to call pop on the scope");
        }
    }
}

parse! {
    /// `asm { (<asm>)* }`
    pub struct InlineAsm<'a> {
        pub asm: lex::Asm<'a>,
        pub left_bracker: lex::LeftBracket<'a>,
        pub right_bracker: lex::RightBracket<'a>,
    }
}

parse! {
    /// `(<statement>)* EOF`
    pub struct Ast<'a, T> {
        pub inner: T,
        pub eof: lex::Eof<'a>,
    }
}

parse! {
    /// `use <path> ;`
    pub struct Use<'a> {
        pub use_: lex::Use<'a>,
        pub path: Path<'a>,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse_vec_separated!(lex::Square<'a>, lex::Ident<'a>);

parse! {
    /// `<head> (:: <tail>)*`
    pub struct Path<'a> {
        pub head: lex::Ident<'a>,
        pub tail: Vec<(lex::Square<'a>, lex::Ident<'a>)>,
    }
}

impl<'a> Parse<'a> for Option<Path<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Parse::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

parse! {
    /// `mod <ident> { <statements> }`
    pub struct Mod<'a, I> {
        pub mod_: lex::Mod<'a>,
        pub ident: lex::Ident<'a>,
        pub left_bracket: lex::LeftBracket<'a>,
        pub inner: I,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `[ <type> ; <length> ]`
    pub struct ArrayType<'a, T, E> {
        pub left_square: lex::LeftSquare<'a>,
        pub type_: T,
        pub semi_coloon: lex::SemiColon<'a>,
        pub len: E,
        pub right_square: lex::RightSquare<'a>,
    }
}

parse! {
    /// `struct [<ident>] { <fields> }`
    pub struct Struct<'a, I, F> {
        pub struct_: lex::Struct<'a>,
        pub ident: I,
        pub left_bracket: lex::LeftBracket<'a>,
        pub fields: F,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `union [<ident>] { <fields> }`
    pub struct Union<'a, I, F> {
        pub union: lex::Union<'a>,
        pub ident: I,
        pub left_bracket: lex::LeftBracket<'a>,
        pub fields: F,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `[mut] <ident> :: <type> ,`
    pub struct Field<'a, T> {
        pub mut_: Option<lex::Mut<'a>>,
        pub ident: lex::Ident<'a>,
        pub square: lex::Square<'a>,
        pub type_: T,
        pub comma: lex::Comma<'a>,
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<Field<'a, T>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        match tokens.peek() {
            Some(Ok(Token::Mut(_))) | Some(Ok(Token::Ident(_))) => {
                Ok(Some(Parse::parse(context, tokens)?))
            }
            _ => Ok(None),
        }
    }
}

// TODO code repetition
impl<'a> Parse<'a> for Vec<Field<'a, Type<'a>>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        let mut vec = Vec::new();
        while let Some(s) = Parse::parse(context, tokens)? {
            vec.push(s);
        }
        Ok(vec)
    }
}

parse! {
    /// `<left> .. [=] <right>`
    pub struct Range<'a, L, R> {
        pub left: L,
        pub dot_dot: lex::DotDot<'a>,
        pub eq: Option<lex::Assign<'a>>,
        pub right: R,
    }
}

parse! {
    /// `<left> = <right> ;`
    pub struct Assign<'a, L, R> {
        pub left: L,
        pub assign: lex::Assign<'a>,
        pub right: R,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse! {
    /// `<expr> ;`
    pub struct InlineExpression<'a, E> {
        pub inner: E,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse! {
    /// `@ <expr>`
    pub struct StaticOffset<'a> {
        pub at: lex::At<'a>,
        pub lit: lex::Lit<'a>,
    }
}

impl<'a> Parse<'a> for Option<StaticOffset<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        if let Some(Ok(Token::At(_))) = tokens.peek() {
            Ok(Some(Parse::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

parse! {
    /// `static [@<lit>] <ident> :: <type> ;`
    pub struct Static<'a, T> {
        pub static_: lex::Static<'a>,
        pub offset: Option<StaticOffset<'a>>,
        pub ident: lex::Ident<'a>,
        pub square: lex::Square<'a>,
        pub type_: T,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse! {
    /// `const <ident> :: <type> = <expr> ;`
    pub struct Const<'a, T, E> {
        pub const_: lex::Const<'a>,
        pub ident: lex::Ident<'a>,
        pub square: lex::Square<'a>,
        pub type_: T,
        pub assign: lex::Assign<'a>,
        pub expr: E,
        pub semi_colon: lex::SemiColon<'a>,
    }
}
