mod utils;

#[test]
#[ignore]
fn deref() {
    let memory = utils::run(include_str!("programs/deref.ggb"));
    assert_eq!(&[1, 2, 3], &memory.static_[..3])
}
