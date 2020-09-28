use crate::{
    ast::{Context, Error, Parse},
    lex,
    lex::{Token, Tokens},
};
use std::iter::Peekable;

/// Marker trait.
pub trait ParseExpression<'a>: Parse<'a> {}

// terminals
impl<'a> ParseExpression<'a> for lex::Ident<'a> {}
impl<'a> ParseExpression<'a> for lex::Lit<'a> {}

// unary | prefix
impl<'a, E> ParseExpression<'a> for Parenthesis<'a, E> where E: ParseExpression<'a> {}
impl<'a, E> ParseExpression<'a> for Minus<'a, E> where E: ParseExpression<'a> {}
impl<'a, E> ParseExpression<'a> for Address<'a, E> where E: ParseExpression<'a> {}
impl<'a, E> ParseExpression<'a> for Deref<'a, E> where E: ParseExpression<'a> {}

// binary
impl<'a, L, R> ParseExpression<'a> for Add<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for Sub<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for Mul<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for Div<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for Assign<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for PlusAssign<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for MinusAssign<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for StarAssign<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for AmpersandAssign<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for PipeAssign<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for CaretAssign<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for Index<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}
impl<'a, L, R> ParseExpression<'a> for Field<'a, L, R>
where
    L: ParseExpression<'a>,
    R: ParseExpression<'a>,
{
}

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
    Assign(Assign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    PlusAssign(PlusAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    MinusAssign(MinusAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    StarAssign(StarAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    AmpersandAssign(AmpersandAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    PipeAssign(PipeAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    CaretAssign(CaretAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Call(Call<'a, Box<Expression<'a>>, Vec<(Expression<'a>, lex::Comma<'a>)>>),
    Index(Index<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
    Deref(Deref<'a, Box<Expression<'a>>>),
    Field(Field<'a, Box<Expression<'a>>, Box<Expression<'a>>>),
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

parse! {
    /// `<expr> = <expr>`
    pub struct Assign<'a, L, R> {
        pub left: L,
        pub assign: lex::Assign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> += <expr>`
    pub struct PlusAssign<'a, L, R> {
        pub left: L,
        pub plus_assign: lex::PlusAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> -= <expr>`
    pub struct MinusAssign<'a, L, R> {
        pub left: L,
        pub minus_assign: lex::MinusAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> *= <expr>`
    pub struct StarAssign<'a, L, R> {
        pub left: L,
        pub star_assign: lex::StarAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> &= <expr>`
    pub struct AmpersandAssign<'a, L, R> {
        pub left: L,
        pub ampersand_assign: lex::AmpersandAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> |= <expr>`
    pub struct PipeAssign<'a, L, R> {
        pub left: L,
        pub pipe_assign: lex::PipeAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> ^= <expr>`
    pub struct CaretAssign<'a, L, R> {
        pub left: L,
        pub caret_assign: lex::CaretAssign<'a>,
        pub right: R,
    }
}

parse! {
    /// `<expr> ( <args> )`
    pub struct Call<'a, L, A> {
        pub left: L,
        pub left_par: lex::LeftPar<'a>,
        pub args: A,
        pub right_par: lex::RightPar<'a>,
    }
}

parse! {
    /// `<expr> [ <expr> ]`
    pub struct Index<'a, L, R> {
        pub left: L,
        pub left_square: lex::LeftSquare<'a>,
        pub right: R,
        pub right_square: lex::RightSquare<'a>,
    }
}

parse! {
    /// `* <expr>`
    pub struct Deref<'a, E> {
        pub star: lex::Star<'a>,
        pub inner: E,
    }
}

parse! {
    /// `<left> :: <right>`
    pub struct Field<'a, L, R> {
        pub left: L,
        pub square: lex::Square<'a>,
        pub right: R,
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
