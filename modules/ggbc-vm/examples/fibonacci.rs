mod utils;

fn main() {
    utils::run(include_str!("programs/fibonacci.ggb"), Some(0..13))
}
