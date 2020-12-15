use parser::lex::{Token, Tokens};

macro_rules! assert_eq_token {
    ($token_var:ident , $tokens:expr) => {
        match $tokens.next() {
            Some(Ok(Token::$token_var(_))) => {}
            token => panic!("Unexpected token: {:?}", token),
        }
    };
    ($token_var:ident ($val:expr), $tokens:expr) => {
        match $tokens.next() {
            Some(Ok(Token::$token_var(t))) => assert_eq!($val, format!("{}", t)),
            token => panic!("Unexpected token: {:?}", token),
        }
    };
}

#[test]
fn lit() {
    let input = " \"hello, world\"\t42   \r\n\n";
    let mut tokens = Tokens::new(input);

    assert_eq_token!(Lit("\"hello, world\""), tokens);
    assert_eq_token!(Lit("42"), tokens);
    assert_eq_token!(Eof, tokens);
}

#[test]
fn lit_numeric_hex() {
    let input = "42 0x42 0x123456789abcdef";
    let mut tokens = Tokens::new(input);

    assert_eq_token!(Lit("42"), tokens);
    assert_eq_token!(Lit("0x42"), tokens);
    assert_eq_token!(Lit("0x123456789abcdef"), tokens);
    assert_eq_token!(Eof, tokens);
}

#[test]
fn match_longest_token() {
    let input = "=====";
    let mut tokens = Tokens::new(input);

    assert_eq_token!(Eq, tokens);
    assert_eq_token!(Eq, tokens);
    assert_eq_token!(Assign, tokens);
    assert_eq_token!(Eof, tokens);
}

#[test]
fn tokens_non_alphanumeric() {
    let input = ">=: ::;\n\t";
    let mut tokens = Tokens::new(input);

    assert_eq_token!(GreaterEq, tokens);
    assert_eq_token!(Colon, tokens);
    assert_eq_token!(Square, tokens);
    assert_eq_token!(SemiColon, tokens);
    assert_eq_token!(Eof, tokens);
}

#[test]
fn tokens_alphanumeric() {
    let input = "if else let fn loop loops _if";
    let mut tokens = Tokens::new(input);

    assert_eq_token!(If, tokens);
    assert_eq_token!(Else, tokens);
    assert_eq_token!(Let, tokens);
    assert_eq_token!(Fn, tokens);
    assert_eq_token!(Loop, tokens);
    assert_eq_token!(Ident("loops"), tokens);
    assert_eq_token!(Ident("_if"), tokens);
    assert_eq_token!(Eof, tokens);
}

#[test]
fn test() {
    let input = "let foo\t=42; if foo == \"hello\" { } else { }";
    let mut tokens = Tokens::new(input);

    assert_eq_token!(Let, tokens);
    assert_eq_token!(Ident("foo"), tokens);
    assert_eq_token!(Assign, tokens);
    assert_eq_token!(Lit("42"), tokens);
    assert_eq_token!(SemiColon, tokens);
    assert_eq_token!(If, tokens);
    assert_eq_token!(Ident("foo"), tokens);
    assert_eq_token!(Eq, tokens);
    assert_eq_token!(Lit("\"hello\""), tokens);
    assert_eq_token!(LeftBracket, tokens);
    assert_eq_token!(RightBracket, tokens);
    assert_eq_token!(Else, tokens);
    assert_eq_token!(LeftBracket, tokens);
    assert_eq_token!(RightBracket, tokens);
    assert_eq_token!(Eof, tokens);
}
