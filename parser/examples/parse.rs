use colorful::Colorful;
use parser::{
    lex::span::{Span, Spanned},
    Error,
};

fn error(input: &str, Span { min, max }: Span, message: &str) {
    assert_eq!(min[0], max[0]);
    let lines: Vec<_> = input.lines().collect();
    let line = lines.iter().nth(min[0]).unwrap();
    let line_index = min[0];
    let prefix = format!("{} | ", line_index + 1);

    if min[0] > 0 {
        for _ in 0..prefix.len() - 3 {
            eprint!(" ");
        }
        eprintln!("{}", " |".blue().bold());
    }

    let left = &line[0..min[1]];
    let mid = &line[min[1]..max[1]];
    let right = &line[max[1]..];
    eprint!("{}", prefix.clone().blue().bold());
    eprintln!("{}{}{}", left, mid, right);

    for _ in 0..prefix.len() - 3 {
        eprint!(" ");
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
    let input = include_str!("../tests/programs/parse.ggb");
    match parser::parse(input) {
        Err(Error::InvalidPath(path)) => error(input, path.span(), "Invalid Path."),
        Err(Error::UnexpectedToken(token)) => error(input, token.span(), "Unexpected Token."),
        Err(Error::ShadowIdent { shadow: ident, .. }) => {
            error(input, ident.span(), "Shadow identifier.")
        }
        other => {
            let _ = other.unwrap();
        }
    }
}
