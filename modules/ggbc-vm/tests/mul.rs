mod utils;

#[test]
fn mul() {
    let memory = utils::run(include_str!("programs/mul.ggb"));
    assert_eq!(&[110, 110], &memory.static_[..2])
}
