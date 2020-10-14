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
//! - **Type checking** of expressions (the `GGB` language is [strongly-typed]).
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
//! Although if enabled, a ggbc-ir may try to optimize them if the operands are
//! **powers of 2**.
//!
//! [lexing]: https://en.wikipedia.org/wiki/Lexical_analysis
pub mod ast;
pub mod error;
pub mod lex;
pub mod span;

pub use crate::ast::{parse, parse_with_context, Ast, ContextBuilder};
