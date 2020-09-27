use crate::lex::{raw::TokenSpan, span::Span};

mod error;
mod raw;
pub mod span;

pub use error::Error;

const KEYWORDS: &[&str] = &[
    // two-char tokens
    "..", "::", "+=", "-=", "*=", "&=", "|=", "^=", // keywords
    "mod", "union", "struct", "mut", "in", "enum", "use", "asm", "static", "const", "pub", "for",
    "loop", "let", // single-char tokens
    "=", ".", ",", ":", ";", "{", "}", "[", "]", "(", ")", "+", "-", "*", "/", "&", "|", "^", "@",
    // base types
    "i16", "u16", "i8", "u8", // asm registers
    "%a", "%f", "%af", "%b", "%c", "%bc", "%d", "%e", "%de", "%h", "%l", "%hl", "%sp", "%pc",
    // asm instructions
    // misc/control
    ".nop", ".stop", ".halt", ".di", ".ei", // load/store/move
    ".ld", ".push", ".pop", // arithmetic
    ".inc", ".dec", ".daa", ".scf", ".cpl", ".ccf", ".add", ".adc", ".sub", ".sbc", ".and", ".xor",
    ".or", ".cp",
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
                        b"+=" => Token::PlusAssign(PlusAssign((raw::Token::Keyword(kw), span))),
                        b"-=" => Token::MinusAssign(MinusAssign((raw::Token::Keyword(kw), span))),
                        b"*=" => Token::StarAssign(StarAssign((raw::Token::Keyword(kw), span))),
                        b"&=" => {
                            Token::AmpersandAssign(AmpersandAssign((raw::Token::Keyword(kw), span)))
                        }
                        b"|=" => Token::PipeAssign(PipeAssign((raw::Token::Keyword(kw), span))),
                        b"^=" => Token::CaretAssign(CaretAssign((raw::Token::Keyword(kw), span))),

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
                        b"&" => Token::Ampersand(Ampersand((raw::Token::Keyword(kw), span))),
                        b"|" => Token::Pipe(Pipe((raw::Token::Keyword(kw), span))),
                        b"^" => Token::Caret(Caret((raw::Token::Keyword(kw), span))),
                        b"@" => Token::At(At((raw::Token::Keyword(kw), span))),

                        b"mod" => Token::Mod(Mod((raw::Token::Keyword(kw), span))),
                        b"union" => Token::Union(Union((raw::Token::Keyword(kw), span))),
                        b"struct" => Token::Struct(Struct((raw::Token::Keyword(kw), span))),
                        b"mut" => Token::Mut(Mut((raw::Token::Keyword(kw), span))),
                        b"in" => Token::In(In((raw::Token::Keyword(kw), span))),
                        b"enum" => Token::Enum(Enum((raw::Token::Keyword(kw), span))),
                        b"use" => Token::Use(Use((raw::Token::Keyword(kw), span))),
                        b"asm" => Token::Asm(Asm((raw::Token::Keyword(kw), span))),
                        b"static" => Token::Static(Static((raw::Token::Keyword(kw), span))),
                        b"const" => Token::Const(Const((raw::Token::Keyword(kw), span))),
                        b"pub" => Token::Pub(Pub((raw::Token::Keyword(kw), span))),
                        b"for" => Token::For(For((raw::Token::Keyword(kw), span))),
                        b"loop" => Token::Loop(Loop((raw::Token::Keyword(kw), span))),
                        b"let" => Token::Let(Let((raw::Token::Keyword(kw), span))),

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
            #[derive(Debug)]
            pub struct $token<'a>(raw::TokenSpan<'a>);

            impl std::fmt::Display for $token<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    (self.0).0.fmt(f)
                }
            }

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
    /// `+=`
    PlusAssign,
    /// `-=`
    MinusAssign,
    /// `*=`
    StarAssign,
    /// `&=`
    AmpersandAssign,
    /// `|=`
    PipeAssign,
    /// `^=`
    CaretAssign,

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
    /// `&`
    Ampersand,
    /// `|`
    Pipe,
    /// `^`
    Caret,
    /// `@`
    At,

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
    /// `asm`
    Asm,
    /// `static`
    Static,
    /// `const`
    Const,
    /// `pub`
    Pub,
    /// `for`
    For,
    /// `loop`
    Loop,
    /// `let`
    Let,

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

    // asm registers

    /// `%a`
    A,
    /// `%f`
    F,
    /// `%af`
    AF,
    /// `%b`
    B,
    /// `%c`
    C,
    /// `%bc`
    BC,
    /// `%d`
    D,
    /// `%e`
    E,
    /// `%de`
    DE,
    /// `%h`
    H,
    /// `%L`
    L,
    /// `%hl`
    HL,
    /// `%sp`
    SP,
    /// `%pc`
    PC,

    // asm misc/control

    /// `.nop`
    Nop,
    /// `.stop`
    Stop,
    /// `.halt`
    Halt,
    /// `.di`
    Di,
    /// `.ei`
    Ei,

    /// `.ld`
    Ld,
    /// `.push`
    Push,
    /// `.pop`
    Pop,

    /// `.inc`
    Inc,
    /// `.dec`
    Dec,
    /// `.daa`
    Daa,
    /// `.scf`
    Scf,
    /// `.cpl`
    Cpl,
    /// `.ccf`
    Ccf,
    /// `.add`
    Add,
    /// `.adc`
    Adc,
    /// `.sub`
    Sub,
    /// `.sbc`
    Sbc,
    /// `.and`
    And,
    /// `.xor`
    Xor,
    /// `.or`
    Or,
    /// `.cp`
    Cp,
}
