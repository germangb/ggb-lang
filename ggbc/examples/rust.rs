use ggbc::target::Rust;

fn main() {
    let rust =
        ggbc::compile::<Rust>(include_str!("../../vm/tests/programs/recursion.ggb")).unwrap();

    std::fs::write("out.rs", rust).unwrap();
    std::process::Command::new("rustc").args(&["out.rs"])
                                       .spawn()
                                       .unwrap()
                                       .wait()
                                       .unwrap();
    std::fs::remove_file("out.rs").unwrap();
}
