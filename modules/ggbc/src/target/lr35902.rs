//! LR35902 (Game Boy) CPU compilation target.
use crate::{byteorder::LittleEndian, ir::Ir, target::Target};

/// LR35902 (Game Boy) CPU compilation target.
#[derive(Debug)]
pub enum LR35902 {}

/// LR35902-codegen-related errors.
#[derive(Debug)]
pub enum Error {}

impl Target for LR35902 {
    type ByteOrder = LittleEndian;
    type Output = Vec<u8>;
    type Error = Error;

    #[warn(unused)]
    fn codegen(ir: &Ir) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
