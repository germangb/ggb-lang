#[test]
fn parse() {
    parser::parse(include_str!("programs/parse.ggb")).unwrap();
}
