use crate::lex::span::{Span, Spanned};
use std::{collections::HashSet, iter::Peekable, str::Bytes};

pub type RawTokenSpan<'a> = (RawToken<'a>, Span);

#[doc(hidden)]
impl Spanned for RawTokenSpan<'_> {
    fn span(&self) -> Span {
        self.1
    }
}

/// Emitted tokens.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RawToken<'a> {
    /// Tokenized keyword.
    Keyword(&'a str),
    /// Tokenized identifier.
    Ident(&'a str),
    /// Tokens literal.
    /// Quoted strings & numeric values.
    Lit(&'a str),
    /// Unexpected byte.
    Unexpected(u8),
    /// End of file.
    Eof,
}

impl std::fmt::Display for RawToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawToken::Keyword(s) => s.fmt(f),
            RawToken::Ident(s) => s.fmt(f),
            RawToken::Lit(s) => s.fmt(f),
            RawToken::Unexpected(s) => s.fmt(f),
            RawToken::Eof => Ok(()),
        }
    }
}

impl RawToken<'_> {
    pub fn is_kword(&self) -> bool {
        matches!(self, RawToken::Keyword(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, RawToken::Ident(_))
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, RawToken::Lit(_))
    }

    pub fn is_unexpected(&self) -> bool {
        matches!(self, RawToken::Unexpected(_))
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, RawToken::Eof)
    }
}

/// Input string Tokens.
pub struct Tokens<'a> {
    ended: bool,
    kwords: HashSet<String>,
    offset: usize,
    input: &'a str,
    chars: Peekable<Bytes<'a>>,
    line: usize,
    line_offset: usize,
}

impl<'a> Tokens<'a> {
    /// Create new Tokens.
    pub fn new(input: &'a str, kwords: HashSet<String>) -> Self {
        let chars = input.bytes().peekable();
        Self { ended: false,
               kwords,
               offset: 0,
               input,
               chars,
               line: 0,
               line_offset: 0 }
    }

    fn comment_ahead(&self) -> bool {
        self.input[self.offset..].starts_with("//")
    }

    fn whitespace_ahead(&self) -> bool {
        self.input[self.offset..].bytes()
                                 .next()
                                 .map(|c| c.is_ascii_whitespace())
                                 .unwrap_or(false)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(b) if b.is_ascii_whitespace() => {
                    self.next_char().unwrap();
                }
                _ => break,
            }
        }
    }

    fn peek_char(&mut self) -> Option<&u8> {
        self.chars.peek()
    }

    fn next_char(&mut self) -> Option<u8> {
        match self.chars.next() {
            Some(c) => {
                self.offset += 1;
                self.update_cursor(c);
                Some(c)
            }
            _ => None,
        }
    }
    fn update_cursor(&mut self, b: u8) {
        if b == b'\n' {
            self.line += 1;
            self.line_offset = 0;
        } else {
            self.line_offset += 1;
        }
    }

    fn cursor(&self) -> [usize; 2] {
        [self.line, self.line_offset]
    }

    fn skip_comment(&mut self) {
        loop {
            match self.peek_char() {
                Some(b'\n') => {
                    self.next_char().unwrap();
                    break;
                }
                Some(_) => {
                    self.next_char().unwrap();
                }
                // EOF
                None => break,
            }
        }
    }

    fn next_token(&mut self) -> Option<RawTokenSpan<'a>> {
        if self.ended {
            return None;
        }
        use RawToken::{Eof, Lit};

        // strip whitespace comments
        // comments begin with the sequence "//"
        while self.whitespace_ahead() || self.comment_ahead() {
            if self.whitespace_ahead() {
                self.skip_whitespace()
            }
            if self.comment_ahead() {
                self.skip_comment()
            }
        }

        match self.peek_char() {
            None => {
                self.ended = true;
                Some((Eof,
                      Span { min: self.cursor(),
                             max: self.cursor() }))
            }
            /* str lit */
            Some(b'"') => {
                let min = self.cursor();
                let lit = self.next_str_lit();
                let max = self.cursor();
                Some((Lit(lit), Span { min, max }))
            }
            /* num lit (decimal) */
            Some(b) if b.is_ascii_digit() && *b != b'0' => {
                let min = self.cursor();
                let lit = self.next_num_lit();
                let max = self.cursor();
                Some(((Lit(lit)), Span { min, max }))
            }
            /* ident | keyword | num lit (hex) */ _ => Some(self.next_ident_kword_hex_lit()),
        }
    }

    fn next_str_lit(&mut self) -> &'a str {
        let cursor = self.offset;
        assert_eq!(Some(b'"'), self.next_char());
        loop {
            let c = self.next_char();
            match c {
                Some(b'"') => return &self.input[cursor..self.offset],
                None => panic!("EOF"),
                _ => {}
            }
        }
    }

    fn next_num_lit(&mut self) -> &'a str {
        let cursor = self.offset;
        loop {
            match self.peek_char() {
                Some(b) if b.is_ascii_digit() => {
                    self.next_char().unwrap();
                }
                _ => break,
            }
        }
        &self.input[cursor..self.offset]
    }

    fn next_ident_kword_hex_lit(&mut self) -> RawTokenSpan<'a> {
        match self.peek_char() {
            /* ident | kword */
            Some(b) if b.is_ascii_alphanumeric() || *b == b'_' => {
                let min = self.cursor();
                let token = self.next_ident_kword_hex_lit_2();
                let max = self.cursor();
                (token, Span { min, max })
            }
            /* kword */
            Some(_) => {
                let min = self.cursor();
                let keyword = self.next_kword();
                let max = self.cursor();
                (keyword, Span { min, max })
            }
            None => panic!("EOF"),
        }
    }

    fn next_ident_kword_hex_lit_2(&mut self) -> RawToken<'a> {
        let cursor = self.offset;
        loop {
            match self.peek_char() {
                Some(b) if b.is_ascii_alphanumeric() || *b == b'_' => {
                    self.next_char().unwrap();
                }
                _ => break,
            }
        }
        let token_str = &self.input[cursor..self.offset];
        let token = token_str;
        if self.has_kword(token) {
            RawToken::Keyword(token)
        } else {
            // TODO edge cases
            // 01234 -> 0 prefix numbers (octal in most languages)
            if token_str.bytes().all(|b| b.is_ascii_digit())
               || token_str.starts_with("0x")
                  && token_str[2..].bytes().all(|b| b.is_ascii_hexdigit())
            {
                RawToken::Lit(token)
            } else {
                RawToken::Ident(token)
            }
        }
    }

    // keyword with non-alphanumeric nor _ characters
    // FIXME cursor bug
    fn next_kword(&mut self) -> RawToken<'a> {
        let mut offset = self.offset;
        let mut line = self.line;
        let mut line_offset = self.line_offset;
        let mut chars = self.chars.clone();

        let mut keyword = &self.input[offset..offset];

        let cursor = self.offset;
        loop {
            match self.peek_char() {
                None => break,
                Some(b) if b.is_ascii_alphanumeric() | b.is_ascii_whitespace() => break,
                Some(_) => {
                    self.next_char().unwrap();

                    let prefix = &self.input[cursor..self.offset];
                    // keep the longest possible keyword
                    if self.has_kword(prefix) {
                        chars = self.chars.clone();
                        offset = self.offset;
                        line = self.line;
                        line_offset = self.line_offset;
                        keyword = prefix;
                    } else if !self.has_prefix(prefix) {
                        break;
                    }
                }
            }
        }

        if keyword.is_empty() {
            let byte = keyword.as_bytes()[self.offset];
            RawToken::Unexpected(byte)
        } else {
            self.chars = chars;
            self.offset = offset;
            self.line = line;
            self.line_offset = line_offset;
            RawToken::Keyword(keyword)
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
    type Item = RawTokenSpan<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod test {
    use crate::lex::raw::{RawToken, Tokens};
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
        use RawToken::{Eof, Lit};

        let input = " \"hello, world\"\t42   \r\n\n";
        let mut tokens = Tokens::new(input, HashSet::new());

        assert_eq!(Some(Lit("\"hello, world\"")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Lit("42")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn lit_numeric_hex() {
        use RawToken::{Eof, Lit};

        let input = "42 0x42 0x123456789abcdef";
        let mut tokens = Tokens::new(input, HashSet::new());

        assert_eq!(Some(Lit("42")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Lit("0x42")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Lit("0x123456789abcdef")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn match_longest_token() {
        use RawToken::{Eof, Keyword};

        let input = "=====";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("==")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("==")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("=")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn tokens_non_alphanumeric() {
        use RawToken::{Eof, Keyword};

        let input = "->>==>: ::;\n\t";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("->")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(">=")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("=>")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(":")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("::")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(";")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn tokens_alphanumeric() {
        use RawToken::{Eof, Ident, Keyword};

        let input = "if else let fn loop loops _if";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("if")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("else")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("let")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("fn")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("loop")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("loops")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("_if")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }

    #[test]
    fn test() {
        use RawToken::{Eof, Ident, Keyword, Lit};

        let input = "let foo\t=42; if foo == \"hello\" { } else { }";
        let mut tokens = Tokens::new(input, rust_kwords());

        assert_eq!(Some(Keyword("let")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("foo")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("=")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Lit("42")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword(";")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("if")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Ident("foo")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("==")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Lit("\"hello\"")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("{")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("}")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("else")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("{")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Keyword("}")), tokens.next().map(|t| t.0));
        assert_eq!(Some(Eof), tokens.next().map(|t| t.0));
        assert_eq!(None, tokens.next().map(|t| t.0));
    }
}
