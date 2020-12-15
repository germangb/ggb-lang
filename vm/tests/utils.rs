use ggbc::{byteorder::NativeEndian, ir::Ir};
use vm::{memory::Memory, Machine, Opts};

pub fn run(input: &str) -> Memory {
    let ast = ggbc::parser::parse(input).unwrap();
    let ir: Ir<NativeEndian> = Ir::new(&ast);
    Machine::new(&ir, Opts::default()).run()
}
