//! Compiler for the `GGB` (Great Game Boy) programming language.
//!
//! Definition of the **intermediate representation** (IR) and compilation
//! **targets**.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.

pub use byteorder;
pub use error::Error;
pub use ggbc_parser as parser;

pub mod error;
pub mod ir;
pub mod target;

pub fn compile<T: target::Target>(input: &str) -> Result<T::Output, Error<T>> {
    let ast = parser::parse(input)?;
    let ir = ir::Ir::compile(&ast);
    T::codegen(&ir).map_err(|e| error::Error::Codegen(e))
}
