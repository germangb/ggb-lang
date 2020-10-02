//! Parser for the `GGB` (Great Game Boy) language.
//!
//! Tools to perform (static) syntactic analysis on high-level source code.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.
//!
//! # Grammar
//! The most important type of this crate is the [`Grammar`] trait, which does:
//! - Group [`Tokens`] into grammars (syntactic analysis).
//! - Type checking of expressions.
//! - Check undefined and/or out-of-reach identifiers.
//!
//! [`Grammar`]: ./ast/trait.Grammar.html
//! [`Tokens`]: ./lex/struct.Tokens.html
//!
//! # Feature flags
//! - `mul` includes multiplication grammars.
//! - `div` includes division grammars.
pub mod ast;
pub mod error;
pub mod lex;
pub mod span;

pub use crate::{
    ast::{
        parse_grammar, parse_grammar_with_context, parse_program, parse_program_with_context,
        Context, Program,
    },
    error::Errors,
};
