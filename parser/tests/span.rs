use ggbc_parser::{
    lex::span::{Span, Spanned},
    Tokens,
};

macro_rules! span {
    ($tokens:expr, $min:expr, $max:expr) => {
        let Span { min, max } = $tokens.next().unwrap().unwrap().span();
        assert_eq!($min, min);
        assert_eq!($max, max);
    };
}

#[test]
fn single_line_let() {
    let input = "let foo:u8 = 4 + 2";
    let mut tokens = Tokens::new(input);

    span!(tokens, [0, 0], [0, 3]); // let
    span!(tokens, [0, 4], [0, 7]); // foo
    span!(tokens, [0, 7], [0, 8]); // :
    span!(tokens, [0, 8], [0, 10]); // u8
    span!(tokens, [0, 11], [0, 12]); // =
    span!(tokens, [0, 13], [0, 14]); // 4
    span!(tokens, [0, 15], [0, 16]); // +
    span!(tokens, [0, 17], [0, 18]); // 2
}

#[test]
fn skip_whitespace() {
    let input = "    let\n\n\tfoo";
    let mut tokens = Tokens::new(input);

    span!(tokens, [0, 4], [0, 7]); // let
    span!(tokens, [2, 1], [2, 4]); // foo
}