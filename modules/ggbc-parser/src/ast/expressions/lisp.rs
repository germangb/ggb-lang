use crate::{
    ast::{expressions::Path, Context, Grammar, Separated},
    lex,
    lex::{Token, Tokens},
    span::{union, Span, Spanned},
    Error,
};
use std::iter::Peekable;

// binary expressions are surrounded by parenthesis: (+ 1 2), (== foo 42),
// etc... unary expressions don't: @foo, *bar, -42, etc...
parse_enum! {
    #[derive(Debug)]
    pub enum Expression<'a> {
        // terminals
        Path(Path<'a>),
        Lit(lex::Lit<'a>),

        // unary
        Minus(Minus<'a, Box<Expression<'a>>>),
        AddressOf(AddressOf<'a, Box<Expression<'a>>>),
        Deref(Deref<'a, Box<Expression<'a>>>),
        Not(Not<'a, Box<Expression<'a>>>),

        // binary
        Add(LispNode<'a, Add<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        Sub(LispNode<'a, Sub<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        #[cfg(feature = "mul")]
        Mul(LispNode<'a, Mul<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        #[cfg(feature = "div")]
        Div(LispNode<'a, Div<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        And(LispNode<'a, And<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        Or(LispNode<'a, Or<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        Xor(LispNode<'a, Xor<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        Assign(LispNode<'a, Assign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        PlusAssign(LispNode<'a, PlusAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        MinusAssign(LispNode<'a, MinusAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        #[cfg(feature = "mul")]
        MulAssign(LispNode<'a, MulAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        #[cfg(feature = "div")]
        DivAssign(LispNode<'a, DivAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        AndAssign(LispNode<'a, AndAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        OrAssign(LispNode<'a, OrAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        XorAssign(LispNode<'a, XorAssign<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        Index(LispNode<'a, Index<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),

        LeftShift(LispNode<'a, LeftShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        RightShift(LispNode<'a, RightShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),

        Eq(LispNode<'a, Eq<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        NotEq(LispNode<'a, NotEq<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        LessEq(LispNode<'a, LessEq<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        GreaterEq(LispNode<'a, GreaterEq<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        Less(LispNode<'a, Less<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
        Greater(LispNode<'a, Greater<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),

        Call(LispNode<'a, Call<'a, Box<Expression<'a>>, Vec<Expression<'a>>>>),
    }
}

impl<'a> Grammar<'a> for Option<Expression<'a>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            None => {
                let _ = tokens.next();
                Err(Error::Eof)
            }
            Some(Err(_)) => Err(tokens.next().unwrap().err().unwrap()),

            Some(Ok(Token::Lit(_))) => Ok(Some(Expression::Lit(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::Ident(_))) => {
                let path = Grammar::parse(context, tokens)?;
                if context.is_defined(&path) {
                    Ok(Some(Expression::Path(path)))
                } else {
                    Err(Error::InvalidPath { path })
                }
            }
            // unary ops
            Some(Ok(Token::Minus(_))) => {
                Ok(Some(Expression::Minus(Grammar::parse(context, tokens)?)))
            }
            Some(Ok(Token::At(_))) => Ok(Some(Expression::AddressOf(Grammar::parse(
                context, tokens,
            )?))),
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
                    Some(Ok(Token::Plus(_))) => Ok(Some(Expression::Add(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::Sub(_))) => Ok(Some(Expression::Sub(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    #[cfg(feature = "mul")]
                    Some(Ok(Token::Star(_))) => Ok(Some(Expression::Mul(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    #[cfg(feature = "div")]
                    Some(Ok(Token::Slash(_))) => Ok(Some(Expression::Div(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::Ampersand(_))) => Ok(Some(Expression::And(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::Pipe(_))) => Ok(Some(Expression::Or(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::Caret(_))) => Ok(Some(Expression::Xor(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),

                    // assignments
                    Some(Ok(Token::Assign(_))) => Ok(Some(Expression::Assign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::PlusAssign(_))) => Ok(Some(Expression::PlusAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::MinusAssign(_))) => {
                        Ok(Some(Expression::MinusAssign(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        })))
                    }
                    #[cfg(feature = "mul")]
                    Some(Ok(Token::StarAssign(_))) => Ok(Some(Expression::MulAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    #[cfg(feature = "div")]
                    Some(Ok(Token::SlashAssign(_))) => Ok(Some(Expression::DivAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::AmpersandAssign(_))) => {
                        Ok(Some(Expression::AndAssign(LispNode {
                            left_par,
                            inner: Grammar::parse(context, tokens)?,
                            right_par: Grammar::parse(context, tokens)?,
                        })))
                    }
                    Some(Ok(Token::PipeAssign(_))) => Ok(Some(Expression::OrAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::CaretAssign(_))) => Ok(Some(Expression::XorAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),

                    // indexing
                    Some(Ok(Token::LeftSquare(_))) => Ok(Some(Expression::Index(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),

                    Some(Ok(Token::LessLess(_))) => Ok(Some(Expression::LeftShift(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::GreatGreat(_))) => Ok(Some(Expression::RightShift(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::Eq(_))) => Ok(Some(Expression::Eq(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::TildeEq(_))) => Ok(Some(Expression::NotEq(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::LessEq(_))) => Ok(Some(Expression::LessEq(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::GreaterEq(_))) => Ok(Some(Expression::GreaterEq(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::Less(_))) => Ok(Some(Expression::Less(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                    Some(Ok(Token::Greater(_))) => Ok(Some(Expression::Greater(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),

                    // calls
                    Some(Ok(_)) => Ok(Some(Expression::Call(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    }))),
                }
            }
            Some(Ok(_)) => Ok(None),
        }
    }
}

impl<'a> Grammar<'a> for Expression<'a> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            // TODO error reporting
            let token = tokens.next().expect("Token please")?;
            Err(Error::UnexpectedToken {
                token,
                expected: None,
            })
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
    /// `+ <expr> <expr>`
    #[derive(Debug)]
    pub struct Add<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub plus: lex::Plus<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `- <expr> <expr>`
    #[derive(Debug)]
    pub struct Sub<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub minus: lex::Minus<'a>,
        pub left: L,
        pub right: R,
    }
}

#[cfg(feature = "mul")]
parse! {
    /// `+ <expr> <expr>`
    #[derive(Debug)]
    pub struct Mul<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub star: lex::Star<'a>,
        pub left: L,
        pub right: R,
    }
}

#[cfg(feature = "div")]
parse! {
    /// `/ <expr> <expr>`
    #[derive(Debug)]
    pub struct Div<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub slash: lex::Slash<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `& <expr> <expr>`
    #[derive(Debug)]
    pub struct And<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub ampersand: lex::Ampersand<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `| <expr> <expr>`
    #[derive(Debug)]
    pub struct Or<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub pipe: lex::Pipe<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `^ <expr> <expr>`
    #[derive(Debug)]
    pub struct Xor<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub caret: lex::Caret<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `- <expr>`
    #[derive(Debug)]
    pub struct Minus<'a, E>
    where
        E: Grammar<'a>,
    {
        pub minus: lex::Minus<'a>,
        pub inner: E,
    }
}

impl<E: Spanned> Spanned for Minus<'_, E> {
    fn span(&self) -> Span {
        union(&self.minus.span(), &self.inner.span())
    }
}

parse! {
    /// `@ <expr>`
    #[derive(Debug)]
    pub struct AddressOf<'a, E>
    where
        E: Grammar<'a>,
    {
        pub at: lex::At<'a>,
        pub inner: E,
    }
}

impl<E: Spanned> Spanned for AddressOf<'_, E> {
    fn span(&self) -> Span {
        union(&self.at.span(), &self.inner.span())
    }
}

parse! {
    /// `* <expr>`
    #[derive(Debug)]
    pub struct Deref<'a, E>
    where
        E: Grammar<'a>,
    {
        pub star: lex::Star<'a>,
        pub inner: E,
    }
}

impl<E: Spanned> Spanned for Deref<'_, E> {
    fn span(&self) -> Span {
        union(&self.star.span(), &self.inner.span())
    }
}

parse! {
    /// `~ <expr>`
    #[derive(Debug)]
    pub struct Not<'a, E>
    where
        E: Grammar<'a>,
    {
        pub tilde: lex::Tilde<'a>,
        pub inner: E,
    }
}

impl<E: Spanned> Spanned for Not<'_, E> {
    fn span(&self) -> Span {
        union(&self.tilde.span(), &self.inner.span())
    }
}

parse! {
    /// `= <expr> <expr>`
    #[derive(Debug)]
    pub struct Assign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub assign: lex::Assign<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `+= <expr> <expr>`
    #[derive(Debug)]
    pub struct PlusAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub plus_assign: lex::PlusAssign<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `-= <expr> <expr>`
    #[derive(Debug)]
    pub struct MinusAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub minus_assign: lex::MinusAssign<'a>,
        pub left: L,
        pub right: R,
    }
}

#[cfg(feature = "mul")]
parse! {
    /// `*= <expr> <expr>`
    #[derive(Debug)]
    pub struct MulAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub star_assign: lex::StarAssign<'a>,
        pub left: L,
        pub right: R,
    }
}

#[cfg(feature = "div")]
parse! {
    /// `/= <expr> <expr>`
    #[derive(Debug)]
    pub struct DivAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub slash_assign: lex::SlashAssign<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `&= <expr> <expr>`
    #[derive(Debug)]
    pub struct AndAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub ampersand_assign: lex::AmpersandAssign<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `|= <expr> <expr>`
    #[derive(Debug)]
    pub struct OrAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub pipe_assign: lex::PipeAssign<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `^= <expr> <expr>`
    #[derive(Debug)]
    pub struct XorAssign<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub caret_assign: lex::CaretAssign<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `<expr> <args>`
    #[derive(Debug)]
    pub struct Call<'a, F, A>
    where
        F: Grammar<'a>,
        A: Grammar<'a>,
    {
        pub left: F,
        pub args: A,
    }

    // phantom data markers
    {
        _phantom: std::marker::PhantomData<&'a ()>,
    }
}

parse! {
    /// `[<expr>] <expr>`
    #[derive(Debug)]
    pub struct Index<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left_square: lex::LeftSquare<'a>,
        pub right: R,
        pub right_square: lex::RightSquare<'a>,
        pub left: L,
    }
}

parse! {
    /// `<< <left> <right>`
    #[derive(Debug)]
    pub struct LeftShift<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub less_less: lex::LessLess<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `>> <left> <right>`
    #[derive(Debug)]
    pub struct RightShift<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub great_great: lex::GreatGreat<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `== <left> <right>`
    #[derive(Debug)]
    pub struct Eq<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub eq: lex::Eq<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `~= <left> <right>`
    #[derive(Debug)]
    pub struct NotEq<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub tilde_eq: lex::TildeEq<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `<= <left> <right>`
    #[derive(Debug)]
    pub struct LessEq<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub less_eq: lex::LessEq<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `>= <left> <right>`
    #[derive(Debug)]
    pub struct GreaterEq<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub greater_eq: lex::GreaterEq<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `< <left> <right>`
    #[derive(Debug)]
    pub struct Less<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub less: lex::Less<'a>,
        pub left: L,
        pub right: R,
    }
}

parse! {
    /// `> <left> <right>`
    #[derive(Debug)]
    pub struct Greater<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub greater: lex::Greater<'a>,
        pub left: L,
        pub right: R,
    }
}
