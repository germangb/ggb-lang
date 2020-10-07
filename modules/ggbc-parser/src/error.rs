//! Errors and multiple error reporting.
use crate::{ast, ast::Expression, lex, span::Span};
use std::borrow::Cow;

#[derive(Debug)]
pub enum Error<'a> {
    ReservedKeyword {
        /// The keyword itself.
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
    /// Used an undefined path.
    InvalidPath {
        path: ast::expressions::Path<'a>,
    },
    /// Invalid expression (due to type checking).
    InvalidExpression(ast::Expression<'a>),
    /// Used the continue; outside of a loop.
    InvalidContinue(ast::Continue<'a>),
    /// Used the break; outside of a loop.
    InvalidBreak(ast::Break<'a>),
    Eof,
}
