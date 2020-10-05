use crate::{
    ast::{Context, Field, FieldGroup, Grammar, Separated, Type},
    lex,
    lex::{Token, Tokens},
    Error,
};
use std::iter::Peekable;

// FIXME
//  fuck this, I mean fix it.
//  Generics are fucked somewhere in this typed mess...
//  I'd suggest removing the "Option<T>: Grammar<'_>" bounds.
impl<'a> Grammar<'a> for Option<Separated<FieldGroup<'a, Type<'a>>, lex::Comma<'a>>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}
impl<'a> Grammar<'a> for Option<Separated<FieldGroup<'a, Box<Type<'a>>>, lex::Comma<'a>>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}

impl<'a> Grammar<'a> for Option<Separated<Field<'a, Type<'a>>, lex::Comma<'a>>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}
impl<'a> Grammar<'a> for Option<Separated<Field<'a, Box<Type<'a>>>, lex::Comma<'a>>> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        if let Some(Ok(Token::Ident(_))) = tokens.peek() {
            Ok(Some(Grammar::parse(context, tokens)?))
        } else {
            Ok(None)
        }
    }
}
