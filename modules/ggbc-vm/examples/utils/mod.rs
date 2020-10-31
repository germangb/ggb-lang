use ggbc::{ir::Ir, parser::Ast};
use ggbc_vm::{Opts, VM};
use std::ops::Range;

pub fn run(program: &str, range: Option<Range<usize>>) {
    print_input(program);
    let ast = ggbc::parser::parse(program).unwrap();
    #[cfg(nope)]
    print_ast(&ast);
    let ir = Ir::compile(&ast);
    print_ir(&ir);
    let mut vm = VM::new(ir, Opts::default());
    run_vm(&mut vm);
    print_result(&vm, range);
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
        if let Some(name) = &routine.debug_name {
            print!(" {}#{}", name, i);
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

fn print_result(vm: &VM, range: Option<Range<usize>>) {
    println!();
    println!("Result (memory)");
    println!("===");
    const OUTPUT: usize = 16;
    for (addr, b) in vm.statik()[range.unwrap_or(0..OUTPUT)].iter().enumerate() {
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
