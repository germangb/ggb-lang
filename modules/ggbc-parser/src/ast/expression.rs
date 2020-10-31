//! Expression grammars.
use crate::{
    ast::{Context, Grammar, Path},
    error::Error,
    lex,
    lex::{
        span::{union, Span, Spanned},
        Token, Tokens,
    },
};
use std::iter::Peekable;

// binary expressions are surrounded by parenthesis: (+ 1 2), (== foo 42),
// etc... unary expressions don't: @foo, *bar, -42tc...
parse_enum! {
    #[derive(Debug)]
    pub enum Expression<'a> {
        // terminals
        Path(Path<'a>),
        Lit(lex::Lit<'a>),

        // Array
        Array(Array<'a>),

        // unary
        Minus(Box<Minus<'a>>),
        AddressOf(Box<AddressOf<'a>>),
        Deref(Box<Deref<'a>>),
        Not(Box<Not<'a>>),

        // binary
        Add(Box<LispNode<'a, Add<'a>>>),
        Sub(Box<LispNode<'a, Sub<'a>>>),
        Mul(Box<LispNode<'a, Mul<'a>>>),
        Div(Box<LispNode<'a, Div<'a>>>),
        And(Box<LispNode<'a, And<'a>>>),
        Or(Box<LispNode<'a, Or<'a>>>),
        Xor(Box<LispNode<'a, Xor<'a>>>),
        Assign(Box<LispNode<'a, Assign<'a>>>),
        PlusAssign(Box<LispNode<'a, PlusAssign<'a>>>),
        MinusAssign(Box<LispNode<'a, MinusAssign<'a>>>),
        MulAssign(Box<LispNode<'a, MulAssign<'a>>>),
        DivAssign(Box<LispNode<'a, DivAssign<'a>>>),
        AndAssign(Box<LispNode<'a, AndAssign<'a>>>),
        OrAssign(Box<LispNode<'a, OrAssign<'a>>>),
        XorAssign(Box<LispNode<'a, XorAssign<'a>>>),
        Index(Box<LispNode<'a, Index<'a>>>),

        LeftShift(Box<LispNode<'a, LeftShift<'a>>>),
        RightShift(Box<LispNode<'a, RightShift<'a>>>),

        Eq(Box<LispNode<'a, Eq<'a>>>),
        NotEq(Box<LispNode<'a, NotEq<'a>>>),
        LessEq(Box<LispNode<'a, LessEq<'a>>>),
        GreaterEq(Box<LispNode<'a, GreaterEq<'a>>>),
        Less(Box<LispNode<'a, Less<'a>>>),
        Greater(Box<LispNode<'a, Greater<'a>>>),

        Call(Box<LispNode<'a, Call<'a>>>),
    }
}

impl<'a> Grammar<'a> for Option<Expression<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        match tokens.peek() {
            None => {
                let _ = tokens.next();
                Err(Error::Eof)
            }
            Some(Err(_)) => Err(tokens.next().unwrap().err().unwrap()),

            Some(Ok(Token::Lit(_))) => Ok(Some(Expression::Lit(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Ident(_))) => {
                // FIXME remove allocs
                let path = Grammar::parse(context, tokens)?;
                if !context.is_defined(&path) {
                    return Err(Error::InvalidPath { path,
                                                    reason: Some("Undefined identifier") });
                }
                Ok(Some(Expression::Path(path)))
            }

            // array
            Some(Ok(Token::LeftSquare(_))) => {
                Ok(Some(Expression::Array(Grammar::parse(context, tokens)?)))
            }

            // unary ops
            Some(Ok(Token::Minus(_))) => {
                Ok(Some(Expression::Minus(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::At(_))) => {
                Ok(Some(Expression::AddressOf(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Star(_))) => {
                Ok(Some(Expression::Deref(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::Tilde(_))) => {
                Ok(Some(Expression::Not(Grammar::parse(context, tokens)?)))
            }

            // others
            Some(Ok(Token::LeftPar(_))) => {
                let left_par = Grammar::parse(context, tokens)?;
                match tokens.peek() {
                    None => unimplemented!(),
                    Some(Err(_)) => Err(tokens.next().unwrap().err().unwrap()),

                    // arithmetic
                    Some(Ok(Token::Plus(_))) => Ok(Some(Expression::Add(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                    Some(Ok(Token::Minus(_))) => Ok(Some(Expression::Sub(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                    Some(Ok(Token::Star(_))) => Ok(Some(Expression::Mul(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                    Some(Ok(Token::Slash(_))) => Ok(Some(Expression::Div(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                    Some(Ok(Token::Ampersand(_))) => {
                        Ok(Some(Expression::And(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::Pipe(_))) => Ok(Some(Expression::Or(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                    Some(Ok(Token::Caret(_))) => Ok(Some(Expression::Xor(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),

                    // assignments
                    Some(Ok(Token::Assign(_))) => {
                        Ok(Some(Expression::Assign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::PlusAssign(_))) => {
                        Ok(Some(Expression::PlusAssign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::MinusAssign(_))) => {
                        Ok(Some(Expression::MinusAssign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::StarAssign(_))) => {
                        Ok(Some(Expression::MulAssign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::SlashAssign(_))) => {
                        Ok(Some(Expression::DivAssign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::AmpersandAssign(_))) => {
                        Ok(Some(Expression::AndAssign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::PipeAssign(_))) => {
                        Ok(Some(Expression::OrAssign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::CaretAssign(_))) => {
                        Ok(Some(Expression::XorAssign(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }

                    // indexing
                    Some(Ok(Token::LeftSquare(_))) => {
                        Ok(Some(Expression::Index(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }

                    Some(Ok(Token::LessLess(_))) => {
                        Ok(Some(Expression::LeftShift(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::GreatGreat(_))) => {
                        Ok(Some(Expression::RightShift(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::Eq(_))) => Ok(Some(Expression::Eq(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                    Some(Ok(Token::TildeEq(_))) => {
                        Ok(Some(Expression::NotEq(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::LessEq(_))) => {
                        Ok(Some(Expression::LessEq(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::GreaterEq(_))) => {
                        Ok(Some(Expression::GreaterEq(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }
                    Some(Ok(Token::Less(_))) => Ok(Some(Expression::Less(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                    Some(Ok(Token::Greater(_))) => {
                        Ok(Some(Expression::Greater(Box::new(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        }))))
                    }

                    // calls
                    Some(Ok(_)) => Ok(Some(Expression::Call(Box::new(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })))),
                }
            }
            Some(Ok(_)) => Ok(None),
        }
    }
}

parse! {
    #[derive(Debug)]
    pub struct Array<'a>
    {
        pub left_square: lex::LeftSquare<'a>,
        pub inner: Vec<Expression<'a>>,
        pub right_square: lex::RightSquare<'a>,
    }
}

impl Spanned for Array<'_> {
    fn span(&self) -> Span {
        union(&self.left_square.span(), &self.right_square.span())
    }
}

impl<'a> Grammar<'a> for Expression<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken { token,
                                         expected: None })
        }
    }
}

#[derive(Debug)]
pub struct LispNode<'a, I> {
    pub left_par: lex::LeftPar<'a>,
    pub inner: I,
    pub right_par: lex::RightPar<'a>,
}

impl<I> Spanned for LispNode<'_, I> {
    fn span(&self) -> Span {
        union(&self.left_par.span(), &self.right_par.span())
    }
}

parse! {
    /// `+ <expressions> <expressions>`
    #[derive(Debug)]
    pub struct Add<'a>
    {
        pub plus: lex::Plus<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `- <expressions> <expressions>`
    #[derive(Debug)]
    pub struct Sub<'a>
    {
        pub minus: lex::Minus<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `+ <expressions> <expressions>`
    #[derive(Debug)]
    pub struct Mul<'a>
    {
        pub star: lex::Star<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `/ <expressions> <expressions>`
    #[derive(Debug)]
    pub struct Div<'a>
    {
        pub slash: lex::Slash<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `& <expressions> <expressions>`
    #[derive(Debug)]
    pub struct And<'a>
    {
        pub ampersand: lex::Ampersand<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `| <expressions> <expressions>`
    #[derive(Debug)]
    pub struct Or<'a>
    {
        pub pipe: lex::Pipe<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `^ <expressions> <expressions>`
    #[derive(Debug)]
    pub struct Xor<'a>
    {
        pub caret: lex::Caret<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `- <expressions>`
    #[derive(Debug)]
    pub struct Minus<'a>
    {
        pub minus: lex::Minus<'a>,
        pub inner: Expression<'a>,
    }
}

impl Spanned for Minus<'_> {
    fn span(&self) -> Span {
        union(&self.minus.span(), &self.inner.span())
    }
}

parse! {
    /// `@ <expressions>`
    #[derive(Debug)]
    pub struct AddressOf<'a>
    {
        pub at: lex::At<'a>,
        pub inner: Expression<'a>,
    }
}

impl Spanned for AddressOf<'_> {
    fn span(&self) -> Span {
        union(&self.at.span(), &self.inner.span())
    }
}

parse! {
    /// `* <expressions>`
    #[derive(Debug)]
    pub struct Deref<'a>
    {
        pub star: lex::Star<'a>,
        pub inner: Expression<'a>,
    }
}

impl Spanned for Deref<'_> {
    fn span(&self) -> Span {
        union(&self.star.span(), &self.inner.span())
    }
}

parse! {
    /// `~ <expressions>`
    #[derive(Debug)]
    pub struct Not<'a>
    {
        pub tilde: lex::Tilde<'a>,
        pub inner: Expression<'a>,
    }
}

impl Spanned for Not<'_> {
    fn span(&self) -> Span {
        union(&self.tilde.span(), &self.inner.span())
    }
}

parse! {
    /// `= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct Assign<'a>
    {
        pub assign: lex::Assign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `+= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct PlusAssign<'a>
    {
        pub plus_assign: lex::PlusAssign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `-= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct MinusAssign<'a>
    {
        pub minus_assign: lex::MinusAssign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `*= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct MulAssign<'a>
    {
        pub star_assign: lex::StarAssign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `/= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct DivAssign<'a>
    {
        pub slash_assign: lex::SlashAssign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `&= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct AndAssign<'a>
    {
        pub ampersand_assign: lex::AmpersandAssign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `|= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct OrAssign<'a>
    {
        pub pipe_assign: lex::PipeAssign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `^= <expressions> <expressions>`
    #[derive(Debug)]
    pub struct XorAssign<'a>
    {
        pub caret_assign: lex::CaretAssign<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `<expressions> <args>`
    #[derive(Debug)]
    pub struct Call<'a>
    {
        pub left: Expression<'a>,
        pub args: Vec<Expression<'a>>,
    }
}

parse! {
    /// `[<expressions>] <expressions>`
    #[derive(Debug)]
    pub struct Index<'a>
    {
        pub left_square: lex::LeftSquare<'a>,
        pub left: Expression<'a>,
        pub right_square: lex::RightSquare<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `<< <left> <right>`
    #[derive(Debug)]
    pub struct LeftShift<'a>
    {
        pub less_less: lex::LessLess<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `>> <left> <right>`
    #[derive(Debug)]
    pub struct RightShift<'a>
    {
        pub great_great: lex::GreatGreat<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `== <left> <right>`
    #[derive(Debug)]
    pub struct Eq<'a>
    {
        pub eq: lex::Eq<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `~= <left> <right>`
    #[derive(Debug)]
    pub struct NotEq<'a>
    {
        pub tilde_eq: lex::TildeEq<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `<= <left> <right>`
    #[derive(Debug)]
    pub struct LessEq<'a>
    {
        pub less_eq: lex::LessEq<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `>= <left> <right>`
    #[derive(Debug)]
    pub struct GreaterEq<'a>
    {
        pub greater_eq: lex::GreaterEq<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `< <left> <right>`
    #[derive(Debug)]
    pub struct Less<'a>
    {
        pub less: lex::Less<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}

parse! {
    /// `> <left> <right>`
    #[derive(Debug)]
    pub struct Greater<'a>
    {
        pub greater: lex::Greater<'a>,
        pub left: Expression<'a>,
        pub right: Expression<'a>,
    }
}
