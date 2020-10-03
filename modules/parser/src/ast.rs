//! Grammar definitions and syntactic analysis.
//!
//! # Statements
//! - `S := !!` (panic)
//! - `S := use P ;`
//! - `S := mod <ident> { S }`
//! - `S := struct <ident> { F }`
//! - `S := union <ident> { F }`
//! - `S := asm { <instructions> }`
//! - `S := static [@ <lit>] <ident> :: T ;`
//! - `S := const <ident> :: T = E ;`
//! - `S := for <ident> :: T in E .. [=] [+] E { S }`
//! - `S := loop { S }`
//! - `S := let <ident> :: T = E ;`
//! - `S := fn <ident> [()] [T] { S }`
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
//! # Path
//! - `P := <ident>`
//! - `P := P :: <ident>`
//!
//! # Field group
//! - `F := <ident> :: T`
//! - `F := F , F`
//!
//! # Remarks
//! - Note that the [`Pointer`] type differ compared to Rust, C, or C++.
//! - Likewise, the [`AddressOf`] operator expression is also different.
//!
//! [`Pointer`]: ./struct.Pointer.html
//! [`AddressOf`]: ./expressions/struct.AddressOf.html
use crate::{lex, lex::Tokens};
use std::iter::Peekable;

#[macro_use]
mod macros;
pub mod asm;
pub mod expressions;

use crate::{
    ast::expressions::{Expression, ExpressionGrammar},
    error::Error,
    lex::Token,
};
use std::{borrow::Cow, collections::HashSet};

pub type Program<'a> = Ast<'a, Vec<Statement<'a>>>;

pub fn parse_program(input: &str) -> Result<Program, Error> {
    parse_grammar(input)
}

pub fn parse_program_with_context<'a>(
    input: &'a str,
    context: &mut Context,
) -> Result<Program<'a>, Error<'a>> {
    parse_grammar_with_context(input, context)
}

pub fn parse_grammar<'a, P: Grammar<'a>>(input: &'a str) -> Result<P, Error<'a>> {
    let mut context = Context::new();
    parse_grammar_with_context(input, &mut context)
}

pub fn parse_grammar_with_context<'a, P: Grammar<'a>>(
    input: &'a str,
    context: &mut Context,
) -> Result<P, Error<'a>> {
    let mut tokens = Tokens::new(input).peekable();
    P::parse(context, &mut tokens)
}

pub trait Grammar<'a>: Sized {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>>;
}

pub trait TypeGrammar<'a>: Grammar<'a> {}
pub trait StatementGrammar<'a>: Grammar<'a> {}

impl<'a> TypeGrammar<'a> for Type<'a> {}
impl<'a> TypeGrammar<'a> for lex::U8<'a> {}
impl<'a> TypeGrammar<'a> for lex::U16<'a> {}
impl<'a, T: TypeGrammar<'a>, E: ExpressionGrammar<'a>> TypeGrammar<'a> for Array<'a, T, E> {}
impl<'a> TypeGrammar<'a>
    for Struct<'a, (), Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>>>
{
}
impl<'a> TypeGrammar<'a>
    for Union<'a, (), Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>>>
{
}
impl<'a, T: TypeGrammar<'a>> TypeGrammar<'a> for Pointer<'a, T> {}

// FIXME
//  fuck this, I mean fix it.
//  Generics are fucked somewhere in this typed mess...
//  I'd suggest removing the "Option<T>: Grammar<'_>" bounds.
impl<'a> Grammar<'a> for Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}
impl<'a> Grammar<'a> for Option<Separated<FieldGroup<'a, Box<Type<'a>>>, lex::Comma<'a>>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

impl<'a> StatementGrammar<'a> for Vec<Statement<'a>> {}
impl<'a> StatementGrammar<'a> for Option<Statement<'a>> {}
impl<'a> StatementGrammar<'a> for Statement<'a> {}
impl<'a> StatementGrammar<'a> for Use<'a> {}
impl<'a, E: StatementGrammar<'a>> StatementGrammar<'a> for Mod<'a, E> {}
impl<'a> StatementGrammar<'a>
    for Struct<'a, lex::Ident<'a>, Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>>>
{
}
impl<'a, E: ExpressionGrammar<'a>> StatementGrammar<'a> for Inline<'a, E> {}
impl<'a, T: TypeGrammar<'a>> StatementGrammar<'a> for Static<'a, T> {}
impl<'a, T: TypeGrammar<'a>, E: ExpressionGrammar<'a>> StatementGrammar<'a> for Const<'a, T, E> {}
impl<
        'a,
        T: TypeGrammar<'a>,
        L: ExpressionGrammar<'a>,
        R: ExpressionGrammar<'a>,
        I: StatementGrammar<'a>,
    > StatementGrammar<'a> for For<'a, T, L, R, I>
{
}
impl<'a, I: StatementGrammar<'a>> StatementGrammar<'a> for Loop<'a, I> {}
impl<'a, T: TypeGrammar<'a>, E: ExpressionGrammar<'a>> StatementGrammar<'a> for Let<'a, T, E> {}
impl<'a, I: StatementGrammar<'a>> StatementGrammar<'a> for Fn<'a, I> {}

// For parsing no-arguments (performs a no-op).
// Mainly used on the Struct statement, which behaves like a statement when it
// has an identifier, and like a type when it doesn't have.
impl<'a> Grammar<'a> for () {
    fn parse(_: &mut Context, _: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        Ok(())
    }
}

// Parsing of boxed types for self-referential types.
// Mostly used within expression syntax trees.
impl<'a, P: Grammar<'a>> Grammar<'a> for Box<P> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        P::parse(context, tokens).map(Box::new)
    }
}

// Parsing of boxed types for self-referential types.
// Mostly used within expression syntax trees.
impl<'a, P> Grammar<'a> for Vec<P>
where
    Option<P>: Grammar<'a>,
{
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        let mut vec = Vec::new();
        while let Some(item) = Grammar::parse(context, tokens)? {
            vec.push(item);
        }
        Ok(vec)
    }
}

pub enum Statement<'a> {
    Panic(Panic<'a>),
    Use(Use<'a>),
    Mod(Mod<'a, Vec<Statement<'a>>>),
    Struct(Struct<'a, lex::Ident<'a>, Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>>>),
    Union(Union<'a, lex::Ident<'a>, Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>>>),
    Asm(Asm<'a, Vec<asm::Asm<'a>>>),
    Static(Static<'a, Type<'a>>),
    Const(Const<'a, Type<'a>, Expression<'a>>),
    For(For<'a, Type<'a>, Expression<'a>, Expression<'a>, Vec<Statement<'a>>>),
    Loop(Loop<'a, Vec<Statement<'a>>>),
    Let(Let<'a, Type<'a>, Expression<'a>>),
    Fn(Fn<'a, Vec<Statement<'a>>>),
    Inline(Inline<'a, Expression<'a>>),
}

pub enum Type<'a> {
    U16(lex::U16<'a>),
    U8(lex::U8<'a>),
    Array(Array<'a, Box<Type<'a>>, Expression<'a>>),
    Struct(Struct<'a, (), Option<Separated<FieldGroup<'a, Box<Type<'a>>>, lex::Comma<'a>>>>),
    Union(Union<'a, (), Option<Separated<FieldGroup<'a, Box<Type<'a>>>, lex::Comma<'a>>>>),
    Pointer(Pointer<'a, Box<Type<'a>>>),
    Ident(lex::Ident<'a>),
}

impl<'a> Grammar<'a> for Option<Statement<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            None => Ok(None),
            Some(Ok(Token::BangBang(_))) => {
                Ok(Some(Statement::Panic(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Use(_))) => Ok(Some(Statement::Use(Grammar::parse(context, tokens)?))),
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
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken(token))
        }
    }
}

impl<'a> Grammar<'a> for Option<Type<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            Some(Err(_)) => {
                let err = tokens.next().unwrap().err().unwrap();
                Err(err)
            }
            Some(Ok(Token::U16(_))) => Ok(Some(Type::U16(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::U8(_))) => Ok(Some(Type::U8(Grammar::parse(context, tokens)?))),
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
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken(token))
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
    /// `!!`
    pub struct Panic<'a> {
        pub bang_bang: lex::BangBang<'a>,
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

parse! {
    /// `(<statement>)* EOF`
    pub struct Ast<'a, T>
    where
        T: Grammar<'a>,
    {
        pub inner: T,
        pub eof: lex::Eof<'a>,
    }
}

parse! {
    /// `use <path> ;`
    pub struct Use<'a> {
        pub use_: lex::Use<'a>,
        pub path: Separated<lex::Ident<'a>, lex::Square<'a>>,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse! {
    /// `mod <ident> { <statements> }`
    pub struct Mod<'a, I>
    where
        I: Grammar<'a>,
    {
        pub mod_: lex::Mod<'a>,
        pub ident: lex::Ident<'a>,
        pub left_bracket: lex::LeftBracket<'a>,
        pub inner: I,
        pub right_bracket: lex::RightBracket<'a>,
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

parse! {
    /// `[ <type> ; <length> ]`
    pub struct Array<'a, T, E>
    where
        T: Grammar<'a>,
        E: Grammar<'a>,
    {
        pub left_square: lex::LeftSquare<'a>,
        pub type_: T,
        pub semi_coloon: lex::SemiColon<'a>,
        pub len: E,
        pub right_square: lex::RightSquare<'a>,
    }
}

parse! {
    /// `struct [<ident>] { <fields> }`
    pub struct Struct<'a, I, F>
    where
        I: Grammar<'a>,
        F: Grammar<'a>,
    {
        pub struct_: lex::Struct<'a>,
        pub ident: I,
        pub left_bracket: lex::LeftBracket<'a>,
        pub fields: F,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `union [<ident>] { <fields> }`
    pub struct Union<'a, I, F>
    where
        I: Grammar<'a>,
        F: Grammar<'a>,
    {
        pub union: lex::Union<'a>,
        pub ident: I,
        pub left_bracket: lex::LeftBracket<'a>,
        pub fields: F,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `<ident> :: <type>`
    pub struct Field<'a, T>
    where
        T: Grammar<'a>,
    {
        pub ident: lex::Ident<'a>,
        pub square: lex::Square<'a>,
        pub type_: T,
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

impl<'a, T> Grammar<'a> for Option<FieldGroup<'a, T>>
where
    T: Grammar<'a>,
{
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            Some(Ok(Token::Ident(_))) => Ok(Some(Grammar::parse(context, tokens)?)),
            _ => Ok(None),
        }
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

parse! {
    /// `@ <expr>`
    pub struct StaticOffset<'a> {
        pub at: lex::At<'a>,
        pub lit: lex::Lit<'a>,
    }
}

impl<'a> Grammar<'a> for Option<StaticOffset<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
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
        pub field: Field<'a, T>,
        pub semi_colon: lex::SemiColon<'a>,
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
        pub field: Field<'a, T>,
        pub assign: lex::Assign<'a>,
        pub expr: E,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse! {
    /// `for <field> in <range> { (<statement>)* }`
    pub struct For<'a, T, L, R, I>
    where
        T: Grammar<'a>,
        L: Grammar<'a>,
        R: Grammar<'a>,
        I: Grammar<'a>,
    {
        pub for_: lex::For<'a>,
        pub field: Field<'a, T>,
        pub in_: lex::In<'a>,
        pub range: Range<'a, L, R>,
        pub left_bracket: lex::LeftBracket<'a>,
        pub inner: I,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `loop { (<statement>)* }`
    pub struct Loop<'a, I>
    where
        I: Grammar<'a>,
    {
        pub loop_: lex::Loop<'a>,
        pub left_bracket: lex::LeftBracket<'a>,
        pub inner: I,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    /// `let <field> = <expr> ;`
    pub struct Let<'a, T, E>
    where
        T: Grammar<'a>,
        E: Grammar<'a>,
    {
        pub let_: lex::Let<'a>,
        pub field: Field<'a, T>,
        pub assign: lex::Assign<'a>,
        pub expr: E,
        pub semi_colon: lex::SemiColon<'a>,
    }
}

parse! {
    /// `fn <ident> [<args>] <type> { }`
    pub struct Fn<'a, I>
    where
        I: Grammar<'a>,
    {
        pub fn_: lex::Fn<'a>,
        pub ident: lex::Ident<'a>,
        pub fn_args: Option<FnArgs<'a>>,
        pub type_: Option<Type<'a>>,
        pub left_bracket: lex::LeftBracket<'a>,
        pub inner: I,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    pub struct FnArgs<'a> {
        pub left_par: lex::LeftPar<'a>,
        pub args: Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>,
        pub right_par: lex::RightPar<'a>,
    }
}

impl<'a> Grammar<'a> for Option<FnArgs<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::LeftPar(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

pub struct Separated<T, S> {
    pub head: T,
    pub tail: Vec<(S, T)>,
}

impl<'a, T, S> Grammar<'a> for Separated<T, S>
where
    T: Grammar<'a>,
    S: Grammar<'a>,
    Option<S>: Grammar<'a>,
{
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        let head = Grammar::parse(context, tokens)?;
        let mut tail = Vec::new();
        while let Some(sep) = Grammar::parse(context, tokens)? {
            let item = Grammar::parse(context, tokens)?;
            tail.push((sep, item))
        }
        Ok(Self { head, tail })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn panic() {
        crate::parse_program("!!").unwrap();
    }

    #[test]
    fn use_() {
        crate::parse_program("use foo;").unwrap();
        crate::parse_program("use foo::bar;").unwrap();
    }

    #[test]
    #[should_panic]
    fn use_panic() {
        crate::parse_program("use foo::bar::;").unwrap();
    }

    #[test]
    fn mod_() {
        crate::parse_program("mod foo { mod bar { } } mod baz { }").unwrap();
    }

    #[test]
    fn struct_() {
        crate::parse_program("struct Foo { }").unwrap();
        crate::parse_program("struct Foo { a :: u8 }").unwrap();
        crate::parse_program("struct Foo { a :: u8, b :: u16 }").unwrap();
        crate::parse_program("struct Foo { a :: u8, b :: u16 , c :: struct { } }").unwrap();
        crate::parse_program("struct Foo { a :: u8, b :: u16 , c :: struct { a::[u8;42] } }")
            .unwrap();
    }

    #[test]
    fn union_() {
        crate::parse_program("union Foo { }").unwrap();
        crate::parse_program("union Foo { a :: u8 }").unwrap();
        crate::parse_program("union Foo { a :: u8, b :: u16 }").unwrap();
        crate::parse_program("union Foo { a :: u8, b :: u16 , c :: union { } }").unwrap();
        crate::parse_program("union Foo { a :: u8, b :: u16 , c :: union { a::[u8;42] } }")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn asm() {
        crate::parse_program("asm { }").unwrap();
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn asm_panic() {
        crate::parse_program("asm { loop { } }").unwrap();
    }

    #[test]
    fn static_() {
        crate::parse_program("static FOO :: [u8; 0x800];").unwrap();
        crate::parse_program("static@0x8000 FOO :: [u8; 0x800];").unwrap();
        crate::parse_program("static@0x8000 FOO :: [u8; 0x800];").unwrap();
    }

    #[test]
    #[should_panic]
    fn static_panic() {
        crate::parse_program("static FOO :: u8 = 0;").unwrap();
        crate::parse_program("static FOO :: u8 = 0;").unwrap();
    }

    #[test]
    fn const_() {
        crate::parse_program("const FOO :: u8 = 42;").unwrap();
        crate::parse_program("const FOO :: u8 = 42;").unwrap();
    }

    #[test]
    #[should_panic]
    fn const_panic() {
        crate::parse_program("const FOO, BAR :: u8 = 0;").unwrap();
    }

    #[test]
    fn for_() {
        crate::parse_program("for i::u8 in 0.. 42 { }").unwrap();
        crate::parse_program("for i::u8 in 0..=42 { }").unwrap();
        crate::parse_program("for i::u8 in 0..=+42 { }").unwrap();
    }

    #[test]
    fn loop_() {
        crate::parse_program("loop {}").unwrap();
        crate::parse_program("loop {} loop {}").unwrap();
        crate::parse_program("loop {loop{}} loop {}").unwrap();
    }

    #[test]
    fn let_() {
        crate::parse_program("let foo::u8 = 42;").unwrap();
        crate::parse_program("let foo_bar::u16 = 0xffff;").unwrap();
    }

    #[test]
    #[should_panic]
    fn let_panic() {
        crate::parse_program("let foo = 42;").unwrap();
    }

    #[test]
    fn fn_() {
        crate::parse_program("fn foo { }").unwrap();
        crate::parse_program("fn foo u8 { }").unwrap();
        crate::parse_program("fn foo(bar::u8) { }").unwrap();
        crate::parse_program("fn foo(bar::u8) u8 { }").unwrap();
        crate::parse_program("fn foo(bar::u8, baz::u16) u8 { }").unwrap();
    }

    #[test]
    #[ignore]
    fn inline() {
        crate::parse_program("1;").unwrap();
        crate::parse_program("foo;").unwrap();
        crate::parse_program("foo();").unwrap();
        crate::parse_program("foo = 42;").unwrap();
    }
}
