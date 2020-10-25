use ggbc::ir::Ir;
use ggbc_vm::VM;

static PROGRAM: &str = r#"
    // store program result here
    static@0x0000 R0:u8
    static@0x0001 R1:u8
    let foo:u8 = 4
    let bar:u8 = 2
    let baz:u8 = (+ foo bar)
    (= R0 0x42)
    (= R1 baz)
"#;

fn print_ir(ir: &Ir) {
    for statement in &ir.routines[ir.main].statements {
        println!("{:?}", statement);
    }
    println!("===");
}

fn run_vm(mut vm: VM) {
    while vm.running() {
        vm.update();
    }

    const OUTPUT: usize = 8;
    for (addr, b) in vm.absolute()[..OUTPUT].iter().enumerate() {
        println!("{:04x} {:02x}", addr, b);
    }
}

fn main() {
    let ast = ggbc::parser::parse(PROGRAM).unwrap();
    let ir = ggbc::ir::compile(&ast);

    print_ir(&ir);
    run_vm(VM::new(ir));
}
