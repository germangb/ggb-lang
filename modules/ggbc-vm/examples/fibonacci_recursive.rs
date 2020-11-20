mod utils;

fn main() {
    utils::run(include_str!("../tests/programs/recursion.ggb"), Some(0..2))
}
