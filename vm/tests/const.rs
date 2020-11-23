mod utils;

#[test]
fn const_() {
    let memory = utils::run(include_str!("programs/const.ggb"));
    assert_eq!(&[4, 2], &memory.static_[..2])
}
