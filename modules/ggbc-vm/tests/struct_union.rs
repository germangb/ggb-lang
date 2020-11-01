mod utils;

#[test]
fn struct_() {
    let memory = utils::run(include_str!("programs/struct.ggb"));
    assert_eq!(&[1, 2, 3, 4], &memory.static_[..4])
}

#[test]
fn union() {
    let memory = utils::run(include_str!("programs/union.ggb"));
    assert_eq!(&[3, 4], &memory.static_[..2])
}
