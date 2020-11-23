//! Errors and multiple error reporting.
use crate::{ast, lex, lex::span::Span};

#[derive(Debug)]
pub enum Error<'a> {
    Eof,

    /// Reserved word used.
    ReservedKeyword {
        /// The keyword itself.
        key_word: &'a str,

        /// Location of the keyword in the programs source.
        span: Span,
    },

    /// Unexpected byte in the programs source code.
    UnexpectedByte {
        /// The unexpected byte.
        byte: u8,

        /// Location of the byte in the programs source code.
        span: Span,
    },

    /// Unexpected token while parsing.
    UnexpectedToken {
        /// Token not expected by the grammar.
        token: lex::Token<'a>,
    },

    /// Used an undefined path.
    InvalidPath {
        /// The path itself, which is invalid.
        path: ast::Path<'a>,
    },

    /// Illegal shadowing.
    ShadowIdent {
        /// An already defined and previously validated identifier.
        ident: lex::Ident<'a>,

        /// The new identifier shadowing the one above.
        shadow: lex::Ident<'a>,
    },

    /// Use of a forbidden identifier-
    ForbiddenIdent {
        /// The identifier itself.
        ident: lex::Ident<'a>,
    },
}
