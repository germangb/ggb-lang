use ggbc::target::Rust;

fn main() {
    let rust =
        ggbc::compile::<Rust>(include_str!("../../vm/tests/programs/recursion.ggb")).unwrap();

    println!("{}", rust);
}
