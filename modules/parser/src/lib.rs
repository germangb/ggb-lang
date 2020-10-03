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
//!
//! # Example
//!
//! ```
//! # parser::parse_program(r#"
//! // adds a layer of typing to an existing region of memory
//! // here, VRAM starts at address 0x8000 ans is layed out like this:
//! static@0x8000 VRAM :: struct {
//!     tile_data :: union {
//!         x8000 :: struct {                        data::[u8; 0x1000] },
//!         x8800 :: struct { _padding::[u8; 0x800], data::[u8; 0x1000] }
//!     },
//!     tile_map :: struct { x9800::[u8; 0x400],
//!                          x9c00::[i8; 0x400] }
//! };
//!
//! // C-style for loop
//! for i::u16 in 0..16 {
//!     let offset = 16<<2;
//!     VRAM::tile_data::x8000[offset + i] = 0xff;
//! }
//!
//! // more concisely:
//! for offset::u16 in 16<<2..+16 {
//!     VRAM::tile_data::x8000[offset] = 0xff;
//! }
//!
//! VRAM::tile_map::x9800[0] = 4;
//! # "#).unwrap();
//! ```
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
