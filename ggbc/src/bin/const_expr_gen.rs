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
fn gen(
    target: u8,
    const_out: &mut Vec<u8>,
    expr_out: &mut Vec<u8>,
    depth: usize,
) -> Result<(), io::Error> {
    if depth < 16 && (target == 0xff || rand::random()) {
        match Op::rand() {
            Op::Add => {
                let (l, r) = add_split(target);
                write!(expr_out, "(+ ")?;
                gen(l, const_out, expr_out, depth + 1)?;
                write!(expr_out, " ")?;
                gen(r, const_out, expr_out, depth + 1)?;
                write!(expr_out, ")")?;
            }
            Op::Sub => {
                let (l, r) = sub_split(target);
                write!(expr_out, "(- ")?;
                gen(l, const_out, expr_out, depth + 1)?;
                write!(expr_out, " ")?;
                gen(r, const_out, expr_out, depth + 1)?;
                write!(expr_out, ")")?;
            }
        }
    } else {
        if rand::random() {
            let const_ident = format!("c{}", expr_out.len());
            write!(const_out, "const {}:u8={}\n", const_ident, target)?;
            write!(expr_out, "{}", const_ident)?;
        } else {
            write!(expr_out, "{}", target)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    // target expression
    let target = 0xff;
    let mut const_out = Vec::new();
    let mut expr_out = Vec::new();

    gen(target, &mut const_out, &mut expr_out, 0).unwrap();

    io::copy(&mut Cursor::new(const_out), &mut io::stdout())?;
    println!("static RESULT:u8");
    print!("(= RESULT ");
    io::copy(&mut Cursor::new(expr_out), &mut io::stdout())?;
    println!(")");
    Ok(())
}
