use std::io;

fn gen(out: &mut Vec<u8>, depth: u8) -> Result<(), io::Error> {
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let mut out = Vec::new();
    gen(&mut out, 0)?;
    Ok(())
}
