//! LR35902 (Game Boy) CPU compilation target.
use crate::{byteorder::LittleEndian, ir::Ir, target::Target, Bytes};
use thiserror::Error;

/// LR35902 (Game Boy) CPU compilation target.
#[derive(Debug)]
#[warn(clippy::empty_enum)]
pub enum LR35902 {}

/// LR35902-codegen-related errors.
#[derive(Error, Debug)]
#[warn(clippy::empty_enum)]
pub enum Error {}

impl Target for LR35902 {
    type ByteOrder = LittleEndian;
    type Output = Bytes;
    type Error = Error;

    #[warn(unused)]
    fn codegen(_ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
