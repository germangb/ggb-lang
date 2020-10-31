mod utils;

fn main() {
    utils::run(include_str!("programs/memcopy.ggb"), Some(0..54))
}
