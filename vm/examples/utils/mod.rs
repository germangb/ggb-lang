use ggbc::{byteorder::NativeEndian, ir::Ir, parser::Ast};
use std::ops::Range;
use vm::{memory::Memory, Machine, Opts};

pub fn run(program: &str, range: Option<Range<usize>>) {
    print_input(program);
    let ast = ggbc::parser::parse(program).unwrap();
    #[cfg(nope)]
    print_ast(&ast);
    let ir = Ir::new(&ast);
    print_ir(&ir);
    let vm: Machine<NativeEndian> = Machine::new(&ir, Opts::default());
    print_result(&vm.run(), range);
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

fn print_ir(ir: &Ir<NativeEndian>) {
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
        print!(" (args: {}b)", routine.args_size);
        println!(":");
        for (i, statement) in routine.statements.iter().enumerate() {
            println!("{:04x} |   {:?}", i, statement);
        }
    }
}

fn print_result(memory: &Memory, range: Option<Range<usize>>) {
    println!();
    println!("Result (memory)");
    println!("===");
    const OUTPUT: usize = 16;
    for (addr, b) in memory.static_[range.unwrap_or(0..OUTPUT)]
        .iter()
        .enumerate()
    {
        println!("{:04x} | {:02x} ({})", addr, b, b);
    }
}
