//! utility to generate const_expr test cases
use rand::prelude::*;
use std::{
    io,
    io::{Cursor, Write},
};

enum Op {
    Add,
    Sub,
}

impl Op {
    fn rand() -> Self {
        match rand::thread_rng().gen_range(0, 2) {
            0 => Self::Add,
            1 => Self::Sub,
            _ => unreachable!(),
        }
    }
}

fn add_split(n: u8) -> (u8, u8) {
    let div = thread_rng().gen_range(1, 8);
    (n / div, n - n / div)
}

fn sub_split(n: u8) -> (u8, u8) {
    if n < 0xff {
        let diff = 0xff - n;
        let t = rand::thread_rng().gen_range(0, diff);
        (n + t, t)
    } else {
        (n, 0)
    }
}

// generate a const u8 expression
fn gen(target: u8, out: &mut Vec<u8>, depth: usize) -> Result<(), io::Error> {
    if depth < 16 && (target == 0xff || rand::random()) {
        match Op::rand() {
            Op::Add => {
                let (l, r) = add_split(target);
                write!(out, "(+ ")?;
                gen(l, out, depth + 1)?;
                write!(out, " ")?;
                gen(r, out, depth + 1)?;
                write!(out, ")")?;
            }
            Op::Sub => {
                let (l, r) = sub_split(target);
                write!(out, "(- ")?;
                gen(l, out, depth + 1)?;
                write!(out, " ")?;
                gen(r, out, depth + 1)?;
                write!(out, ")")?;
            }
        }
    } else {
        write!(out, "{}", target)?;
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    // target expression
    let target = 0xff;
    let mut expr = Vec::new();

    gen(target, &mut expr, 0).unwrap();

    print!("static RESULT:u8 ");
    print!("(= RESULT ");
    io::copy(&mut Cursor::new(expr), &mut io::stdout())?;
    println!(")");
    Ok(())
}
