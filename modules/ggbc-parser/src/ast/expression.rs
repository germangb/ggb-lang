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
// etc... unary expressions aren't: @foo, *bar, -42, etc...
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
        LeftShift(Box<LispNode<'a, LeftShift<'a>>>),
        RightShift(Box<LispNode<'a, RightShift<'a>>>),
        // array index
        Index(Box<LispNode<'a, Index<'a>>>),
        // compare
        Eq(Box<LispNode<'a, Eq<'a>>>),
        NotEq(Box<LispNode<'a, NotEq<'a>>>),
        LessEq(Box<LispNode<'a, LessEq<'a>>>),
        GreaterEq(Box<LispNode<'a, GreaterEq<'a>>>),
        Less(Box<LispNode<'a, Less<'a>>>),
        Greater(Box<LispNode<'a, Greater<'a>>>),
        // functions
        Call(Box<LispNode<'a, Call<'a>>>),
    }
}

impl<'a> Grammar<'a> for Option<Expression<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        macro_rules! prefix_match_arm {
            ($var:ident, $left_par:expr) => {{
                Ok(Some(Expression::$var(Box::new(LispNode {
                    left_par: $left_par,
                    inner: Grammar::parse(context, tokens)?,
                    right_par: Grammar::parse(context, tokens)?,
                }))))
            }};
        }

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
                    // arithmetic
                    Some(Ok(Token::Plus(_))) => prefix_match_arm!(Add, left_par),
                    Some(Ok(Token::Minus(_))) => prefix_match_arm!(Sub, left_par),
                    Some(Ok(Token::Star(_))) => prefix_match_arm!(Mul, left_par),
                    Some(Ok(Token::Slash(_))) => prefix_match_arm!(Div, left_par),
                    Some(Ok(Token::Ampersand(_))) => prefix_match_arm!(And, left_par),
                    Some(Ok(Token::Pipe(_))) => prefix_match_arm!(Or, left_par),
                    Some(Ok(Token::Caret(_))) => prefix_match_arm!(Xor, left_par),
                    // assignment
                    Some(Ok(Token::Assign(_))) => prefix_match_arm!(Assign, left_par),
                    Some(Ok(Token::PlusAssign(_))) => prefix_match_arm!(PlusAssign, left_par),
                    Some(Ok(Token::MinusAssign(_))) => prefix_match_arm!(MinusAssign, left_par),
                    Some(Ok(Token::StarAssign(_))) => prefix_match_arm!(MulAssign, left_par),
                    Some(Ok(Token::SlashAssign(_))) => prefix_match_arm!(DivAssign, left_par),
                    Some(Ok(Token::AmpersandAssign(_))) => prefix_match_arm!(AndAssign, left_par),
                    Some(Ok(Token::PipeAssign(_))) => prefix_match_arm!(OrAssign, left_par),
                    Some(Ok(Token::CaretAssign(_))) => prefix_match_arm!(XorAssign, left_par),
                    // indexing
                    Some(Ok(Token::LeftSquare(_))) => prefix_match_arm!(Index, left_par),
                    // compare
                    Some(Ok(Token::LessLess(_))) => prefix_match_arm!(LeftShift, left_par),
                    Some(Ok(Token::GreatGreat(_))) => prefix_match_arm!(RightShift, left_par),
                    Some(Ok(Token::Eq(_))) => prefix_match_arm!(Eq, left_par),
                    Some(Ok(Token::TildeEq(_))) => prefix_match_arm!(NotEq, left_par),
                    Some(Ok(Token::LessEq(_))) => prefix_match_arm!(LessEq, left_par),
                    Some(Ok(Token::GreaterEq(_))) => prefix_match_arm!(GreaterEq, left_par),
                    Some(Ok(Token::Less(_))) => prefix_match_arm!(Less, left_par),
                    Some(Ok(Token::Greater(_))) => prefix_match_arm!(Greater, left_par),
                    // calls
                    Some(Ok(_)) => prefix_match_arm!(Call, left_par),
                    // fallbacks
                    // errors
                    None => unimplemented!(),
                    Some(Err(_)) => Err(tokens.next().unwrap().err().unwrap()),
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
