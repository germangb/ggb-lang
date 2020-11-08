//! Errors and multiple error reporting.
use crate::{ast, lex, lex::span::Span};

#[derive(Debug)]
pub enum Error<'a> {
    Eof,
    /// Error signaling that a reserved keyword has been found.
    /// Reserved keyword may be used in future revisions of the language.
    ReservedKeyword {
        /// The keyword itself.
        key_word: &'a str,
        /// Location of the keyword in the programs source.
        span: Span,
    },
    /// Encountered an unexpected byte in the programs source code.
    UnexpectedByte {
        /// The unexpected byte.
        byte: u8,
        /// Location of the byte in the programs source code.
        span: Span,
    },
    /// Found an unexpected token while parsing.
    UnexpectedToken {
        /// Token not expected by the grammar.
        token: lex::Token<'a>,
        /// Expected token (if applicable).
        expected: Option<lex::Token<'static>>,
    },
    /// Used an undefined path.
    InvalidPath {
        /// The path itself, which is invalid.
        path: ast::Path<'a>,
        /// A reason for the error.
        reason: Option<&'static str>,
    },
    /// Attempted to shadow a named symbol.
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
        /// Justification for not using the forbidden identifier.
        reason: Option<&'static str>,
    },
}
