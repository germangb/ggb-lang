use crate::{lex, span::Span};

#[derive(Debug)]
pub enum Error<'a> {
    // lex
    UnexpectedByte { byte: u8, span: Span },
    // ast
    UnexpectedToken(lex::Token<'a>),
    UndefinedIdent(lex::Ident<'a>),
    // ir
}

// /// `lex` errors.
// #[derive(Debug)]
// pub enum LexError {
//     UnexpectedByte { byte: u8, span: Span },
// }
//
// /// `ast` errors.
// #[derive(Debug)]
// pub enum AstError<'a> {
//     UnexpectedToken(lex::Token<'a>),
//     UndefinedIdent(lex::Ident<'a>),
// }
