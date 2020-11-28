mod utils;

#[test]
fn array_assign() {
    let memory = utils::run(include_str!("programs/array_assign.ggb"));
    assert_eq!(&[1, 2, 3, 4], &memory.static_[..4])
}
