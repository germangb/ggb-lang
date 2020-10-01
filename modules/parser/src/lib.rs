//! ```
//! # let input = r#"
//! mod std {
//!     static@0x8000 VRAM :: struct {
//!         tile_data :: union {
//!             x8000 :: struct {                          data :: [u8; 0x1000] },
//!             x8800 :: struct { _padding :: [u8; 0x800], data :: [u8; 0x1000] },
//!         },
//!         tile_map :: struct { x9800, x9c00 :: [u8; 0x400] },
//!     };
//! }
//!
//! fn black_color u8 {
//!     0xff;
//! }
//!
//! for i in 0..16 {
//!     //std
//!     //  ::VRAM
//!     //  ::tile_data
//!     //  ::x8000[i] = black_color();
//! }
//!
//! //asm {
//! //    run_dma:
//! //      .ld  %a, 0xff00
//! //      .ldh  (0x46), %a
//! //      .ld  %a, 0x28
//! //    wait:
//! //      .dec %a
//! //      .jr  nz, wait
//! //      .ret
//! //}
//! # "#;
//! #
//! # let program: parser::Program = parser::parse(input).unwrap();
//! # assert_eq!(3, program.inner.len());
//! ```
use crate::{ast::Grammar, lex::Tokens};
pub use crate::{
    ast::{Context, Program},
    error::Error,
};

#[macro_use]
mod macros;

pub mod ast;
pub mod error;
#[cfg(feature = "ir")]
pub mod ir;
pub mod lex;

#[doc(inline)]
pub use crate::ast::{parse, parse_with_context};
#[doc(inline)]
#[cfg(feature = "ir")]
pub use crate::ir::compile;
