mod utils;

#[test]
fn function() {
    let memory = utils::run(include_str!("programs/function.ggb"));
    assert_eq!(&[2], &memory.static_[..1])
}
