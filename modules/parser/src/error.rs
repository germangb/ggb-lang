//! Errors and multiple error reporting.
use crate::{lex, span::Span};
use std::ops::Deref;

#[derive(Debug)]
pub struct Errors<'a>(Vec<Error<'a>>);

impl<'a> Errors<'a> {
    pub(crate) fn push(&mut self, error: Error<'a>) {
        self.0.push(error);
    }
}

impl<'a> Deref for Errors<'a> {
    type Target = [Error<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

#[derive(Debug)]
pub enum Error<'a> {
    // lex
    UnexpectedByte { byte: u8, span: Span },
    // ast
    UnexpectedToken(lex::Token<'a>),
    UndefinedIdent(lex::Ident<'a>),
}
