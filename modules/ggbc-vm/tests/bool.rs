mod utils;

#[test]
fn fibonacci() {
    let memory = utils::run(include_str!("programs/bool.ggb"));
    assert_eq!(&[42], &memory.static_[..1])
}
