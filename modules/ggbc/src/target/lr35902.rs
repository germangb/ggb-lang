use crate::{byteorder::LittleEndian, ir::Ir, target::Target};

/// LR35902 (Game Boy) CPU with no ROM bank switching.
#[derive(Debug)]
pub enum LR35902 {}

/// LR35902-codegen-related errors.
#[derive(Debug)]
pub enum Error {}

impl Target for LR35902 {
    type ByteOrder = LittleEndian;
    type Output = Vec<u8>;
    type Error = Error;

    fn codegen(ir: &Ir) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
