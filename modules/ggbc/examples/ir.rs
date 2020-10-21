fn main() {
    let ast = ggbc::parser::parse(include_str!("../../ggbc-parser/examples/example.ggb"))
        .expect("Parsing error");
    let ir = ggbc::ir::compile(&ast);

    for (i, routine) in ir.routines.iter().enumerate() {
        if i == ir.main {
            println!("main:");
        } else {
            println!("{}:", i);
        }
        for statement in &routine.statements {
            println!("{:?}", statement);
        }
    }
}
