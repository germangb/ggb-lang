//! Compilation targets.
use crate::ir::Intermediate;

pub mod lr35902;
pub mod rust;

pub trait Target {
    type Error;

    fn emmit(ir: &Intermediate) -> Result<Vec<u8>, Self::Error>;
}
