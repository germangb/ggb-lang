mod utils;

#[test]
fn fibonacci() {
    let memory = utils::run(include_str!("programs/compare.ggb"));
    assert_eq!(&[1, 0, 0, 1, 1, 1], &memory.static_[..6])
}
