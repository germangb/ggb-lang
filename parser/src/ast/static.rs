use crate::{
    ast::{Context, Expression, Field, Grammar},
    error::Error,
    lex,
    lex::{
        span,
        span::{Span, Spanned},
        Token,
    },
    Tokens,
};
use std::iter::Peekable;

parse! {
    #[derive(Debug)]
    pub struct StaticOffset<'a> {
        /// `@` token.
        pub at: lex::At<'a>,

        /// Offset expression.
        pub expression: Expression<'a>,
    }
}

impl<'a> Grammar<'a> for Option<StaticOffset<'a>> {
    fn parse(context: &mut Context<'a>,
             tokens: &mut Peekable<Tokens<'a>>)
             -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::At(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

parse! {
    #[derive(Debug)]
    pub struct Static<'a> {
        /// `static` token.
        pub static_: lex::Static<'a>,

        /// Optional [`StaticOffset`](StaticOffset) tokens.
        pub offset: Option<StaticOffset<'a>>,

        /// [`Field`](Field) tokens.
        pub field: Field<'a>,
    }
}

impl Spanned for Static<'_> {
    fn span(&self) -> Span {
        span::union(&self.static_.span(), &self.field.span())
    }
}
