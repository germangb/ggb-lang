use ggbc::ir::Ir;
use ggbc_vm::{Memory, Opts, VM};

pub fn run(input: &str) -> Memory {
    let ast = ggbc::parser::parse(input).unwrap();
    let ir = Ir::new(&ast);
    VM::<ggbc::byteorder::NativeEndian>::new(&ir, Opts::default()).run()
}
