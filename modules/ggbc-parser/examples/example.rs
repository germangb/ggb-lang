use colorful::Colorful;
use ggbc_parser::{
    span::{Span, Spanned},
    Ast, Error,
};

fn error(input: &str, Span { min, max }: Span, message: &str) {
    assert_eq!(min[0], max[0]);
    let lines: Vec<_> = input.lines().collect();
    let line = lines.iter().skip(min[0]).next().unwrap();
    let line_index = min[0];
    let prefix = format!("{} | ", line_index + 1);

    if min[0] > 0 {
        for _ in 0..prefix.len() - 3 {
            eprint!("{}", ".".blue().bold());
        }
        eprintln!("{}", " |".blue().bold());
    }

    let left = &line[0..min[1]];
    let mid = &line[min[1]..max[1]];
    let right = &line[max[1]..];
    eprint!("{}", prefix.clone().blue().bold());
    eprintln!("{}{}{}", left, mid, right);

    for _ in 0..prefix.len() - 3 {
        if min[0] < lines.len() - 1 {
            eprint!("{}", ".".blue().bold());
        } else {
            eprint!(" ");
        }
    }
    eprint!("{}", " | ".blue().bold());
    for _ in 0..min[1] {
        eprint!(" ");
    }
    for _ in min[1]..max[1] {
        eprint!("{}", "^".red().bold());
    }
    eprintln!(" {}", message.red());
}

fn main() {
    let input = include_str!("example.ggb");
    match ggbc_parser::parse::<Ast>(input) {
        Err(Error::InvalidPath { path, .. }) => error(input, path.span(), "Undefined Path."),
        Err(Error::UnexpectedToken { token, .. }) => {
            error(input, token.span(), "Unexpected Token.")
        }
        Err(Error::ShadowIdent { ident, .. }) => error(input, ident.span(), "Shadowed identifier."),
        other => {
            let _ = other.unwrap();
        }
    }
}
