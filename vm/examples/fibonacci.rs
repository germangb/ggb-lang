mod utils;

fn main() {
    utils::run(include_str!("../tests/programs/fibonacci.ggb"), Some(0..13))
}
