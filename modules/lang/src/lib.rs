//! ```
//! use lang::Program;
//!
//! let input = r#"
//!     mod std {
//!         static@0x8000 VRAM :: struct {
//!             tile_data :: union {
//!                 x8000 :: struct {                          data :: [u8; 0x1000] },
//!                 x8800 :: struct { _padding :: [u8; 0x800], data :: [u8; 0x1000] },
//!             },
//!             tile_map :: struct { x9800, x9c00 :: [u8; 0x400] },
//!         };
//!     }
//!
//!     static P0, P1 :: struct {
//!         y, x, :: u16,
//!         name :: [u8; 5],
//!     };
//!
//!     fn main(foo, bar :: u8) u8 {
//!         let i = 0;
//!         loop {
//!             //gb::VRAM::tile_data[i] = 0;
//!             //gb::VRAM::tile_data[i+1] = 0;
//!             //gb::VRAM::tile_data[i+2] = 0;
//!         }
//!         //return 0;
//!     }
//! "#;
//!
//! let program: Program = lang::parse(input).unwrap();
//! assert_eq!(3, program.inner.len());
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
