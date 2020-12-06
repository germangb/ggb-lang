//! Expression grammars.
use crate::{
    ast::{Context, Grammar, Path},
    lex,
    lex::{Token, Tokens},
    Error,
};
use std::iter::Peekable;

parse! {
    #[derive(Debug)]
    pub enum Expression<'a> {
        Path(Path<'a>),
        Lit(lex::Lit<'a>),
        Array(Array<'a>),
        Minus(Box<Minus<'a>>),
        AddressOf(Box<AddressOf<'a>>),
        Deref(Box<Deref<'a>>),
        Not(Box<Not<'a>>),
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
        Index(Box<LispNode<'a, Index<'a>>>),
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
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        macro_rules! prefix_match_arm {
            ($var:ident, $left_par:expr) => {{
                Expression::$var(Box::new(LispNode {
                    left_par: $left_par,
                    inner: Grammar::parse(context, tokens)?,
                    right_par: Grammar::parse(context, tokens)?,
                }))
            }};
        }

        let expression = match tokens.peek() {
            None => {
                let _ = tokens.next();
                return Err(Error::Eof);
            }
            Some(Err(_)) => return Err(tokens.next().unwrap().err().unwrap()),

            Some(Ok(Token::Lit(_))) => Expression::Lit(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Ident(_))) => {
                let path = Grammar::parse(context, tokens)?;
                if !context.is_defined(&path) {
                    return Err(Error::InvalidPath(path));
                }
                Expression::Path(path)
            }
            // array
            Some(Ok(Token::LeftSquare(_))) => Expression::Array(Grammar::parse(context, tokens)?),
            // unary ops
            Some(Ok(Token::Minus(_))) => Expression::Minus(Grammar::parse(context, tokens)?),
            Some(Ok(Token::At(_))) => Expression::AddressOf(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Star(_))) => Expression::Deref(Grammar::parse(context, tokens)?),
            Some(Ok(Token::Tilde(_))) => Expression::Not(Grammar::parse(context, tokens)?),

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
                    Some(Err(_)) => return Err(tokens.next().unwrap().err().unwrap()),
                }
            }
            Some(Ok(_)) => return Ok(None),
        };

        Ok(Some(expression))
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
            let token = tokens.next().unwrap()?;
            Err(Error::UnexpectedToken(token))
        }
    }
}

span!(Array {
    left_square,
    right_square
});
span!(LispNode<I> {
    left_par,
    right_par
});
span!(Add { plus, right });
span!(Sub { minus, right });
span!(Mul { star, right });
span!(Div { slash, right });
span!(And { ampersand, right });
span!(Or { pipe, right });
span!(Xor { caret, right });
span!(Minus { minus, inner });
span!(AddressOf { at, inner });
span!(Deref { star, inner });
span!(Not { tilde, inner });
span!(Assign { assign, right });
span!(PlusAssign { plus_assign, right });
span!(MinusAssign {
    minus_assign,
    right
});
span!(MulAssign { star_assign, right });
span!(DivAssign {
    slash_assign,
    right
});
span!(AndAssign {
    ampersand_assign,
    right
});
span!(OrAssign { pipe_assign, right });
span!(XorAssign {
    caret_assign,
    right
});
span!(Call { left });
span!(Index { left_square, right });
span!(LeftShift { less_less, right });
span!(RightShift { great_great, right });
span!(Eq { eq, right });
span!(NotEq { tilde_eq, right });
span!(LessEq { less_eq, right });
span!(GreaterEq { greater_eq, right });
span!(Less { less, right });
span!(Greater { greater, right });

#[derive(Debug)]
pub struct LispNode<'a, I> {
    pub left_par: lex::LeftPar<'a>,
    pub inner: I,
    pub right_par: lex::RightPar<'a>,
}

parse! {
    #[derive(Debug)]
    pub struct Array<'a>
    {
        /// `[` token.
        pub left_square: lex::LeftSquare<'a>,

        /// Inner expression tokens.
        pub inner: Vec<Expression<'a>>,

        /// `]` token.
        pub right_square: lex::RightSquare<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Add<'a> {
        /// `+` token.
        pub plus: lex::Plus<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Sub<'a> {
        /// `-` token.
        pub minus: lex::Minus<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Mul<'a> {
        /// `*` token.
        pub star: lex::Star<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Div<'a> {
        /// `/` token.
        pub slash: lex::Slash<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct And<'a> {
        /// `&` token.
        pub ampersand: lex::Ampersand<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Or<'a> {
        /// `|` token.
        pub pipe: lex::Pipe<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Xor<'a> {
        /// `^` token.
        pub caret: lex::Caret<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Minus<'a> {
        pub minus: lex::Minus<'a>,
        pub inner: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct AddressOf<'a> {
        pub at: lex::At<'a>,
        pub inner: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Deref<'a> {
        pub star: lex::Star<'a>,
        pub inner: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Not<'a> {
        pub tilde: lex::Tilde<'a>,
        pub inner: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Assign<'a> {
        /// `=` token.
        pub assign: lex::Assign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct PlusAssign<'a> {
        /// `+=` token.
        pub plus_assign: lex::PlusAssign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct MinusAssign<'a> {
        /// `-=` token.
        pub minus_assign: lex::MinusAssign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct MulAssign<'a> {
        /// `*=` token.
        pub star_assign: lex::StarAssign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct DivAssign<'a> {
        /// `/=` token.
        pub slash_assign: lex::SlashAssign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct AndAssign<'a> {
        /// `&=` token.
        pub ampersand_assign: lex::AmpersandAssign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct OrAssign<'a> {
        /// `|=` token.
        pub pipe_assign: lex::PipeAssign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct XorAssign<'a> {
        /// `^=` token.
        pub caret_assign: lex::CaretAssign<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Call<'a> {
        pub left: Expression<'a>,
        pub args: Vec<Expression<'a>>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Index<'a> {
        /// `[` token.
        pub left_square: lex::LeftSquare<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// `]` token.
        pub right_square: lex::RightSquare<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct LeftShift<'a> {
        /// `<<` token.
        pub less_less: lex::LessLess<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct RightShift<'a> {
        /// `>>` token.
        pub great_great: lex::GreatGreat<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Eq<'a> {
        /// `==` token.
        pub eq: lex::Eq<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct NotEq<'a> {
        /// `~=` token.
        pub tilde_eq: lex::TildeEq<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct LessEq<'a> {
        /// `<=` token.
        pub less_eq: lex::LessEq<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct GreaterEq<'a> {
        /// `>=` token.
        pub greater_eq: lex::GreaterEq<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Less<'a> {
        /// `<` token.
        pub less: lex::Less<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Greater<'a> {
        /// `>` token.
        pub greater: lex::Greater<'a>,

        /// lhs expression tokens.
        pub left: Expression<'a>,

        /// rhs expression tokens.
        pub right: Expression<'a>,
    }
}
