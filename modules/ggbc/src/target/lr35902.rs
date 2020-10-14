use crate::{ir::Ir, target::Target};

#[derive(Debug)]
pub enum LR35902 {}

impl Target for LR35902 {
    type Output = Vec<u8>;
    type Error = std::convert::Infallible;

    fn codegen(ir: &Ir) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
