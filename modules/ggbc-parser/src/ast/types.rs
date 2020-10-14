//! Data type grammars.
use crate::{
    ast::{expressions::Expression, Context, FnArg, FnReturn, Grammar, Path},
    error::Error,
    lex,
    lex::{Token, Tokens},
    span::{union, Span, Spanned},
};
use std::iter::Peekable;

pub type Struct<'a> = crate::ast::Struct<'a, ()>;
pub type Union<'a> = crate::ast::Union<'a, ()>;

parse_enum! {
    #[derive(Debug)]
    pub enum Type<'a> {
        U8(lex::U8<'a>),
        I8(lex::I8<'a>),
        Array(Box<Array<'a>>),
        Struct(Struct<'a>),
        Union(Union<'a>),
        Pointer(Box<Pointer<'a>>),
        Path(Path<'a>),
        Fn(Box<Fn<'a>>),
    }
}

parse! {
    #[derive(Debug)]
    pub struct Fn<'a> {
        pub fn_: lex::Fn<'a>,
        pub fn_arg: Option<FnArg<'a, Vec<Type<'a>>>>,
        pub fn_ret: Option<FnReturn<'a>>,
    }
}

impl Spanned for Fn<'_> {
    fn span(&self) -> Span {
        let mut span = self.fn_.span();
        if let Some(fn_args) = &self.fn_arg {
            span = union(&span, &fn_args.span())
        }
        if let Some(fn_ret) = &self.fn_ret {
            span = union(&span, &fn_ret.span())
        }
        span
    }
}

parse! {
    /// `& <type>`
    #[derive(Debug)]
    pub struct Pointer<'a> {
        pub ampersand: lex::Ampersand<'a>,
        pub type_: Type<'a>,
    }
}

impl Spanned for Pointer<'_> {
    fn span(&self) -> Span {
        union(&self.ampersand.span(), &self.type_.span())
    }
}

parse! {
    /// `[ <type> ; <length> ]`
    #[derive(Debug)]
    pub struct Array<'a> {
        pub left_square: lex::LeftSquare<'a>,
        pub type_: Type<'a>,
        pub semi_colon: Option<lex::SemiColon<'a>>,
        pub len: Expression<'a>,
        pub right_square: lex::RightSquare<'a>,
    }
}

impl Spanned for Array<'_> {
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
            Some(Ok(Token::Ident(_))) => {
                let path = Grammar::parse(context, tokens)?;
                if !context.is_type(&path) {
                    return Err(Error::InvalidPath {
                        path,
                        reason: Some("Undefined type"),
                    });
                }
                Ok(Some(Type::Path(path)))
            }
            Some(Ok(Token::Fn(_))) => Ok(Some(Type::Fn(Grammar::parse(context, tokens)?))),
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
