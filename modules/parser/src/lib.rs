//! Parser for the `GGB` (Great Game Boy) programming language.
//!
//! Tools to perform (static) [syntax analysis] on high-level source code.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.
//!
//! [syntax analysis]: https://en.wikipedia.org/wiki/Syntax_(programming_languages)
//!
//! # Grammar
//! The most important type of this crate is the [`Grammar`] trait, which does:
//! - Group [`Tokens`] into grammar groups (syntactic analysis). The syntax
//!   itself looks like a mixture of Rust and C.
//! - **Type checking** of expressions. The `GGB` language is a [strongly-typed]
//!   language.
//! - Resolution of symbolic identifiers.
//!
//! [`Grammar`]: ./ast/trait.Grammar.html
//! [`Tokens`]: ./lex/struct.Tokens.html
//! [strongly-typed]: https://en.wikipedia.org/wiki/Strong_and_weak_typing
//!
//! # Feature flags
//! - `mul` includes multiplication grammars.
//! - `div` includes division grammars.
//!
//! These flags are **disabled by default** is because the Game Boy CPU
//! (LR35902) doesn't have general division/multiplication instructions.
//! Although if enabled, a compiler may try to optimize them if the operands are
//! **powers of 2**.
//!
//! # What this is not
//! - A syntax tree / expression optimizer. Those tasks should be left for an IR
//!   and/or a code generator.
//! - A full compiler pipeline, only the front-end part ([lexing] and and syntax
//!   analysis).
//!
//! [lexing]: https://en.wikipedia.org/wiki/Lexical_analysis
pub mod ast;
pub mod error;
pub mod lex;
pub mod span;

pub use crate::{
    ast::{parse, parse_with_context, Ast, ContextBuilder},
    error::Errors,
};
