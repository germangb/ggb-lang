use ggbc::{ir::Ir, parser::Ast};
use ggbc_vm::VM;

pub fn run(program: &str) {
    print_input(program);
    let ast = ggbc::parser::parse(program).unwrap();
    #[cfg(nope)]
    print_ast(&ast);
    let ir = ggbc::ir::compile(&ast);
    print_ir(&ir);
    let mut vm = ggbc_vm::VM::new(ir);
    run_vm(&mut vm);
    print_result(&vm);
}

fn print_input(input: &str) {
    println!("Input");
    println!("===");
    println!("{}", input);
}

fn print_ast(ast: &Ast) {
    println!();
    println!("Ast");
    println!("===");
    println!("{:?}", ast);
}

fn print_ir(ir: &Ir) {
    println!();
    println!("Intermediate code");
    println!("===");
    for (i, routine) in ir.routines.iter().enumerate() {
        print!("     |");
        if let Some(name) = &routine.name {
            print!(" {}", name);
        } else {
            print!(" routine#{}", i);
        }
        if i == ir.main {
            print!(" (main)");
        }
        println!(":");
        for (i, statement) in routine.statements.iter().enumerate() {
            println!("{:04x} |   {:?}", i, statement);
        }
    }
}

fn print_result(vm: &VM) {
    println!();
    println!("Result (memory)");
    println!("===");
    const OUTPUT: usize = 8;
    for (addr, b) in vm.statik()[..OUTPUT].iter().enumerate() {
        println!("{:04x} | {:02x} ({})", addr, b, b);
    }
}

fn run_vm(vm: &mut VM) {
    println!();
    println!("Cycles");
    println!("===");
    let mut cycles = 0;
    while vm.running() {
        vm.update();
        cycles += 1;
    }
    println!("Ran for {} cycles", cycles);
}
