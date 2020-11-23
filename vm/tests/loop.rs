mod utils;

#[test]
fn loop_() {
    let memory = utils::run(include_str!("programs/loop.ggb"));
    assert_eq!(&[120, 120, 120], &memory.static_[..3])
}
