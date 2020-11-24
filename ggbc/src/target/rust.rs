//! Rust compilation target.
use crate::{byteorder::NativeEndian, ir::Ir, target::Target};
use std::io::Write;

/// Rust compilation target.
#[derive(Debug)]
#[warn(clippy::empty_enum)]
pub enum Rust {}

impl Target for Rust {
    type ByteOrder = NativeEndian;
    type Output = String;
    type Error = std::io::Error;

    #[warn(unused)]
    fn codegen(ir: &Ir<Self::ByteOrder>) -> Result<Self::Output, Self::Error> {
        let mut output = Vec::new();

        writeln!(&mut output,
                 "static CONST:[u8;{}] = {:?};",
                 ir.const_.len(),
                 ir.const_)?;
        writeln!(&mut output,
                 "static mut STATIC:[u8;{}] = [0; {}];",
                 ir.static_alloc, ir.static_alloc)?;
        writeln!(&mut output, "static mut REGISTERS:[u8;16] = [0;16];")?;

        for routine in ir.routines.iter() {
            writeln!(&mut output,
                     "fn _{}(args:[u8;{}])->[u8;{}] {{",
                     routine.debug_name.as_ref().unwrap(),
                     routine.args_size,
                     routine.return_size)?;
            writeln!(&mut output, "    let mut ret=[0;{}];", routine.return_size)?;
            writeln!(&mut output, "    ret")?;
            writeln!(&mut output, "}}")?;
        }
        writeln!(&mut output, "fn main() {{ _main([]); }}")?;
        Ok(String::from_utf8(output).unwrap())
    }
}
