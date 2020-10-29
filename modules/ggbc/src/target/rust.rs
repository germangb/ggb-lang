use crate::{
    byteorder::{ByteOrder, NativeEndian},
    ir::Ir,
    target::Target,
};
use std::marker::PhantomData;

/// Rust compilation target.
///
/// Generic over the byte ordering of the IR.
#[derive(Debug)]
pub struct Rust<B: ByteOrder = NativeEndian>(PhantomData<B>);

impl<B: ByteOrder> Target for Rust<B> {
    type ByteOrder = B;
    type Output = Vec<u8>;
    type Error = std::convert::Infallible;

    fn codegen(ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
