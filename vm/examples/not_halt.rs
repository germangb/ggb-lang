mod utils;

fn main() {
    utils::run(include_str!("../tests/programs/not_halt.ggb"), None)
}
