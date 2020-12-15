//! Parser for the `GGB` (Great Game Boy) programming language.
//!
//! Tools to perform [syntax analysis] on input source code.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.
//!
//! [syntax analysis]: https://en.wikipedia.org/wiki/Syntax_(programming_languages)

#![warn(
    clippy::all,
    clippy::doc_markdown,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::pub_enum_variant_names,
    clippy::mem_forget,
    clippy::use_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    unused,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

pub mod ast;
pub mod lex;

use ast::{Context, Grammar};
use lex::span::Span;
use thiserror::Error;

// re-exports
pub use ast::{Ast, ContextBuilder};
pub use lex::Tokens;

/// Parse input source code.
pub fn parse(input: &str) -> Result<Ast<'_>, Error<'_>> {
    let mut context = ContextBuilder::default().build();
    parse_with_context(input, &mut context)
}

/// Parse input source code with a context.
pub fn parse_with_context<'a>(
    input: &'a str,
    context: &mut Context<'a>,
) -> Result<Ast<'a>, Error<'a>> {
    let mut tokens = Tokens::new(input).peekable();
    Grammar::parse(context, &mut tokens)
}

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Early EOF")]
    Eof,

    #[error("Unexpected token: `{0}`")]
    UnexpectedToken(lex::Token<'a>),

    #[error("Invalid path: {0:?}")]
    InvalidPath(ast::Path<'a>),

    #[error("Use of reserved keyword: `{key_word}`")]
    ReservedKeyword {
        /// The keyword itself.
        key_word: &'a str,

        /// Location of the keyword in the programs source.
        span: Span,
    },

    #[error("Unexpected byte: {byte:02x}")]
    UnexpectedByte {
        /// The unexpected byte.
        byte: u8,

        /// Location of the byte in the programs source code.
        span: Span,
    },

    #[error("Shadowed identifier")]
    ShadowIdent {
        /// An already defined and previously validated identifier.
        ident: lex::Ident<'a>,

        /// The new identifier shadowing the one above.
        shadow: lex::Ident<'a>,
    },
}
