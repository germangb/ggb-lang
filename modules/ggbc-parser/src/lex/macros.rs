macro_rules! tokens {
    ($(
        $(#[$($meta:meta)+])*
        $token:ident,
    )+) => {
        $(
            $(#[$($meta)+])*
            #[derive(Debug, Clone)]
            pub struct $token<'a>(raw::TokenSpan<'a>);

            impl std::fmt::Display for $token<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    (self.0).0.fmt(f)
                }
            }

            impl crate::lex::span::Spanned for $token<'_> {
                fn span(&self) -> crate::lex::span::Span {
                    self.0.span()
                }
            }

            impl<'a> crate::ast::Grammar<'a> for $token<'a> {
                fn parse(
                    _: &mut crate::ast::Context<'a>,
                    tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>,
                ) -> Result<Self, crate::error::Error<'a>> {
                    match tokens.next() {
                        Some(Ok(Token::$token(token))) => Ok(token),
                        Some(Ok(token)) => Err(crate::error::Error::UnexpectedToken { token, expected: None }),
                        Some(Err(err)) => Err(err),
                        None => unimplemented!(),
                    }
                }
            }

            impl<'a> crate::ast::Grammar<'a> for Option<$token<'a>> {
                fn parse(
                    context: &mut crate::ast::Context<'a>,
                    tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>,
                ) -> Result<Self, crate::error::Error<'a>> {
                    match tokens.peek() {
                        Some(Ok(Token::$token(token))) => Ok(Some(crate::ast::Grammar::parse(context, tokens)?)),
                        None | Some(Ok(_)) => Ok(None),
                        // TODO consider to error out,
                        //  make sure it's consistent with the rest of the parsers.
                        Some(Err(error)) => Ok(None),
                    }
                }
            }
        )+

        #[derive(Debug, Clone)]
        pub enum Token<'a> {
            $($token($token<'a>),)+
        }

        impl crate::lex::span::Spanned for Token<'_> {
            fn span(&self) -> crate::lex::span::Span {
                match self {
                    $(Token::$token(var) => var.span(),)+
                }
            }
        }

        impl std::fmt::Display for Token<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Token::$token(var) => var.fmt(f),)+
                }
            }
        }
    }
}
