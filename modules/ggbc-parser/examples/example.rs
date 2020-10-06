use ggbc_parser::{
    span::{Span, Spanned},
    Ast, Error,
};
use colorful::Colorful;

fn main() {
    let input = include_str!("example.ggb");
    match ggbc_parser::parse::<Ast>(input) {
        Err(Error::UndefinedPath { path }) => {
            let Span { min, max } = path.span();
            assert_eq!(min[0], max[0]);
            let line = input.lines().skip(min[0]).next().unwrap();
            let line_num_prefix = format!("{} | ", min[0]);

            println!("{}{}", line_num_prefix, line);
            for _ in 0..min[1] + line_num_prefix.len() {
                print!(" ");
            }
            for _ in min[1]..max[1] {
                print!("{}", "^".red());
            }
            println!();

            println!("\n---\n{:?}", path);
        }
        other => {
            other.unwrap();
        }
    }
}
