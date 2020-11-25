use ggbc::target::Rust;
use std::{
    io,
    io::{BufWriter, Cursor},
};

fn main() {
    let rust =
        ggbc::compile::<Rust>(include_str!("../../vm/tests/programs/fibonacci.ggb")).unwrap();

    std::io::copy(&mut Cursor::new(rust.as_bytes()),
                  &mut BufWriter::new(io::stdout())).unwrap();
}
