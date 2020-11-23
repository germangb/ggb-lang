mod utils;

fn main() {
    utils::run(include_str!("../tests/programs/memcopy.ggb"), Some(0..54))
}
