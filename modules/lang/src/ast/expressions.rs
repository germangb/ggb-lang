use crate::{
    ast::{Context, Error, Parse},
    lex,
    lex::{Token, Tokens},
};
use std::iter::Peekable;

/// Marker trait.
pub trait ParseExpression<'a>: Parse<'a> {}

impl<'a> ParseExpression<'a> for lex::Ident<'a> {}
impl<'a> ParseExpression<'a> for lex::Lit<'a> {}
impl<'a, E: ParseExpression<'a>> ParseExpression<'a> for Parenthesis<'a, E> {}
impl<'a, L: ParseExpression<'a>, R: ParseExpression<'a>> ParseExpression<'a> for Add<'a, L, R> {}
impl<'a, L: ParseExpression<'a>, R: ParseExpression<'a>> ParseExpression<'a> for Sub<'a, L, R> {}
impl<'a, L: ParseExpression<'a>, R: ParseExpression<'a>> ParseExpression<'a> for Mul<'a, L, R> {}
impl<'a, L: ParseExpression<'a>, R: ParseExpression<'a>> ParseExpression<'a> for Div<'a, L, R> {}
impl<'a, E: ParseExpression<'a>> ParseExpression<'a> for Minus<'a, E> {}
impl<'a, E: ParseExpression<'a>> ParseExpression<'a> for Address<'a, E> {}

pub enum Expression<'a> {
    Ident(lex::Ident<'a>),
    Lit(lex::Lit<'a>),
    Parenthesis(Parenthesis<'a, Box<Expression<'a>>>),
    Add(Add<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Sub(Sub<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Mul(Mul<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Div(Div<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    And(And<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Or(Or<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Xor(Xor<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Minus(Minus<'a, Box<Expression<'a>>>),
    Address(Address<'a, Box<Expression<'a>>>),
}

// TODO incomplete implementation
impl<'a> Parse<'a> for Expression<'a> {
    fn parse(context: &mut Context, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error> {
        let mut left = parse_expression_atom(context, tokens)?;
        loop {
            match tokens.peek() {
                Some(Ok(Token::Plus(_))) => {
                    left = Expression::Add(Add {
                        left: Box::new(left),
                        plus: Parse::parse(context, tokens)?,
                        right: parse_expression_atom(context, tokens).map(Box::new)?,
                    })
                }
                Some(Ok(Token::Minus(_))) => {
                    left = Expression::Sub(Sub {
                        left: Box::new(left),
                        minus: Parse::parse(context, tokens)?,
                        right: parse_expression_atom(context, tokens).map(Box::new)?,
                    })
                }
                _ => return Ok(left),
            }
        }
    }
}

// <left> + <right>
// <left> - <right>
fn parse_expression_plus_minus<'a>(
    context: &mut Context,
    tokens: &mut Peekable<Tokens<'a>>,
) -> Result<Expression<'a>, Error> {
    unimplemented!()
}

// <left> * <right>
// <left> / <right>
fn parse_expression_mul_div<'a>(
    context: &mut Context,
    tokens: &mut Peekable<Tokens<'a>>,
) -> Result<Expression<'a>, Error> {
    unimplemented!()
}

// parses an expression of highest precedence: either a LITERAL, an IDENTIFIER,
// a PARENTHESIS expression, or an UNARY expression (plus, minus or address).
fn parse_expression_atom<'a>(
    context: &mut Context,
    tokens: &mut Peekable<Tokens<'a>>,
) -> Result<Expression<'a>, Error> {
    match tokens.peek() {
        Some(Ok(Token::LeftPar(_))) => Ok(Expression::Parenthesis(Parse::parse(context, tokens)?)),
        Some(Ok(Token::Lit(_))) => Ok(Expression::Lit(Parse::parse(context, tokens)?)),
        Some(Ok(Token::Ident(_))) => Ok(Expression::Ident(Parse::parse(context, tokens)?)),
        Some(Ok(Token::Minus(_))) => Ok(Expression::Minus(Parse::parse(context, tokens)?)),
        Some(Ok(Token::Ampersand(_))) => Ok(Expression::Address(Parse::parse(context, tokens)?)),
        _ => {
            // TODO error reporting
            let _token = tokens.next().expect("Token please");
            Err(Error::UnexpectedToken)
        }
    }
}

parse! {
    /// `( <expr> )`
    pub struct Parenthesis<'a, E> {
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
        pub minus: lex::Minus<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> * <expr>`
    pub struct Mul<'a, L, R> {
        pub left: L,
        pub star: lex::Star<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> / <expr>`
    pub struct Div<'a, L, R> {
        pub left: L,
        pub div: lex::Div<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> & <expr>`
    pub struct And<'a, L, R> {
        pub left: L,
        pub ampersand: lex::Ampersand<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> | <expr>`
    pub struct Or<'a, L, R> {
        pub left: L,
        pub pipe: lex::Pipe<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> ^ <expr>`
    pub struct Xor<'a, L, R> {
        pub left: L,
        pub caret: lex::Caret<'a>,
        pub right: R,
    }
}

parse! {
    /// `- <expr>`
    pub struct Minus<'a, E> {
        pub minus: lex::Minus<'a>,
        pub inner: E,
    }
}

parse! {
    /// `& <expr>`
    pub struct Address<'a, E> {
        pub ampersand: lex::Ampersand<'a>,
        pub inner: E,
    }
}

#[cfg(test)]
mod test {
    use crate::ast::expressions::{
        Add, Div, Expression as E, Expression, Minus, Mul, Parenthesis, Sub,
    };

    macro_rules! test_expr {
        ($expr:expr) => {
            assert_eq!($expr, eval(&crate::parse(stringify!($expr)).unwrap()));
        };
    }

    #[test]
    #[ignore]
    fn precedence() {
        // assert!(matches!(crate::parse("1+2-3  "), Ok(E::Add(Add { .. }))));
        // assert!(matches!(crate::parse("1-2+3  "), Ok(E::Sub(Sub { .. }))));
        // assert!(matches!(crate::parse("1+(2-3)"), Ok(E::Add(Add { .. }))));
        // assert!(matches!(crate::parse("1-(2+3)"), Ok(E::Sub(Sub { .. }))));
        // assert!(matches!(crate::parse("1*2+3  "), Ok(E::Add(Add { .. }))));
        // assert!(matches!(crate::parse("1+2*3  "), Ok(E::Add(Add { .. }))));
        //
        // assert!(matches!(crate::parse("1*(2+3/4)"), Ok(E::Mul(Mul { .. }))));
        // assert!(matches!(crate::parse("2+3/4    "), Ok(E::Add(Add { .. }))));
        // assert!(matches!(crate::parse("2/3+4    "), Ok(E::Add(Add { .. }))));
        // assert!(matches!(crate::parse("2/(3+4)  "), Ok(E::Div(Div { .. }))));

        test_expr!(1 + 2);
        test_expr!(-1 + 2);

        test_expr!(1 - 2);
        test_expr!(-1 - 2);

        test_expr!(1 + 1 + 1 + 1 + 1);
        test_expr!(1 + 1 - 1 - 1);
        test_expr!(1 - 9 + 2 - 8 + 3 - 7 + 4 - 6 + 5 - 5 + 6 - 4 + 7 - 3 + 8 - 2 + 9 - 1);
        test_expr!(1 - (9 + 2 - (8 + 3 - (7 + 4) - 6 + 5) - 5) + (6 - 4 + (7 - 3 + 8 - 2)) + 9 - 1);
        test_expr!(1 - ----------1);
        //test_expr!(1 + 2 - 3);
        //test_expr!(1 - 2 + 3);
        // assert_eq!(1 + (2 - 3), eval(&crate::parse("1+(2-3)").unwrap()));
        // assert_eq!(1 - (2 + 3), eval(&crate::parse("1-(2+3)").unwrap()));
        // assert_eq!(1 * 2 + 3, eval(&crate::parse("1*2+3").unwrap()));
        // assert_eq!(1 + 2 * 3, eval(&crate::parse("1+2*3").unwrap()));
    }

    fn eval(expr: &Expression) -> i64 {
        match expr {
            Expression::Parenthesis(Parenthesis { inner, .. }) => eval(inner),
            Expression::Minus(Minus { inner, .. }) => -eval(inner),
            Expression::Lit(lit) => format!("{}", lit).parse().unwrap(),
            Expression::Add(Add { left, right, .. }) => eval(left) + eval(right),
            Expression::Sub(Sub { left, right, .. }) => eval(left) - eval(right),
            Expression::Mul(Mul { left, right, .. }) => eval(left) * eval(right),
            Expression::Div(Div { left, right, .. }) => eval(left) / eval(right),
            _ => unreachable!(),
        }
    }
}
