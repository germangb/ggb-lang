//! Errors and multiple error reporting.
use crate::{ast, lex, span::Span};
use std::borrow::Cow;

#[derive(Debug)]
pub enum Error<'a> {
    ReservedKeyword {
        key_word: Cow<'a, str>,
        span: Span,
    },
    /// Encountered an unexpected byte in the input source code.
    UnexpectedByte {
        /// The unexpected byte.
        byte: u8,
        /// Location of the byte in the input source code.
        span: Span,
    },
    // ast
    UnexpectedToken {
        /// Token not expected by the grammar.
        token: lex::Token<'a>,
        /// Expected token (if applicable).
        expected: Option<lex::Token<'static>>,
    },
    UndefinedPath {
        /// Full path (relative to the scope where the error was raised).
        path: ast::expressions::Path<'a>,
    },
    Eof,
}
