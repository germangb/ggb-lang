//! Data type grammars.
use crate::{
    ast::{expression::Expression, Context, Field, Grammar, Path},
    lex,
    lex::{Token, Tokens},
    Error,
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
    fn parse(ctx: &mut Context<'a>, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        let type_ = match tokens.peek() {
            Some(Err(_)) => return Err(tokens.next().unwrap().err().unwrap()),
            Some(Ok(Token::U8(_))) => Type::U8(Grammar::parse(ctx, tokens)?),
            Some(Ok(Token::I8(_))) => Type::I8(Grammar::parse(ctx, tokens)?),
            Some(Ok(Token::LeftSquare(_))) => Type::Array(Grammar::parse(ctx, tokens)?),
            Some(Ok(Token::Struct(_))) => Type::Struct(Grammar::parse(ctx, tokens)?),
            Some(Ok(Token::Union(_))) => Type::Union(Grammar::parse(ctx, tokens)?),
            Some(Ok(Token::Ampersand(_))) => Type::Pointer(Grammar::parse(ctx, tokens)?),
            Some(Ok(Token::Ident(_))) => {
                let path = Grammar::parse(ctx, tokens)?;
                if !ctx.is_type(&path) {
                    return Err(Error::InvalidPath(path));
                }
                Type::Path(path)
            }
            _ => return Ok(None),
        };

        Ok(Some(type_))
    }
}

impl<'a> Grammar<'a> for Type<'a> {
    fn parse(ctx: &mut Context<'a>, tokens: &mut Peekable<Tokens<'a>>) -> Result<Self, Error<'a>> {
        if let Some(statement) = Grammar::parse(ctx, tokens)? {
            Ok(statement)
        } else {
            Err(Error::UnexpectedToken(tokens.next().unwrap()?))
        }
    }
}

span!(Struct {
    struct_,
    right_bracket
});
span!(Union {
    union,
    right_bracket
});
span!(Array {
    left_square,
    right_square
});
span!(Pointer { ampersand, type_ });

parse! {
    #[derive(Debug)]
    pub struct Struct<'a> {
        /// `struct` token.
        pub struct_: lex::Struct<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner fields.
        pub fields: Vec<Field<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

parse! {
    #[derive(Debug)]
    pub struct Union<'a> {
        /// `union` token.
        pub union: lex::Union<'a>,

        /// `{` token.
        pub left_bracket: lex::LeftBracket<'a>,

        /// Inner fields.
        pub fields: Vec<Field<'a>>,

        /// `}` token.
        pub right_bracket: lex::RightBracket<'a>,
    }
}

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
