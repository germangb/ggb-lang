use crate::{lex, lex::Tokens};
use std::iter::Peekable;

#[macro_use]
mod macros;
mod error;

use crate::lex::Token;
pub use error::Error;
use std::{borrow::Cow, collections::HashSet};

pub type Program<'a> = Ast<'a, Vec<Statement<'a>>>;

// base trait for parse-able types.
pub trait Parse<'a>: Sized {
    /// Parse statement.
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error>;
}
// marker traits
pub trait FieldParse<'a>: Parse<'a> {}
pub trait TypeParse<'a>: Parse<'a> {}
pub trait StatementParse<'a>: Parse<'a> {}
pub trait ExpressionParse<'a>: Parse<'a> {}

impl<'a, T: TypeParse<'a>> FieldParse<'a> for Field<'a, T> {}
impl<'a> FieldParse<'a> for Vec<Field<'a, Type<'a>>> {}
impl<'a> FieldParse<'a> for () {}

impl<'a> TypeParse<'a> for Type<'a> {}
impl<'a> TypeParse<'a> for lex::U8<'a> {}
impl<'a> TypeParse<'a> for lex::I8<'a> {}
impl<'a> TypeParse<'a> for lex::U16<'a> {}
impl<'a> TypeParse<'a> for lex::I16<'a> {}
impl<'a, T: TypeParse<'a>, E: Parse<'a>> TypeParse<'a> for ArrayType<'a, T, E> {}
impl<'a, F: FieldParse<'a>> TypeParse<'a> for Struct<'a, (), F> {}

impl<'a> ExpressionParse<'a> for lex::Ident<'a> {}
impl<'a> ExpressionParse<'a> for lex::Lit<'a> {}
impl<'a, E: ExpressionParse<'a>> ExpressionParse<'a> for Par<'a, E> {}
impl<'a, L: ExpressionParse<'a>, R: ExpressionParse<'a>> ExpressionParse<'a> for Add<'a, L, R> {}
impl<'a, L: ExpressionParse<'a>, R: ExpressionParse<'a>> ExpressionParse<'a> for Sub<'a, L, R> {}
impl<'a, L: ExpressionParse<'a>, R: ExpressionParse<'a>> ExpressionParse<'a> for Mul<'a, L, R> {}
impl<'a, L: ExpressionParse<'a>, R: ExpressionParse<'a>> ExpressionParse<'a> for Div<'a, L, R> {}
impl<'a, E: ExpressionParse<'a>> ExpressionParse<'a> for Plus<'a, E> {}
impl<'a, E: ExpressionParse<'a>> ExpressionParse<'a> for Minus<'a, E> {}

impl<'a> StatementParse<'a> for Vec<Statement<'a>> {}
impl<'a> StatementParse<'a> for Option<Statement<'a>> {}
impl<'a> StatementParse<'a> for Use<'a> {}
impl<'a, E: StatementParse<'a>> StatementParse<'a> for Mod<'a, E> {}
impl<'a, F: FieldParse<'a>> StatementParse<'a> for Struct<'a, lex::Ident<'a>, F> {}
impl<'a, L: ExpressionParse<'a>, R: ExpressionParse<'a>, F: FieldParse<'a>> StatementParse<'a>
    for MemoryMap<'a, L, R, F>
{
}

// For parsing no-arguments (performs a no-op).
// Mainly used on the Struct statement, which behaves like a statement when it
// has an identifier, and like a type when it doesn't have.
impl<'a> Parse<'a> for () {
    fn parse(_: &mut Context, _: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        Ok(())
    }
}

pub enum Statement<'a> {
    Use(Use<'a>),
    Mod(Mod<'a, Vec<Statement<'a>>>),
    Struct(Struct<'a, lex::Ident<'a>, Vec<Field<'a, Type<'a>>>>),
    MemoryMap(MemoryMap<'a, (), (), Vec<Field<'a, Type<'a>>>>),
}

pub enum Type<'a> {
    I16(lex::I16<'a>),
    U16(lex::U16<'a>),
    I8(lex::I8<'a>),
    U8(lex::U8<'a>),
    ArrayType(ArrayType<'a, Box<Type<'a>>, lex::Lit<'a>>),
    Struct(Struct<'a, (), Vec<Field<'a, Type<'a>>>>),
    Ident(lex::Ident<'a>),
}

pub enum Expression<'a> {
    Ident(lex::Ident<'a>),
    Lit(lex::Lit<'a>),
    Par(Par<'a, Box<Expression<'a>>>),
    Add(Add<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Sub(Sub<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Mul(Mul<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Div(Div<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Plus(Plus<'a, Box<Expression<'a>>>),
    Minus(Minus<'a, Box<Expression<'a>>>),
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
            Some(Ok(Token::Ident(_))) => {
                Ok(Some(Statement::MemoryMap(Parse::parse(context, tokens)?)))
            }
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

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        unimplemented!()
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
            Some(Ok(Token::Ident(_))) => Ok(Type::Ident(Parse::parse(context, tokens)?)),
            Some(Ok(_)) => {
                let _ = tokens.next().unwrap();
                Err(Error::UnexpectedToken)
            }
            None => unimplemented!(),
        }
    }
}

impl<'a> Parse<'a> for Box<Type<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        Ok(Box::new(Type::parse(context, tokens)?))
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
    /// `(<statement>)* EOF`
    pub struct Ast<'a, T> {
        pub inner: T,
        pub eof: lex::Eof<'a>,
    }
}

parse! {
    /// `use <head> (:: <tail>)* ;`
    pub struct Use<'a> {
        pub use_: lex::Use<'a>,
        pub head: lex::Ident<'a>,
        pub tail: Vec<(lex::Square<'a>, lex::Ident<'a>)>,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse_vec_separated!(lex::Square<'a>, lex::Ident<'a>);

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
    /// `<ident> in [ [left] .. [right] ] { <fields> }`
    pub struct MemoryMap<'a, L, R, F> {
        pub ident: lex::Ident<'a>,
        pub in_: lex::In<'a>,
        pub left_square: lex::LeftSquare<'a>,
        pub left: L,
        pub dot_dot: lex::DotDot<'a>,
        pub right: R,
        pub right_square: lex::RightSquare<'a>,
        pub left_bracket: lex::LeftBracket<'a>,
        pub fields: F,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `( <expr> )`
    pub struct Par<'a, E> {
        pub left_par: lex::LeftPar<'a>,
        pub inner: E,
        pub right_par: lex::RightPar<'a>,
    }
}

parse! {
    /// `<expr> + <expr>`
    pub struct Add<'a, L, R> {
        pub left: L,
        pub plus: lex::Plus<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> - <expr>`
    pub struct Sub<'a, L, R> {
        pub left: L,
        pub plus: lex::Minus<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> * <expr>`
    pub struct Mul<'a, L, R> {
        pub left: L,
        pub plus: lex::Star<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> / <expr>`
    pub struct Div<'a, L, R> {
        pub left: L,
        pub plus: lex::Div<'a>,
        pub right: R,
    }
}

parse! {
    /// `+ <expr>`
    pub struct Plus<'a, E> {
        pub plus: lex::Plus<'a>,
        pub expr: E,
    }
}

parse! {
    /// `- <expr>`
    pub struct Minus<'a, E> {
        pub minus: lex::Minus<'a>,
        pub expr: E,
    }
}
