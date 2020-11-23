mod utils;

#[test]
fn recursion() {
    let memory = utils::run(include_str!("programs/recursion.ggb"));
    assert_eq!(&[225, 45, 233, 6], &memory.static_[..4])
}
