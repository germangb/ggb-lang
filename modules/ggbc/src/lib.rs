//! Compiler for the `GGB` (Great Game Boy) programming language.
//!
//! Definition of the **intermediate representation** (IR) and compilation
//! **targets**.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.

pub use byteorder;
pub use ggbc_parser as parser;

pub mod error;
pub mod ir;
pub mod target;

/// Compile a program.
/// # Example
/// ```
/// use ggbc::target::LR35902;
///
/// // compile GB rom
/// # #[cfg(well_actually_no)]
/// let program = ggbc::compile::<LR35902>(include_str!("program.ggb")).unwrap();
/// ```
pub fn compile<T: target::Target>(input: &str) -> Result<T::Output, error::Error<T>> {
    let ast = parser::parse(input)?;
    let ir = ir::Ir::new(&ast);
    T::codegen(&ir).map_err(|e| error::Error::Codegen(e))
}
