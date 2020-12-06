//! Compilation targets and codegen.
use crate::{byteorder::ByteOrder, ir::Ir};

pub use lr35902::LR35902;
pub use rust::Rust;
use std::{error::Error, fmt::Debug};

pub mod lr35902;
pub mod rust;

pub trait Target: Debug {
    /// Target byte ordering.
    type ByteOrder: ByteOrder;

    /// Compilation output.
    type Output;

    /// Codegen error type.
    type Error: Error;

    fn codegen(ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error>;
}
