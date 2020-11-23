//! Data type grammars.
use crate::{
    ast::{expression::Expression, Context, Grammar, Path, Struct, Union},
    error::Error,
    lex,
    lex::{Token, Tokens},
};
use std::iter::Peekable;

parse! {
    #[derive(Debug)]
    pub enum Type<'a> {
        /// u8 type.
        U8(lex::U8<'a>),

        /// i8 type.
        I8(lex::I8<'a>),

        /// Array type.
        Array(Box<Array<'a>>),

        /// Struct type.
        Struct(Struct<'a>),

        /// Union type.
        Union(Union<'a>),

        /// Pointer type.
        Pointer(Box<Pointer<'a>>),

        /// Path type.
        Path(Path<'a>),
    }
}

impl<'a> Grammar<'a> for Option<Type<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
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
            Some(Ok(Token::Ident(_))) => {
                let path = Grammar::parse(context, tokens)?;
                if !context.is_type(&path) {
                    return Err(Error::InvalidPath { path });
                }
                Ok(Some(Type::Path(path)))
            }
            #[cfg(todo)]
            Some(Ok(Token::Fn(_))) => Ok(Some(Type::Fn(Grammar::parse(context, tokens)?))),
            _ => Ok(None),
        }
    }
}

impl<'a> Grammar<'a> for Type<'a> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(context, tokens)? {
            Ok(statement)
        } else {
            Err(Error::UnexpectedToken { token: tokens.next().unwrap()? })
        }
    }
}

span!(Array { left_square,
              right_square });
span!(Pointer { ampersand, type_ });

parse! {
    /// `& <type>`
    #[derive(Debug)]
    pub struct Pointer<'a> {
        /// `&` token.
        pub ampersand: lex::Ampersand<'a>,

        /// Pointer type tokens.
        pub type_: Type<'a>,
    }
}

parse! {
    /// `[ <type> ; <length> ]`
    #[derive(Debug)]
    pub struct Array<'a> {
        /// `[` token.
        pub left_square: lex::LeftSquare<'a>,

        /// Array inner type tokens.
        pub type_: Type<'a>,

        /// `;` token.
        pub semi_colon: Option<lex::SemiColon<'a>>,

        /// Array length expression tokens.
        pub len: Expression<'a>,

        /// `]` token.
        pub right_square: lex::RightSquare<'a>,
    }
}