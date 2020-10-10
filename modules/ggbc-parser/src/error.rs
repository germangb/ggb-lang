//! Errors and multiple error reporting.
use crate::{ast, lex, span::Span};
use std::borrow::Cow;

#[derive(Debug)]
pub enum Error<'a> {
    /// Error signaling that a reserved keyword has been found.
    /// Reserved keyword may be used in future revisions of the language.
    ReservedKeyword {
        /// The keyword itself.
        key_word: Cow<'a, str>,
        /// Location of the keyword in the input source.
        span: Span,
    },
    /// Encountered an unexpected byte in the input source code.
    UnexpectedByte {
        /// The unexpected byte.
        byte: u8,
        /// Location of the byte in the input source code.
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
        /// The conflicting identifier within path above.
        ident: Option<lex::Ident<'a>>,
    },
    /// Tried to access a private field.
    PrivatePath(ast::Path<'a>),
    /// Invalid expression (due to type checking).
    InvalidExpression(ast::Expression<'a>),
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
    Eof,
}
