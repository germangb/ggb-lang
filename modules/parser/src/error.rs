use crate::{lex, lex::span::Span};

/// `crate` errors.
#[derive(Debug)]
pub enum Error<'a> {
    Lex(LexError),
    Ast(AstError<'a>),
}

/// `lex` errors.
#[derive(Debug)]
pub enum LexError {
    UnexpectedByte { byte: u8, span: Span },
}

/// `ast` errors.
#[derive(Debug)]
pub enum AstError<'a> {
    UnexpectedToken(lex::Token<'a>),
    UndefinedIdent(lex::Ident<'a>),
}
