//! Type grammars.
use crate::{
    ast::{Context, Expression, Grammar, Struct, Union},
    lex,
    lex::{Token, Tokens},
    span::{union, Span, Spanned},
    Error,
};
use std::iter::Peekable;

parse_enum! {
    pub enum Type<'a> {
        U8(lex::U8<'a>),
        I8(lex::I8<'a>),
        Array(Array<'a, Box<Type<'a>>, Expression<'a>>),
        Struct(Box<Struct<'a, ()>>),
        Union(Box<Union<'a, ()>>),
        Pointer(Pointer<'a, Box<Type<'a>>>),
        Ident(lex::Ident<'a>),
        // TODO function type
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

impl<T: Spanned> Spanned for Pointer<'_, T> {
    fn span(&self) -> Span {
        union(&self.ampersand.span(), &self.type_.span())
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
        pub semi_colon: Option<lex::SemiColon<'a>>,
        pub len: E,
        pub right_square: lex::RightSquare<'a>,
    }
}

impl<T, E> Spanned for Array<'_, T, E> {
    fn span(&self) -> Span {
        union(&self.left_square.span(), &self.right_square.span())
    }
}

impl<'a> Grammar<'a> for Option<Type<'a>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        match tokens.peek() {
            Some(Err(_)) => {
                let err = tokens.next().unwrap().err().unwrap();
                Err(err)
            }
            Some(Ok(Token::U8(_))) => Ok(Some(Type::U8(Grammar::parse(context, tokens)?))),
            Some(Ok(Token::I8(_))) => Ok(Some(Type::I8(Grammar::parse(context, tokens)?))),
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
