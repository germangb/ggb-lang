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
    let input = include_str!("example.ggb");
    match ggbc_parser::parse(input) {
        Err(Error::InvalidPath {
            path,
            reason: Some(reason),
        }) => error(input, path.span(), &format!("Invalid Path: {}.", reason)),
        Err(Error::InvalidPath { path, .. }) => error(input, path.span(), "Invalid Path."),
        Err(Error::UnexpectedToken { token, .. }) => {
            error(input, token.span(), "Unexpected Token.")
        }
        Err(Error::ShadowIdent { shadow: ident, .. }) => {
            error(input, ident.span(), "Shadow identifier.");
        }
        Err(Error::ForbiddenIdent {
            ident,
            reason: Some(reason),
        }) => error(
            input,
            ident.span(),
            &format!("Forbidden identifier: {}.", reason),
        ),
        Err(Error::ForbiddenIdent { ident, .. }) => {
            error(input, ident.span(), "Forbidden identifier.")
        }
        other => {
            let _ = other.unwrap();
        }
    }
}
