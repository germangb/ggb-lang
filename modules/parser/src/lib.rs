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
//! # "#;
//! #
//! # let program: parser::Program = parser::parse(input).unwrap();
//! # assert_eq!(3, program.inner.len());
//! ```
pub use crate::ast::{Context, Program};
use crate::{
    ast::{Error, Parse},
    lex::Tokens,
};

pub mod ast;
pub mod lex;

pub fn parse<'a, P: Parse<'a>>(input: &'a str) -> Result<P, Error> {
    let mut context = Context::new();
    parse_with_context(&mut context, input)
}

pub fn parse_with_context<'a, P: Parse<'a>>(
    context: &mut Context,
    input: &'a str,
) -> Result<P, Error> {
    let mut tokens = Tokens::new(input).peekable();
    P::parse(context, &mut tokens)
}
