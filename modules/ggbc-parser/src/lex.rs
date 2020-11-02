//! Token definitions and lexical analysis.
use crate::{
    error::Error,
    lex::{raw::TokenSpan, span::Span},
};

#[macro_use]
mod macros;
mod raw;
pub mod span;

const KEYWORDS: &[&str] = &[// two-char tokens
                            "!!", "..", "::", "+=", "-=", "*=", "/=", "&=", "|=", "^=", "<<", ">>",
                            "==", "~=", ">=", "<=", // longer tokens
                            "<<=", ">>=", // keywords
                            "mod", "union", "struct", "mut", "in", "enum", "use", "asm", "static",
                            "const", "pub", "for", "loop", "let", "fn", "if", "else", "continue",
                            "break", "return", // single-char tokens
                            "=", ".", ",", ":", ";", "{", "}", "[", "]", "(", ")", "+", "-", "*",
                            "/", "&", "|", "^", "~", ">", "<", "@", // base types
                            "u16", "u8", "i8", // asm registers
                            "%a", "%f", "%af", "%b", "%c", "%bc", "%d", "%e", "%de", "%h", "%l",
                            "%hl", "%sp", "%pc",
                            // asm instructions
                            // misc/control
                            ".nop", ".stop", ".halt", ".di", ".ei", // load/store/move
                            ".ld", ".ldh", ".push", ".pop", // arithmetic
                            ".inc", ".dec", ".daa", ".scf", ".cpl", ".ccf", ".add", ".adc",
                            ".sub", ".sbc", ".and", ".xor", ".or", ".cp"];

tokens! {
    // two character tokens

    /// `!!`
    BangBang,
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
    /// `/=`
    SlashAssign,
    /// `&=`
    AmpersandAssign,
    /// `|=`
    PipeAssign,
    /// `^=`
    CaretAssign,
    /// `<<`
    LessLess,
    /// `>>`
    GreatGreat,
    /// `==`
    Eq,
    /// `~=`
    TildeEq,
    /// `<=`
    LessEq,
    /// `>=`
    GreaterEq,

    // single character tokens

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
    Slash,
    /// `&`
    Ampersand,
    /// `|`
    Pipe,
    /// `^`
    Caret,
    /// `~`
    Tilde,
    /// `@`
    At,
    /// `>`
    Greater,
    /// `<`
    Less,

    // alphanumeric tokens

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
    /// `fn`
    Fn,
    /// `if`
    If,
    /// `else`
    Else,
    /// `continue`
    Continue,
    /// `break`
    Break,
    /// `return`
    Return,

    // types

    /// `u8`
    U8,
    /// `i8`
    I8,

    // variables

    Ident,
    Lit,

    // misc tokens

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
    /// `.ldh`
    Ldh,
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

impl Ident<'_> {
    pub fn as_str(&self) -> &str {
        if let raw::Token::Ident(ident) = &(self.0).0 {
            ident.as_ref()
        } else {
            unreachable!()
        }
    }
}

impl Lit<'_> {
    pub fn as_str(&self) -> &str {
        if let raw::Token::Lit(lit) = &(self.0).0 {
            lit.as_ref()
        } else {
            unreachable!()
        }
    }
}

pub struct Tokens<'a> {
    ended: bool,
    raw: raw::Tokens<'a>,
}

impl<'a> Tokens<'a> {
    /// Create new Tokens.
    pub fn new(input: &'a str) -> Self {
        let kwords = KEYWORDS.iter().map(|s| s.to_string()).collect();
        Self { ended: false,
               raw: raw::Tokens::new(input, kwords) }
    }

    fn next_token(&mut self) -> Option<Result<Token<'a>, Error<'a>>> {
        if self.ended {
            return None;
        }
        loop {
            match self.raw.next() {
                Some(ts) if ts.0.is_unexpected() => {
                    // TODO
                    self.ended = true;
                    return Some(Err(Error::UnexpectedByte { byte: 0,
                                                            span: ts.1 }));
                }
                Some(ts) if ts.0.is_ident() => return Some(Ok(Token::Ident(Ident(ts)))),
                Some(ts) if ts.0.is_lit() => return Some(Ok(Token::Lit(Lit(ts)))),
                Some(ts) if ts.0.is_eof() => {
                    self.ended = true;
                    return Some(Ok(Token::Eof(Eof(ts))));
                }
                Some((raw::Token::Keyword(keyword), span)) => {
                    #[rustfmt::skip]
                        return Some(Ok(match keyword.as_bytes() {
                        // 2 char keywords
                        b"!!" => Token::BangBang(BangBang((raw::Token::Keyword(keyword), span))),
                        b".." => Token::DotDot(DotDot((raw::Token::Keyword(keyword), span))),
                        b"::" => Token::Square(Square((raw::Token::Keyword(keyword), span))),
                        b"+=" => Token::PlusAssign(PlusAssign((raw::Token::Keyword(keyword), span))),
                        b"-=" => Token::MinusAssign(MinusAssign((raw::Token::Keyword(keyword), span))),
                        b"*=" => Token::StarAssign(StarAssign((raw::Token::Keyword(keyword), span))),
                        b"/=" => Token::SlashAssign(SlashAssign((raw::Token::Keyword(keyword), span))),
                        b"&=" => Token::AmpersandAssign(AmpersandAssign((raw::Token::Keyword(keyword), span))),
                        b"|=" => Token::PipeAssign(PipeAssign((raw::Token::Keyword(keyword), span))),
                        b"^=" => Token::CaretAssign(CaretAssign((raw::Token::Keyword(keyword), span))),
                        b"<<" => Token::LessLess(LessLess((raw::Token::Keyword(keyword), span))),
                        b">>" => Token::GreatGreat(GreatGreat((raw::Token::Keyword(keyword), span))),
                        b"==" => Token::Eq(Eq((raw::Token::Keyword(keyword), span))),
                        b"~=" => Token::TildeEq(TildeEq((raw::Token::Keyword(keyword), span))),
                        b"<=" => Token::LessEq(LessEq((raw::Token::Keyword(keyword), span))),
                        b">=" => Token::GreaterEq(GreaterEq((raw::Token::Keyword(keyword), span))),

                        // 1 char keywords
                        b"=" => Token::Assign(Assign((raw::Token::Keyword(keyword), span))),
                        b"." => Token::Dot(Dot((raw::Token::Keyword(keyword), span))),
                        b"," => Token::Comma(Comma((raw::Token::Keyword(keyword), span))),
                        b":" => Token::Colon(Colon((raw::Token::Keyword(keyword), span))),
                        b";" => Token::SemiColon(SemiColon((raw::Token::Keyword(keyword), span))),
                        b"{" => Token::LeftBracket(LeftBracket((raw::Token::Keyword(keyword), span))),
                        b"}" => Token::RightBracket(RightBracket((raw::Token::Keyword(keyword), span))),
                        b"[" => Token::LeftSquare(LeftSquare((raw::Token::Keyword(keyword), span))),
                        b"]" => Token::RightSquare(RightSquare((raw::Token::Keyword(keyword), span))),
                        b"(" => Token::LeftPar(LeftPar((raw::Token::Keyword(keyword), span))),
                        b")" => Token::RightPar(RightPar((raw::Token::Keyword(keyword), span))),
                        b"+" => Token::Plus(Plus((raw::Token::Keyword(keyword), span))),
                        b"-" => Token::Minus(Minus((raw::Token::Keyword(keyword), span))),
                        b"*" => Token::Star(Star((raw::Token::Keyword(keyword), span))),
                        b"/" => Token::Slash(Slash((raw::Token::Keyword(keyword), span))),
                        b"&" => Token::Ampersand(Ampersand((raw::Token::Keyword(keyword), span))),
                        b"|" => Token::Pipe(Pipe((raw::Token::Keyword(keyword), span))),
                        b"^" => Token::Caret(Caret((raw::Token::Keyword(keyword), span))),
                        b"~" => Token::Tilde(Tilde((raw::Token::Keyword(keyword), span))),
                        b"@" => Token::At(At((raw::Token::Keyword(keyword), span))),
                        b">" => Token::Greater(Greater((raw::Token::Keyword(keyword), span))),
                        b"<" => Token::Less(Less((raw::Token::Keyword(keyword), span))),

                        // alphanum keywords
                        b"mod" => Token::Mod(Mod((raw::Token::Keyword(keyword), span))),
                        b"union" => Token::Union(Union((raw::Token::Keyword(keyword), span))),
                        b"struct" => Token::Struct(Struct((raw::Token::Keyword(keyword), span))),
                        b"mut" => Token::Mut(Mut((raw::Token::Keyword(keyword), span))),
                        b"in" => Token::In(In((raw::Token::Keyword(keyword), span))),
                        b"enum" => Token::Enum(Enum((raw::Token::Keyword(keyword), span))),
                        b"use" => Token::Use(Use((raw::Token::Keyword(keyword), span))),
                        b"asm" => Token::Asm(Asm((raw::Token::Keyword(keyword), span))),
                        b"static" => Token::Static(Static((raw::Token::Keyword(keyword), span))),
                        b"const" => Token::Const(Const((raw::Token::Keyword(keyword), span))),
                        b"pub" => Token::Pub(Pub((raw::Token::Keyword(keyword), span))),
                        b"for" => Token::For(For((raw::Token::Keyword(keyword), span))),
                        b"loop" => Token::Loop(Loop((raw::Token::Keyword(keyword), span))),
                        b"let" => Token::Let(Let((raw::Token::Keyword(keyword), span))),
                        b"fn" => Token::Fn(Fn((raw::Token::Keyword(keyword), span))),
                        b"if" => Token::If(If((raw::Token::Keyword(keyword), span))),
                        b"else" => Token::Else(Else((raw::Token::Keyword(keyword), span))),
                        b"continue" => Token::Continue(Continue((raw::Token::Keyword(keyword), span))),
                        b"break" => Token::Break(Break((raw::Token::Keyword(keyword), span))),
                        b"return" => Token::Return(Return((raw::Token::Keyword(keyword), span))),

                        // types
                        b"u8" => Token::U8(U8((raw::Token::Keyword(keyword), span))),
                        b"i8" => Token::I8(I8((raw::Token::Keyword(keyword), span))),
                        _ => {
                            self.ended = true;
                            return Some(Err(Error::ReservedKeyword { key_word: keyword, span }));
                        }
                    }));
                }
                None => return None,
                _ => unreachable!(),
            }
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
