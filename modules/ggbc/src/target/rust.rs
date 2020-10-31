//! Rust compilation target.
use crate::{
    byteorder::{ByteOrder, NativeEndian},
    ir::Ir,
    target::Target,
};

/// Rust compilation target.
#[derive(Debug)]
pub enum Rust {}

impl Target for Rust {
    type ByteOrder = NativeEndian;
    type Output = Vec<u8>;
    type Error = std::convert::Infallible;

    fn codegen(ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
