use ggbc::{ir::Ir, parser::Ast};
use ggbc_vm::VM;

static INPUT: &str = indoc::indoc! {r#"
    // store result here
    static RESULT:u8

    // multiply two nibbles
    let foo:u8 = 0xa
    let bar:u8 = 0xb
    loop {
        if foo {
            (= RESULT (+ RESULT bar))
            (= foo (- foo 1))
        } else {
            break
        }
    }
"#};

fn print_input(input: &str, ast: &Ast) {
    println!("Input");
    println!("===");
    println!("{}", input);
    #[cfg(nope)]
    {
        println!();
        println!("Ast");
        println!("===");
        println!("{:?}", ast);
    }
}

fn print_ir(ir: &Ir) {
    println!();
    println!("Ir");
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

fn run_vm(mut vm: VM) {
    println!();
    println!("Cycles");
    println!("===");
    let mut cycles = 0;
    while vm.running() {
        vm.update();
        cycles += 1;
    }
    println!("Ran for {} cycles", cycles);

    println!();
    println!("Result (memory)");
    println!("===");
    const OUTPUT: usize = 8;
    for (addr, b) in vm.statik()[..OUTPUT].iter().enumerate() {
        println!("{:04x} | {:02x} ({})", addr, b, b);
    }
}

fn main() {
    // run program
    let ast = ggbc::parser::parse(INPUT).unwrap();
    // display program
    print_input(INPUT, &ast);

    let ir = ggbc::ir::compile(&ast);
    print_ir(&ir);

    run_vm(VM::new(ir));
}
