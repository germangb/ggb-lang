//! Compiler for the `GGB` (Great Game Boy) programming language.
//!
//! Definition of the intermediate representation (IR).
//!
//! Compilation to multiple targets (**Rust** and **LR35902**).

pub use ggbc_parser as parser;
pub mod ir;
pub mod targets;

pub use crate::targets::{lr35902::LR35902, rust::Rust};
use crate::targets::Target;

pub fn compile<T: Target>(input: &str) -> Result<Vec<u8>, T::Error> {
    unimplemented!()
}