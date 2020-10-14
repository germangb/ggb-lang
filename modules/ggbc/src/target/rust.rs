use crate::{ir::Ir, target::Target};

#[derive(Debug)]
pub enum Rust {}

impl Target for Rust {
    type Output = Vec<u8>;
    type Error = std::convert::Infallible;

    fn codegen(ir: &Ir) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
