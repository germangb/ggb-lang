use crate::lex::{raw::TokenSpan, span::Span};

mod error;
mod raw;
pub mod span;

pub use error::Error;

const KEYWORDS: &[&str] = &[
    "..", "::", "mod", "union", "struct", "mut", "in", "enum", "use", "=", ".", ",", ":", ";", "{",
    "}", "[", "]", "(", ")", "+", "-", "*", "/", "i16", "u16", "i8", "u8",
];

pub struct Tokens<'a> {
    ended: bool,
    raw: raw::Tokens<'a>,
}

impl<'a> Tokens<'a> {
    /// Create new Tokens.
    pub fn new(input: &'a str) -> Self {
        let kwords = KEYWORDS.iter().map(|s| s.to_string()).collect();
        Self {
            ended: false,
            raw: raw::Tokens::new(input, kwords),
        }
    }

    fn next_token(&mut self) -> Option<Result<Token<'a>, Error>> {
        if self.ended {
            return None;
        }
        loop {
            match self.raw.next() {
                Some((raw::Token::LineBreak, _)) => {
                    // TODO comment support
                }
                Some(ts) if ts.0.is_unexpected() => {
                    self.ended = true;
                    return Some(Err(Error::Unexpected));
                }
                Some(ts) if ts.0.is_ident() => return Some(Ok(Token::Ident(Ident(ts)))),
                Some(ts) if ts.0.is_lit() => return Some(Ok(Token::Lit(Lit(ts)))),
                Some(ts) if ts.0.is_eof() => {
                    self.ended = true;
                    return Some(Ok(Token::Eof(Eof(ts))));
                }
                Some((raw::Token::Keyword(kw), span)) => {
                    return Some(Ok(match kw.as_bytes() {
                        b".." => Token::DotDot(DotDot((raw::Token::Keyword(kw), span))),
                        b"::" => Token::Square(Square((raw::Token::Keyword(kw), span))),

                        b"=" => Token::Assign(Assign((raw::Token::Keyword(kw), span))),
                        b"." => Token::Dot(Dot((raw::Token::Keyword(kw), span))),
                        b"," => Token::Comma(Comma((raw::Token::Keyword(kw), span))),
                        b":" => Token::Colon(Colon((raw::Token::Keyword(kw), span))),
                        b";" => Token::SemiColon(SemiColon((raw::Token::Keyword(kw), span))),
                        b"{" => Token::LeftBracket(LeftBracket((raw::Token::Keyword(kw), span))),
                        b"}" => Token::RightBracket(RightBracket((raw::Token::Keyword(kw), span))),
                        b"[" => Token::LeftSquare(LeftSquare((raw::Token::Keyword(kw), span))),
                        b"]" => Token::RightSquare(RightSquare((raw::Token::Keyword(kw), span))),
                        b"(" => Token::LeftPar(LeftPar((raw::Token::Keyword(kw), span))),
                        b")" => Token::RightPar(RightPar((raw::Token::Keyword(kw), span))),
                        b"+" => Token::Plus(Plus((raw::Token::Keyword(kw), span))),
                        b"-" => Token::Minus(Minus((raw::Token::Keyword(kw), span))),
                        b"*" => Token::Star(Star((raw::Token::Keyword(kw), span))),
                        b"/" => Token::Div(Div((raw::Token::Keyword(kw), span))),

                        b"mod" => Token::Mod(Mod((raw::Token::Keyword(kw), span))),
                        b"union" => Token::Union(Union((raw::Token::Keyword(kw), span))),
                        b"struct" => Token::Struct(Struct((raw::Token::Keyword(kw), span))),
                        b"mut" => Token::Mut(Mut((raw::Token::Keyword(kw), span))),
                        b"in" => Token::In(In((raw::Token::Keyword(kw), span))),
                        b"enum" => Token::Enum(Enum((raw::Token::Keyword(kw), span))),
                        b"use" => Token::Use(Use((raw::Token::Keyword(kw), span))),

                        b"i16" => Token::I16(I16((raw::Token::Keyword(kw), span))),
                        b"u16" => Token::U16(U16((raw::Token::Keyword(kw), span))),
                        b"i8" => Token::I8(I8((raw::Token::Keyword(kw), span))),
                        b"u8" => Token::U8(U8((raw::Token::Keyword(kw), span))),
                        _ => unreachable!(),
                    }))
                }
                None => return None,
                _ => unreachable!(),
            }
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

macro_rules! tokens {
    ($(
        $(#[$($meta:meta)+])*
        $token:ident,
    )+) => {
        $(
            $(#[$($meta)+])*
            pub struct $token<'a>(raw::TokenSpan<'a>);

            impl<'a> crate::ast::Parse<'a> for $token<'a> {
                fn parse(
                    _: &mut crate::ast::Context,
                    mut tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>,
                ) -> Result<Self, crate::ast::Error> {
                    match tokens.next() {
                        Some(Ok(Token::$token(token))) => Ok(token),
                        Some(Err(err)) => Err(crate::ast::Error::Lexer(err)),
                        _ => Err(crate::ast::Error::UnexpectedToken),
                    }
                }
            }

            impl<'a> crate::ast::Parse<'a> for Option<$token<'a>> {
                fn parse(
                    context: &mut crate::ast::Context,
                    tokens: &mut std::iter::Peekable<crate::lex::Tokens<'a>>,
                ) -> Result<Self, crate::ast::Error> {
                    match tokens.peek() {
                        Some(Ok(Token::$token(token))) => Ok(Some(crate::ast::Parse::parse(context, tokens)?)),
                        None | Some(Ok(_)) => Ok(None),
                        Some(Err(_)) => {
                            let err = tokens.next().unwrap().err().unwrap();
                            Err(crate::ast::Error::Lexer(err))
                        }
                    }
                }
            }
        )+

        pub enum Token<'a> {
            $($token($token<'a>),)+
        }
    }
}

tokens! {
    // two chars

    /// `..`
    DotDot,
    /// `::`
    Square,

    // single char

    /// `=`
    Assign,
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `;`
    SemiColon,
    /// `{`
    LeftBracket,
    /// `}`
    RightBracket,
    /// `[`
    LeftSquare,
    /// `]`
    RightSquare,
    /// `(`
    LeftPar,
    /// `)`
    RightPar,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Div,

    // alphanumeric

    /// `mod`
    Mod,
    /// `union`
    Union,
    /// `struct`
    Struct,
    /// `mut`
    Mut,
    /// `in`
    In,
    /// `enum`
    Enum,
    /// `use`
    Use,

    // types

    /// `i16`
    I16,
    /// `u16`
    U16,
    /// `i8`
    I8,
    /// `u8`
    U8,

    // variables

    Ident,
    Lit,

    // others

    /// `EOF`
    Eof,
}
