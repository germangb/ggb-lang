use crate::{
    ast::{expressions::Path, Context, Grammar, Separated},
    lex,
    lex::{Token, Tokens},
    Error,
};
use std::iter::Peekable;

pub enum Expression<'a> {
    Path(Path<'a>),
    Lit(lex::Lit<'a>),
    Add(LispNode<'a, Add<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    Sub(LispNode<'a, Sub<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    #[cfg(feature = "mul")]
    Mul(LispNode<'a, Mul<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    #[cfg(feature = "div")]
    Div(LispNode<'a, Div<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    And(LispNode<'a, And<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    Or(LispNode<'a, Or<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    Xor(LispNode<'a, Xor<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    // TODO
    Minus(LispNode<'a, Minus<'a, Box<Expression<'a>>>>),
    // TODO
    AddressOf(LispNode<'a, AddressOf<'a, Box<Expression<'a>>>>),
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
    Call(
        LispNode<
            'a,
            Call<
                'a,
                Box<Expression<'a>>,
                LispNode<'a, Box<Separated<Expression<'a>, lex::Comma<'a>>>>,
            >,
        >,
    ),
    Index(LispNode<'a, Index<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    // TODO
    Deref(LispNode<'a, Deref<'a, Box<Expression<'a>>>>),
    LeftShift(LispNode<'a, LeftShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
    RightShift(LispNode<'a, RightShift<'a, Box<Expression<'a>>, Box<Expression<'a>>>>),
}

impl<'a> Grammar<'a> for Expression<'a> {
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

            Some(Ok(Token::Lit(_))) => Ok(Expression::Lit(Grammar::parse(context, tokens)?)),
            Some(Ok(Token::Ident(_))) => {
                let path = Grammar::parse(context, tokens)?;
                if context.is_defined(&path) {
                    Ok(Expression::Path(path))
                } else {
                    Err(Error::UndefinedPath { path })
                }
            }
            Some(Ok(Token::LeftPar(_))) => {
                let left_par = Grammar::parse(context, tokens)?;
                match tokens.peek() {
                    None => unimplemented!(),
                    Some(Err(_)) => Err(tokens.next().unwrap().err().unwrap()),

                    // arithmetic
                    Some(Ok(Token::Plus(_))) => Ok(Expression::Add(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::Sub(_))) => Ok(Expression::Sub(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    #[cfg(feature = "mul")]
                    Some(Ok(Token::Star(_))) => Ok(Expression::Mul(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    #[cfg(feature = "div")]
                    Some(Ok(Token::Slash(_))) => Ok(Expression::Div(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::Ampersand(_))) => Ok(Expression::And(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::Pipe(_))) => Ok(Expression::Or(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::Caret(_))) => Ok(Expression::Xor(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),

                    // assignments
                    Some(Ok(Token::Assign(_))) => Ok(Expression::Assign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::PlusAssign(_))) => Ok(Expression::PlusAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::MinusAssign(_))) => Ok(Expression::MinusAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    #[cfg(feature = "mul")]
                    Some(Ok(Token::StarAssign(_))) => Ok(Expression::MulAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    #[cfg(feature = "div")]
                    Some(Ok(Token::SlashAssign(_))) => Ok(Expression::DivAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::AmpersandAssign(_))) => Ok(Expression::AndAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::PipeAssign(_))) => Ok(Expression::OrAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                    Some(Ok(Token::CaretAssign(_))) => Ok(Expression::XorAssign(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),

                    // indexing
                    Some(Ok(Token::LeftSquare(_))) => Ok(Expression::Index(LispNode {
                        left_par,
                        inner: Grammar::parse(context, tokens)?,
                        right_par: Grammar::parse(context, tokens)?,
                    })),

                    // calls
                    Some(Ok(_)) => Ok(Expression::Call(LispNode {
                        left_par,
                        //inner: Grammar::parse(context, tokens)?,
                        inner: unimplemented!(),
                        right_par: Grammar::parse(context, tokens)?,
                    })),
                }
            }
            Some(Ok(_)) => {
                let token = tokens.next().unwrap()?;
                Err(Error::UnexpectedToken {
                    token,
                    expected: None,
                })
            }
        }
    }
}

pub struct LispNode<'a, I> {
    pub left_par: lex::LeftPar<'a>,
    pub inner: I,
    pub right_par: lex::RightPar<'a>,
}

parse! {
    /// `+ <expr> <expr>`
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
    /// `= <expr> <expr>`
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
    /// `[] <expr> <expr>`
    pub struct Index<'a, L, R>
    where
        L: Grammar<'a>,
        R: Grammar<'a>,
    {
        pub left_square: lex::LeftSquare<'a>,
        pub right_square: lex::RightSquare<'a>,
        pub left: L,
        pub right: R,
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
    /// `<< <left> <right>`
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
