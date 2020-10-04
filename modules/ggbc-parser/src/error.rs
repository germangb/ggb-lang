//! Errors and multiple error reporting.
use crate::{ast::Context, lex, span::Span};

#[derive(Debug)]
pub enum Error<'a> {
    // lex
    UnexpectedByte { byte: u8, span: Span },
    // ast
    UnexpectedToken(lex::Token<'a>),
    UndefinedIdent(lex::Ident<'a>),
}
