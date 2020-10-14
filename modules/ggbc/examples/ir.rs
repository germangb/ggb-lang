fn main() {
    let ast = ggbc::parser::parse(include_str!("../../ggbc-parser/examples/example.ggb"))
        .expect("Parsing error");
    ggbc::ir::compile(&ast);
}
