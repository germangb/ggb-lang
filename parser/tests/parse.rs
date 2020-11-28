#[test]
fn parse() {
    ggbc_parser::parse(include_str!("programs/parse.ggb")).unwrap();
}
