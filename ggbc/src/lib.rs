//! Compiler for the `GGB` (Great Game Boy) programming language.
//!
//! Definition of the **intermediate representation** (IR) and compilation
//! **targets**.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.

#![warn(
    clippy::all,
    clippy::doc_markdown,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::pub_enum_variant_names,
    clippy::mem_forget,
    clippy::use_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    unused,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

pub use byteorder;
pub use parser;
use target::Target;
use thiserror::Error;

pub mod ir;
pub mod target;

pub type Bytes = Box<[u8]>;

/// Compilation errors.
#[derive(Error, Debug)]
pub enum Error<'a, T: Target> {
    #[error("Parsing error")]
    Parser(parser::Error<'a>),

    #[error("Codegen error")]
    Codegen(T::Error),
}

impl<'a, T: Target> From<parser::Error<'a>> for Error<'a, T> {
    fn from(error: parser::Error<'a>) -> Self {
        Self::Parser(error)
    }
}

/// Compile a program.
/// # Example
/// ```
/// use ggbc::target::LR35902;
///
/// // compile GB rom
/// # #[cfg(well_actually_no)]
/// let program = ggbc::compile::<LR35902>(include_str!("program.ggb")).unwrap();
/// ```
pub fn compile<T: Target>(input: &str) -> Result<T::Output, Error<'_, T>> {
    let ast = parser::parse(input)?;
    let mut ir = ir::Ir::new(&ast);
    ir.optimize();
    T::codegen(&ir).map_err(Error::Codegen)
}
