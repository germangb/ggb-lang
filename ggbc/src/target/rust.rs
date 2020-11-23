//! Rust compilation target.
use crate::{byteorder::NativeEndian, ir::Ir, target::Target};

/// Rust compilation target.
#[derive(Debug)]
#[warn(clippy::empty_enum)]
pub enum Rust {}

impl Target for Rust {
    type ByteOrder = NativeEndian;
    type Output = Vec<u8>;
    type Error = std::convert::Infallible;

    #[warn(unused)]
    fn codegen(_ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
