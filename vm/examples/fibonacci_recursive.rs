mod utils;

fn main() {
    utils::run(
        include_str!("../tests/programs/fibonacci_recursive.ggb"),
        None,
    )
}
