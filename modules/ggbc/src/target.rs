//! Compilation targets.
use crate::ir::Ir;

pub use lr35902::LR35902;
pub use rust::Rust;

pub mod lr35902;
pub mod rust;

pub trait Target {
    /// Compilation output.
    type Output;
    /// Codegen error type.
    type Error;

    fn codegen(ir: &Ir) -> Result<Self::Output, Self::Error>;
}
