mod utils;

#[test]
fn array_assign() {
    let memory = utils::run(include_str!("programs/assign.ggb"));
    assert_eq!(&[4, 8, 2, 0xff, 0xf8, 0x8, 0x10], &memory.static_[..7])
}
