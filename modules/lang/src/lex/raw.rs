use crate::lex::span::Span;
use std::{borrow::Cow, collections::HashSet, iter::Peekable, str::Bytes};

pub type TokenSpan<'a> = (Token<'a>, Span);

/// Emitted tokens.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Token<'a> {
    /// Tokenized keyword.
    Keyword(Cow<'a, str>),
    /// Tokenized identifier.
    Ident(Cow<'a, str>),
    /// Tokens literal.
    /// Quoted strings & numeric values.
    Lit(Cow<'a, str>),
    /// Unexpected byte.
    Unexpected(u8),
    LineBreak,
    /// End of file.
    Eof,
}

impl Token<'_> {
    pub fn is_kword(&self) -> bool {
        match self {
            Token::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false,
        }
    }

    pub fn is_lit(&self) -> bool {
        match self {
            Token::Lit(_) => true,
            _ => false,
        }
    }

    pub fn is_unexpected(&self) -> bool {
        match self {
            Token::Unexpected(_) => true,
            _ => false,
        }
    }

    pub fn is_eof(&self) -> bool {
        match self {
            Token::Eof => true,
            _ => false,
        }
    }
}

/// Input string Tokens.
pub struct Tokens<'a> {
    ended: bool,
    kwords: HashSet<String>,
    offset: usize,
    input: &'a str,
    chars: Peekable<Bytes<'a>>,
}

impl<'a> Tokens<'a> {
    /// Create new Tokens.
    pub fn new(input: &'a str, kwords: HashSet<String>) -> Self {
        let chars = input.bytes().peekable();
        Self {
            ended: false,
            kwords,
            offset: 0,
            input,
            chars,
        }
    }

    fn next_token(&mut self) -> Option<TokenSpan<'a>> {
        if self.ended {
            return None;
        }
        use Token::*;
        self.skip_whitespace();
        match self.chars.peek() {
            None => {
                self.ended = true;
                Some((Eof, Span::zero()))
            }
            // Some(b'\n') => {
            //     assert_eq!(Some(b'\n'), self.chars.next());
            //     Some((LineBreak, Span::zero()))
            // }
            /* str lit */
            Some(b'"') => Some((Lit(Cow::Borrowed(self.next_str_lit())), Span::zero())),
            /* num lit */
            Some(b) if b.is_ascii_digit() => Some(((Lit(self.next_num_lit())), Span::zero())),
            /* ident | keyword */ _ => Some(self.next_ident_kword()),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                //Some(b'\n') => return,
                Some(b) if b.is_ascii_whitespace() => {
                    self.chars.next().unwrap();
                    self.offset += 1;
                }
                _ => break,
            }
        }
    }

    fn next_str_lit(&mut self) -> &'a str {
        assert_eq!(Some(b'"'), self.chars.next());
        let cursor = self.offset;
        self.offset += 1;
        loop {
            let c = self.chars.next();
            self.offset += 1;
            match c {
                Some(b'"') => return &self.input[cursor..self.offset],
                None => panic!("EOF"),
                _ => {}
            }
        }
    }

    fn next_num_lit(&mut self) -> Cow<'a, str> {
        let cursor = self.offset;
        loop {
            match self.chars.peek() {
                Some(b) if b.is_ascii_digit() => {
                    self.chars.next().unwrap();
                    self.offset += 1;
                }
                _ => break,
            }
        }
        Cow::Borrowed(&self.input[cursor..self.offset])
    }

    fn next_ident_kword(&mut self) -> TokenSpan<'a> {
        match self.chars.peek() {
            /* ident | kword */
            Some(b) if b.is_ascii_alphanumeric() || *b == b'_' => {
                (self.next_ident_kword_2(), Span::zero())
            }
            /* kword */ Some(_) => (self.next_kword(), Span::zero()),
            None => panic!("EOF"),
        }
    }

    fn next_ident_kword_2(&mut self) -> Token<'a> {
        let cursor = self.offset;
        loop {
            match self.chars.peek() {
                Some(b) if b.is_ascii_alphanumeric() || *b == b'_' => {
                    self.chars.next().unwrap();
                    self.offset += 1;
                }
                _ => break,
            }
        }
        let token = Cow::Borrowed(&self.input[cursor..self.offset]);
        if self.has_kword(&token) {
            Token::Keyword(token)
        } else {
            Token::Ident(token)
        }
    }

    // keyword with non-alphanumeric nor _ characters
    fn next_kword(&mut self) -> Token<'a> {
        let mut chars = self.chars.clone();
        let mut offset = self.offset;
        let mut keyword = &self.input[offset..offset];

        let cursor = self.offset;
        loop {
            match self.chars.peek() {
                None => break,
                Some(b) if b.is_ascii_alphanumeric() | b.is_ascii_whitespace() => break,
                Some(_) => {
                    self.chars.next();
                    self.offset += 1;

                    let prefix = &self.input[cursor..self.offset];
                    // keep the longest possible keyword
                    if self.has_kword(prefix) {
                        chars = self.chars.clone();
                        offset = self.offset;
                        keyword = prefix;
                    } else if !self.has_prefix(prefix) {
                        break;
                    }
                }
            }
        }

        if keyword.is_empty() {
            let byte = keyword.as_bytes()[self.offset];
            self.offset += 1;
            Token::Unexpected(byte)
        } else {
            self.chars = chars;
            self.offset = offset;
            Token::Keyword(Cow::Borrowed(keyword))
        }
    }

    // trie operations.
    // maybe use an actual trie later.
    fn has_prefix(&self, pre: &str) -> bool {
        self.kwords.iter().any(|k| k.starts_with(pre))
    }

    fn has_kword(&self, k: &str) -> bool {
        self.kwords.contains(k)
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = TokenSpan<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod test {
    use crate::lex::raw::{Token, Tokens};
    use std::collections::HashSet;

    fn rust_kwords() -> HashSet<String> {
        [
            "if", "else", "let", "loop", "fn", "->", ">=", "<=", "=>", "~=", "==", "::", "~", "&",
            "|", ";", "{", "}", ",", ".", ":", "=", "(", ")", "[", "]", "<", ">", "+", "-", "/",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn lit() {
        use Token::*;

        let input = " \"hello, world\"\t42   \r\n\n";
        let mut tokens = Tokens::new(input, HashSet::new());

        assert_eq!(
            Some(Lit("\"hello, world\"".into())),
            tokens.next().map(|t| t.0)
        );
        assert_eq!(Some(Lit("42".into())), tokens.next().map(|t| t.0));
        //assert_eq!(Some(LineBreak), tokens.next().map(|t| t.0));
        //assert_eq!(Some(LineBreak), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn match_longest_token() {
        use Token::*;

        let input = "=====";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("==".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("==".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("=".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn tokens_non_alphanumeric() {
        use Token::*;

        let input = "->>==>: ::;\n\t";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("->".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(">=".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("=>".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(":".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("::".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(";".into())), tokens.next().map(|t| t.0));
        //assert_eq!(Some(LineBreak), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn tokens_alphanumeric() {
        use Token::*;

        let input = "if else let fn loop loops _if";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("if".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("else".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("let".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("fn".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("loop".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("loops".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("_if".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn test() {
        use Token::*;

        let input = "let foo\t=42; if foo == \"hello\" { } else { }";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("let".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("foo".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("=".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Lit("42".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(";".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("if".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("foo".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("==".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Lit("\"hello\"".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("{".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("}".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("else".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("{".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("}".into())), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }
}
