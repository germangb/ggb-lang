mod utils;

#[test]
fn fibonacci() {
    let memory = utils::run(include_str!("programs/fibonacci.ggb"));
    assert_eq!(&[1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233],
               &memory.static_[..13])
}
