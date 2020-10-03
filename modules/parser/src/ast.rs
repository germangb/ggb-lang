//! Grammar definitions and syntactic analysis.
//!
//! # Statements
//! - `S := !!` (panic)
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
//!
//! # Example
//!
//! ```
//! # use parser::Ast;
//! # parser::parse::<Ast>(r#"
//! // adds a layer of typing to an existing region of memory
//! // here, VRAM starts at address 0x8000 ans is layed out like this:
//! static@0x8000 VRAM :: struct {
//!     tile_data :: union {
//!         x8000 :: struct {                        data::[u8; 0x1000] },
//!         x8800 :: struct { _padding::[u8; 0x800], data::[u8; 0x1000] }
//!     },
//!     tile_map :: struct { x9800::[u8; 0x400],
//!                          x9c00::[i8; 0x400] }
//! };
//!
//! // C-style for loop
//! for offset::u16 in 0x40..+16 {
//!     //VRAM::tile_data::x8000[offset] = 0xff;
//! }
//!
//! //VRAM::tile_map::x9800[0] = 4;
//! # "#).unwrap();
//! ```
use crate::{
    ast::expressions::{Expression, ExpressionGrammar},
    error::Error,
    lex,
    lex::{Token, Tokens},
};
use std::iter::Peekable;

// re-export parsing context.
pub use context::{Context, ContextBuilder};

#[macro_use]
mod macros;
mod context;

pub mod asm;
pub mod expressions;

// user-facing API functions:

pub fn parse<'a, P: Grammar<'a>>(input: &'a str) -> Result<P, Error<'a>> {
    let mut context = Context::default();
    parse_with_context(input, &mut context)
}

pub fn parse_with_context<'a, P: Grammar<'a>>(
    input: &'a str,
    context: &mut Context<'a, '_>,
) -> Result<P, Error<'a>> {
    let mut tokens = Tokens::new(input).peekable();
    P::parse(context, &mut tokens)
}

// grammar definitions

pub trait Grammar<'a>: Sized {
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>>;
}

// #[doc(hidden)]
// pub trait TypeGrammar<'a>: Grammar<'a> {}
// #[doc(hidden)]
// pub trait StatementGrammar<'a>: Grammar<'a> {}

impl<'a> From<lex::U16<'a>> for Type<'a> {
    fn from(ty: lex::U16<'a>) -> Self {
        Type::U16(ty)
    }
}

impl<'a> From<lex::U8<'a>> for Type<'a> {
    fn from(ty: lex::U8<'a>) -> Self {
        Type::U8(ty)
    }
}

impl<'a, T, E> From<Array<'a, T, E>> for Type<'a>
where
    T: Into<Type<'a>>,
    E: Into<Expression<'a>>,
{
    fn from(ty: Array<'a, T, E>) -> Self {
        let Array {
            left_square,
            type_,
            semi_colon,
            len,
            right_square,
        } = ty;
        Type::Array(Array {
            left_square,
            type_: Box::new(type_.into()),
            semi_colon,
            len: len.into(),
            right_square,
        })
    }
}

impl<'a> From<Struct<'a, ()>> for Type<'a> {
    fn from(ty: Struct<'a, ()>) -> Self {
        Type::Struct(Box::new(ty))
    }
}

impl<'a> From<Union<'a, ()>> for Type<'a> {
    fn from(ty: Union<'a, ()>) -> Self {
        Type::Union(Box::new(ty))
    }
}

impl<'a, T> From<Pointer<'a, T>> for Type<'a>
where
    T: Into<Type<'a>>,
{
    fn from(ty: Pointer<'a, T>) -> Self {
        let Pointer { ampersand, type_ } = ty;
        Type::Pointer(Pointer {
            ampersand,
            type_: Box::new(type_.into()),
        })
    }
}

impl<'a> From<lex::Ident<'a>> for Type<'a> {
    fn from(ty: lex::Ident<'a>) -> Self {
        Type::Ident(ty)
    }
}

// impl<'a> TypeGrammar<'a> for Type<'a> {}
// impl<'a> TypeGrammar<'a> for lex::U8<'a> {}
// impl<'a> TypeGrammar<'a> for lex::U16<'a> {}
// impl<'a, T: TypeGrammar<'a>, E: ExpressionGrammar<'a>> TypeGrammar<'a> for
// Array<'a, T, E> {} impl<'a> TypeGrammar<'a> for Struct<'a, ()> {}
// impl<'a> TypeGrammar<'a> for Union<'a, ()> {}
// impl<'a, T: TypeGrammar<'a>> TypeGrammar<'a> for Pointer<'a, T> {}

// FIXME
//  fuck this, I mean fix it.
//  Generics are fucked somewhere in this typed mess...
//  I'd suggest removing the "Option<T>: Grammar<'_>" bounds.
impl<'a> Grammar<'a> for Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>> {
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}
impl<'a> Grammar<'a> for Option<Separated<FieldGroup<'a, Box<Type<'a>>>, lex::Comma<'a>>> {
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

// impl<'a> StatementGrammar<'a> for Vec<Statement<'a>> {}
// impl<'a> StatementGrammar<'a> for Option<Statement<'a>> {}
// impl<'a> StatementGrammar<'a> for Statement<'a> {}
// impl<'a, E: StatementGrammar<'a>> StatementGrammar<'a> for Mod<'a, E> {}
// impl<'a> StatementGrammar<'a> for Struct<'a, lex::Ident<'a>> {}
// impl<'a, E: ExpressionGrammar<'a>> StatementGrammar<'a> for Inline<'a, E> {}
// impl<'a, T: TypeGrammar<'a>> StatementGrammar<'a> for Static<'a, T> {}
// impl<'a, T: TypeGrammar<'a>, E: ExpressionGrammar<'a>> StatementGrammar<'a>
// for Const<'a, T, E> {} impl<
//         'a,
//         T: TypeGrammar<'a>,
//         L: ExpressionGrammar<'a>,
//         R: ExpressionGrammar<'a>,
//         I: StatementGrammar<'a>,
//     > StatementGrammar<'a> for For<'a, T, L, R, I>
// {
// }
// impl<'a, I: StatementGrammar<'a>> StatementGrammar<'a> for Loop<'a, I> {}
// impl<'a, T: TypeGrammar<'a>, E: ExpressionGrammar<'a>> StatementGrammar<'a>
// for Let<'a, T, E> {} impl<'a, I: StatementGrammar<'a>> StatementGrammar<'a>
// for Fn<'a, I> {}

// For parsing no-arguments (performs a no-op).
// Mainly used on the Struct statement, which behaves like a statement when it
// has an identifier, and like a type when it doesn't have.
impl<'a> Grammar<'a> for () {
    fn parse(_: &mut Context<'a, '_>, _: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        Ok(())
    }
}

// Parsing of boxed types for self-referential types.
// Mostly used within expression syntax trees.
impl<'a, P: Grammar<'a>> Grammar<'a> for Box<P> {
    fn parse(
        context: &mut Context<'a, '_>,
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
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let mut vec = Vec::new();
        while let Some(item) = Grammar::parse(context, tokens)? {
            vec.push(item);
        }
        Ok(vec)
    }
}

pub enum Statement<'a> {
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
}

// TODO function type
pub enum Type<'a> {
    U16(lex::U16<'a>),
    U8(lex::U8<'a>),
    Array(Array<'a, Box<Type<'a>>, Expression<'a>>),
    Struct(Box<Struct<'a, ()>>),
    Union(Box<Union<'a, ()>>),
    Pointer(Pointer<'a, Box<Type<'a>>>),
    Ident(lex::Ident<'a>),
}

impl<'a> Grammar<'a> for Option<Statement<'a>> {
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            None => Ok(None),
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
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
    pub struct Ast<'a> {
        pub inner: Vec<Statement<'a>>,
        pub eof: lex::Eof<'a>,
    }
}

parse! {
    /// `mod <ident> { <statements> }`
    pub struct Mod<'a, I>
    where
        I: Grammar<'a>,
    {
        pub mod_: lex::Mod<'a>,
        pub ident: ModIdent<'a>,
        pub left_bracket: lex::LeftBracket<'a>,
        pub inner: I,
        pub right_bracket: lex::RightBracket<'a>,
    }
}

pub struct ModIdent<'a>(pub lex::Ident<'a>);

impl<'a> Grammar<'a> for ModIdent<'a> {
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let ident: lex::Ident = Grammar::parse(context, tokens)?;
        context.define_mod(ident.clone());
        Ok(Self(ident))
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
        pub semi_colon: lex::SemiColon<'a>,
        pub len: E,
        pub right_square: lex::RightSquare<'a>,
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
        pub fields: Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>>,
        pub right_bracket: lex::RightBracket<'a>,
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
        pub fields: Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>>,
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
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
    fn parse(
        context: &mut Context<'a, '_>,
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
    fn parse(
        context: &mut Context<'a, '_>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
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
    fn parse(
        context: &mut Context<'a, '_>,
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

#[cfg(test)]
mod test {
    use crate::ast::Ast;

    fn parse_program(input: &str) -> Ast {
        super::parse(input).unwrap()
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
        parse_program("struct Foo { a :: u8, b :: u16 }");
        parse_program("struct Foo { a :: u8, b :: u16 , c :: struct { } }");
        parse_program("struct Foo { a :: u8, b :: u16 , c :: struct { a::[u8;42] } }");
    }

    #[test]
    fn union_() {
        parse_program("union Foo { }");
        parse_program("union Foo { a :: u8 }");
        parse_program("union Foo { a :: u8, b :: u16 }");
        parse_program("union Foo { a :: u8, b :: u16 , c :: union { } }");
        parse_program("union Foo { a :: u8, b :: u16 , c :: union { a::[u8;42] } }");
    }

    #[test]
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
        parse_program("let foo_bar::u16 = 0xffff;");
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
        parse_program("fn foo(bar::u8, baz::u16) u8 { }");
    }

    #[test]
    fn inline() {
        parse_program("1;");
        parse_program("foo;");
        parse_program("foo();");
        parse_program("foo::bar(0, 1, 2);");
        parse_program("foo = 42;");
        parse_program("foo += 42;");
    }
}
