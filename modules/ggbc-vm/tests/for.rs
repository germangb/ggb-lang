mod utils;

#[test]
fn for_() {
    let memory = utils::run(include_str!("programs/for.ggb"));
    assert_eq!(&[120], &memory.static_[..1])
}
