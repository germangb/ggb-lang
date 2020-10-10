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
    ast::{context::Context, Grammar, Path},
    error::Error,
    lex,
    lex::{Token, Tokens},
    span::{union, Span, Spanned},
};
use std::iter::Peekable;

pub enum Expression<'a> {
    Path(Path<'a>),
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
    //Call(Call<'a, Box<Expression<'a>>, Box<Separated<Expression<'a>, lex::Comma<'a>>>>),
    Index(Index<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Deref(Deref<'a, Box<Expression<'a>>>),
    LeftShift(LeftShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    RightShift(RightShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
}

// TODO incomplete implementation
impl<'a> Grammar<'a> for Expression<'a> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            Some(Ok(Token::Ident(_))) => Ok(Expression::Path(Grammar::parse(context, tokens)?)),
            Some(Ok(Token::Lit(_))) => Ok(Expression::Lit(Grammar::parse(context, tokens)?)),
            _ => unimplemented!(),
        }
    }
}

parse! {
    /// `( <expressions> )`
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
    /// `<expressions> + <expressions>`
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
    /// `<expressions> - <expressions>`
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
    /// `<expressions> * <expressions>`
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
    /// `<expressions> / <expressions>`
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
    /// `<expressions> & <expressions>`
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
    /// `<expressions> | <expressions>`
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
    /// `<expressions> ^ <expressions>`
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
    /// `- <expressions>`
    pub struct Minus<'a, E>
    where
        E: Grammar<'a>,
    {
        pub minus: lex::Minus<'a>,
        pub inner: E,
    }
}

parse! {
    /// `@ <expressions>`
    pub struct AddressOf<'a, E>
    where
        E: Grammar<'a>,
    {
        pub at: lex::At<'a>,
        pub inner: E,
    }
}

parse! {
    /// `<expressions> = <expressions>`
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
    /// `<expressions> += <expressions>`
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
    /// `<expressions> -= <expressions>`
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
    /// `<expressions> *= <expressions>`
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
    /// `<expressions> /= <expressions>`
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
    /// `<expressions> &= <expressions>`
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
    /// `<expressions> |= <expressions>`
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
    /// `<expressions> ^= <expressions>`
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
    /// `<expressions> ( <args> )`
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
    /// `<expressions> [ <expressions> ]`
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
    /// `* <expressions>`
    pub struct Deref<'a, E>
    where
        E: Grammar<'a>,
    {
        pub star: lex::Star<'a>,
        pub inner: E,
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
#[cfg(nope)]
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
