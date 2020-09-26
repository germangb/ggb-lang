//! ```
//! use lang::Program;
//!
//! let input = r#"
//!     mod gb { }
//!
//!     struct Arrays {
//!         _0 :: [u8; 1],
//!         _1 :: [[u8; 1]; 1],
//!         _2 :: [[[u8; 1]; 1]; 1],
//!         _3 :: [[[[u8; 1]; 1]; 1]; 1],
//!     }
//!
//!     use gb::wram;
//!     use gb::vram;
//!
//!     memory in wram[..] {
//!         bar    :: u8,
//!         baz    :: [u8; 1024],
//!         point  :: struct { y :: i8, x :: i8, },
//!         arrays :: Arrays,
//!     }
//!
//!     vram in vram[..] {
//!         tile_data :: union {
//!             x8000 :: struct {                         data :: [u8; 4096], },
//!             x8800 :: struct { _padding :: [u8; 2048], data :: [u8; 4096], },
//!         },
//!         tile_map  :: struct { x9800 :: [u8; 1024],
//!                               x9c00 :: [u8; 1024], },
//!     }
//! "#;
//!
//! let program: Program = lang::parse(input).unwrap();
//!
//! assert_eq!(6, program.inner.len());
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
