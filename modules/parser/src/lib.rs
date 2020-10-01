//! Parser for the `GGB` (Great Game Boy) language.
//!
//! Tools to perform (static) syntactic analysis on high-level source code.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.
#[deny(unused)]
#[macro_use]
mod macros;
pub mod ast;
pub mod error;
pub mod lex;
pub mod span;

pub use crate::{
    ast::{parse, parse_program, parse_program_with_context, parse_with_context, Context, Program},
    error::Error,
};
