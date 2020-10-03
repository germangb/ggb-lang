//! Expression grammars.
//!
//! # Expressions
//!
//! - `E := <lit>`
//! - `E := <ident>`
//! - `E := ( E )`
//! - `E := E + E`
//! - `E := E - E`
//! - `E := E & E`
//! - `E := E | E`
//! - `E := E ^ E`
//! - `E := - E`
//! - `E := @ E` (address of)
//! - `E := * E`
//! - `E := E = E`
//! - `E := E += E`
//! - `E := E -= E`
//! - `E := E &= E`
//! - `E := E |= E`
//! - `E := E ^= E`
//! - `E := E << E`
//! - `E := E >> E`
//!
//! # Features
//!
//! - `cfg(feature = "mul")`
//!     - `E := E * E`
//!     - `E := E *= E`
//!
//! - `cfg(feature = "div")`
//!     - `E := E / E`
//!     - `E := E /= E`
//!
use crate::{
    ast::{Context, Grammar, Separated},
    error::Error,
    lex,
    lex::{Token, Tokens},
};
use std::iter::Peekable;

pub trait ExpressionGrammar<'a>: Grammar<'a> {}

// terminals
impl<'a> ExpressionGrammar<'a> for lex::Ident<'a> {}
impl<'a> ExpressionGrammar<'a> for lex::Lit<'a> {}

// unary | prefix
impl<'a, E> ExpressionGrammar<'a> for Parenthesis<'a, E> where E: ExpressionGrammar<'a> {}
impl<'a, E> ExpressionGrammar<'a> for Minus<'a, E> where E: ExpressionGrammar<'a> {}
impl<'a, E> ExpressionGrammar<'a> for AddressOf<'a, E> where E: ExpressionGrammar<'a> {}
impl<'a, E> ExpressionGrammar<'a> for Deref<'a, E> where E: ExpressionGrammar<'a> {}

// binary
impl<'a, L, R> ExpressionGrammar<'a> for Add<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for Sub<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
#[cfg(feature = "mul")]
impl<'a, L, R> ExpressionGrammar<'a> for Mul<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
#[cfg(feature = "div")]
impl<'a, L, R> ExpressionGrammar<'a> for Div<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for Assign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for PlusAssign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for MinusAssign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
#[cfg(feature = "mul")]
impl<'a, L, R> ExpressionGrammar<'a> for MulAssign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
#[cfg(feature = "div")]
impl<'a, L, R> ExpressionGrammar<'a> for DivAssign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for AmpersandAssign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for PipeAssign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for CaretAssign<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for Index<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for Field<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for LeftShift<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}
impl<'a, L, R> ExpressionGrammar<'a> for RightShift<'a, L, R>
where
    L: ExpressionGrammar<'a>,
    R: ExpressionGrammar<'a>,
{
}

pub enum Expression<'a> {
    Ident(lex::Ident<'a>),
    Lit(lex::Lit<'a>),
    Parenthesis(Parenthesis<'a, Box<Expression<'a>>>),
    Add(Add<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Sub(Sub<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    #[cfg(feature = "mul")]
    Mul(Mul<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    #[cfg(feature = "div")]
    Div(Div<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    And(And<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Or(Or<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Xor(Xor<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Minus(Minus<'a, Box<Expression<'a>>>),
    AddressOf(AddressOf<'a, Box<Expression<'a>>>),
    Assign(Assign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    PlusAssign(PlusAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    MinusAssign(MinusAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    #[cfg(feature = "mul")]
    MulAssign(MulAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    #[cfg(feature = "div")]
    DivAssign(DivAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    AmpersandAssign(AmpersandAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    PipeAssign(PipeAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    CaretAssign(CaretAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Call(Call<'a, Box<Expression<'a>>, Box<Separated<Expression<'a>, lex::Comma<'a>>>>),
    Index(Index<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Deref(Deref<'a, Box<Expression<'a>>>),
    Field(Field<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    LeftShift(LeftShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    RightShift(RightShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
}

// TODO incomplete implementation
impl<'a> Grammar<'a> for Expression<'a> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        match tokens.next() {
            Some(Ok(Token::Ident(ident))) => Ok(Expression::Ident(ident)),
            Some(Ok(Token::Lit(lit))) => Ok(Expression::Lit(lit)),
            _ => unimplemented!(),
        }
        //parse_expr(NONE, context, tokens)
    }
}

impl<'a> Grammar<'a> for Option<Expression<'a>> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        unimplemented!()
    }
}

type Precedence = u8;

const NONE: Precedence = 0;
const ADD: Precedence = 1; // +
const SUB: Precedence = 2; // -
const MUL: Precedence = 3; // *
const DIV: Precedence = 4; // /
const OR: Precedence = 5; // |
const XOR: Precedence = 6; // ^
const AND: Precedence = 7; // &
const ADDRESS: Precedence = 8; // &a (unary)
const MINUS: Precedence = 8; // -a (unary)
const FIELD: Precedence = 9; // ::
const CALL: Precedence = 10; // (
const INDEX: Precedence = 10; // [

// parses an expression of operator <= than precedence.
// recurse if an operator of > precedence is found.
fn parse_expr<'a>(
    precedence: Precedence,
    context: &mut Context<'a, '_>,
    tokens: &mut Peekable<Tokens<'a>>,
) -> Result<Expression<'a>, Error<'a>> {
    let mut left = None;
    loop {
        break;
        // parse lhs of the expression
        // at this point it can be either: unary, parenthesis
        match tokens.peek() {
            Some(Ok(Token::Lit(_))) => {
                let lit = Grammar::parse(context, tokens)?;
            }
            _ => break,
        }
    }
    left.expect("Expression please")
}

parse! {
    /// `( <expr> )`
    pub struct Parenthesis<'a, E>
    where
        E: Grammar<'a>,
    {
        pub left_par: lex::LeftPar<'a>,
        pub inner: E,
        pub right_par: lex::RightPar<'a>,
    }
}

parse! {
    /// `<expr> + <expr>`
    pub struct Add<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub plus: lex::Plus<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> - <expr>`
    pub struct Sub<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub minus: lex::Minus<'a>,
        pub right: R,
    }
}

#[cfg(feature = "mul")]
parse! {
    /// `<expr> * <expr>`
    pub struct Mul<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub star: lex::Star<'a>,
        pub right: R,
    }
}

#[cfg(feature = "div")]
parse! {
    /// `<expr> / <expr>`
    pub struct Div<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub slash: lex::Slash<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> & <expr>`
    pub struct And<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub ampersand: lex::Ampersand<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> | <expr>`
    pub struct Or<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub pipe: lex::Pipe<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> ^ <expr>`
    pub struct Xor<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub caret: lex::Caret<'a>,
        pub right: R,
    }
}

parse! {
    /// `- <expr>`
    pub struct Minus<'a, E>
    where
        E: Grammar<'a>,
    {
        pub minus: lex::Minus<'a>,
        pub inner: E,
    }
}

parse! {
    /// `@ <expr>`
    pub struct AddressOf<'a, E>
    where
        E: Grammar<'a>,
    {
        pub at: lex::At<'a>,
        pub inner: E,
    }
}

parse! {
    /// `<expr> = <expr>`
    pub struct Assign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub assign: lex::Assign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> += <expr>`
    pub struct PlusAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub plus_assign: lex::PlusAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> -= <expr>`
    pub struct MinusAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub minus_assign: lex::MinusAssign<'a>,
        pub right: R,
    }
}

#[cfg(feature = "mul")]
parse! {
    /// `<expr> *= <expr>`
    pub struct MulAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub star_assign: lex::StarAssign<'a>,
        pub right: R,
    }
}

#[cfg(feature = "div")]
parse! {
    /// `<expr> *= <expr>`
    pub struct DivAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub slash_assign: lex::SlashAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> &= <expr>`
    pub struct AmpersandAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub ampersand_assign: lex::AmpersandAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> |= <expr>`
    pub struct PipeAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub pipe_assign: lex::PipeAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> ^= <expr>`
    pub struct CaretAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub caret_assign: lex::CaretAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> ( <args> )`
    pub struct Call<'a, L, A>
    where
        L: Grammar<'a>,
        A: Grammar<'a>,
    {
        pub left: L,
        pub left_par: lex::LeftPar<'a>,
        pub args: A,
        pub right_par: lex::RightPar<'a>,
    }
}

parse! {
    /// `<expr> [ <expr> ]`
    pub struct Index<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub left_square: lex::LeftSquare<'a>,
        pub right: R,
        pub right_square: lex::RightSquare<'a>,
    }
}

parse! {
    /// `* <expr>`
    pub struct Deref<'a, E>
    where
        E: Grammar<'a>,
    {
        pub star: lex::Star<'a>,
        pub inner: E,
    }
}

parse! {
    /// `<left> :: <right>`
    pub struct Field<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub square: lex::Square<'a>,
        pub right: R,
    }
}

parse! {
    /// `<left> << <right>`
    pub struct LeftShift<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub less_less: lex::LessLess<'a>,
        pub right: R,
    }
}

parse! {
    /// `<left> >> <right>`
    pub struct RightShift<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left: L,
        pub great_great: lex::GreatGreat<'a>,
        pub right: R,
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "div")]
    use crate::ast::expressions::Div;
    #[cfg(feature = "mul")]
    use crate::ast::expressions::Mul;
    use crate::ast::expressions::{Add, Expression as E, Expression, Minus, Parenthesis, Sub};

    macro_rules! test_expr {
        ($expr:expr) => {
            assert_eq!(
                $expr,
                eval(
                    &crate::parse::<crate::ast::expressions::Expression>(stringify!($expr))
                        .unwrap()
                )
            );
        };
    }

    #[test]
    #[ignore]
    fn precedence() {
        test_expr!(1 + 2);
    }

    fn eval(expr: &Expression) -> i64 {
        match expr {
            Expression::Parenthesis(Parenthesis { inner, .. }) => eval(inner),
            Expression::Minus(Minus { inner, .. }) => -eval(inner),
            Expression::Lit(lit) => format!("{}", lit).parse().unwrap(),
            Expression::Add(Add { left, right, .. }) => eval(left) + eval(right),
            Expression::Sub(Sub { left, right, .. }) => eval(left) - eval(right),
            #[cfg(feature = "mul")]
            Expression::Mul(Mul { left, right, .. }) => eval(left) * eval(right),
            #[cfg(feature = "div")]
            Expression::Div(Div { left, right, .. }) => eval(left) / eval(right),
            _ => unreachable!(),
        }
    }
}
