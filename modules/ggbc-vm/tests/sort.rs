mod utils;

#[test]
fn sort() {
    let memory = utils::run(include_str!("programs/sort.ggb"));
    assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
               &memory.static_[..16])
}
