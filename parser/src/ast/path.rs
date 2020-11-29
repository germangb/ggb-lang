use crate::{
    ast::{Context, Grammar},
    error::Error,
    lex,
    lex::{
        span,
        span::{Span, Spanned},
    },
    Tokens,
};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Path<'a> {
    /// Head identifier token.
    pub head: lex::Ident<'a>,

    /// Rest of the tokens.
    pub tail: Vec<(lex::Square<'a>, lex::Ident<'a>)>,
}

impl Spanned for Path<'_> {
    fn span(&self) -> Span {
        let mut span = self.head.span();
        if let Some((_, t)) = self.tail.last() {
            span = span::union(&span, &t.span());
        }
        span
    }
}

impl Path<'_> {
    /// Returns an ordered iterator over the separated items of type `T`.
    pub fn iter(&self) -> impl Iterator<Item = &lex::Ident<'_>> {
        let tail_iter = self.tail.iter().map(|(_, ref t)| t);
        Some(&self.head).into_iter().chain(tail_iter)
    }

    /// Returns the total number of items of type `T`.
    pub fn len(&self) -> usize {
        self.tail.len() + 1
    }

    /// Returns whether the path has no items in it.
    /// Equivalent to `Self::len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> Grammar<'a> for Path<'a> {
    fn parse(
        context: &mut Context<'a>,
        tokens: &mut Peekable<Tokens<'a>>,
    ) -> Result<Self, Error<'a>> {
        let head = Grammar::parse(context, tokens)?;
        let mut tail = Vec::new();
        while let Some(sep) = Grammar::parse(context, tokens)? {
            let item = Grammar::parse(context, tokens)?;
            tail.push((sep, item))
        }
        Ok(Self { head, tail })
    }
}
