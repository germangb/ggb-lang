//! ```
//! use lang::Program;
//!
//! let input = r#"
//!     use foo;
//!
//!     mod bar {
//!     }
//!
//!     struct Foo {
//!         foo :: u8,
//!         bar :: [i8; 42],
//!         baz :: [[i16; 4]; 4],
//!     }
//!
//!     memory in [..] {
//!         bar      :: u8,
//!         baz      :: [u8; 1024],
//!         point    :: struct {
//!                         y :: i8,
//!                         x :: i8,
//!                     },
//!         foo      :: Foo,
//!         _padding :: [u8; 100],
//!     }
//! "#;
//!
//! let program: Program = lang::parse(input).unwrap();
//!
//! assert_eq!(4, program.inner.len());
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
