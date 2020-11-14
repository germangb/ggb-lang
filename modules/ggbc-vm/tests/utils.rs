use ggbc::{byteorder::NativeEndian, ir::Ir};
use ggbc_vm::{Memory, Opts, VM};

pub fn run(input: &str) -> Memory {
    let ast = ggbc::parser::parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    VM::new(&ir, Opts::default()).run()
}
