//! Token definitions and lexical analysis.
use crate::{error::Error, lex::span::Span};

#[macro_use]
mod macros;
mod raw;
pub mod span;

pub struct Tokens<'a> {
    ended: bool,
    raw: raw::Tokens<'a>,
}

tokens! {
    // two character tokens

    /// `!!`
    "!!" => BangBang,

    /// `..`
    ".." => DotDot,

    /// `::`
    "::" => Square,

    /// `+=`
    "+=" => PlusAssign,

    /// `-=`
    "-=" => MinusAssign,

    /// `*=`
    "*=" => StarAssign,

    /// `/=`
    "/=" => SlashAssign,

    /// `&=`
    "&=" => AmpersandAssign,

    /// `|=`
    "|=" => PipeAssign,

    /// `^=`
    "^=" => CaretAssign,

    /// `<<`
    "<<" => LessLess,

    /// `>>`
    ">>" => GreatGreat,

    /// `==`
    "==" => Eq,

    /// `~=`
    "~=" => TildeEq,

    /// `<=`
    "<=" => LessEq,

    /// `>=`
    ">=" => GreaterEq,

    // single character tokens

    /// `=`
    "=" => Assign,

    /// `.`
    "." => Dot,

    /// `,`
    "," => Comma,

    /// `:`
    ":" => Colon,

    /// `;`
    ";" => SemiColon,

    /// `{`
    "{" => LeftBracket,

    /// `}`
    "}" => RightBracket,

    /// `[`
    "[" => LeftSquare,

    /// `]`
    "]" => RightSquare,

    /// `(`
    "(" => LeftPar,

    /// `)`
    ")" => RightPar,

    /// `+`
    "+" => Plus,

    /// `-`
    "-" => Minus,

    /// `*`
    "*" => Star,

    /// `/`
    "/" => Slash,

    /// `&`
    "&" => Ampersand,

    /// `|`
    "|" => Pipe,

    /// `^`
    "^" => Caret,

    /// `~`
    "~" => Tilde,

    /// `@`
    "@" => At,

    /// `>`
    ">" => Greater,

    /// `<`
    "<" => Less,

    // alphanumeric tokens

    /// `mod`
    "mod" => Mod,

    /// `union`
    "union" => Union,

    /// `struct`
    "struct" => Struct,

    /// `mut`
    "mut" => Mut,

    /// `in`
    "in" => In,

    /// `enum`
    "enum" => Enum,

    /// `use`
    "use" => Use,

    /// `asm`
    "asm" => Asm,

    /// `static`
    "static" => Static,

    /// `const`
    "const" => Const,

    /// `pub`
    "pub" => Pub,

    /// `for`
    "for" => For,

    /// `loop`
    "loop" => Loop,

    /// `let`
    "let" => Let,

    /// `fn`
    "fn" => Fn,

    /// `if`
    "if" => If,

    /// `else`
    "else" => Else,

    /// `continue`
    "continue" => Continue,

    /// `break`
    "break" => Break,

    /// `return`
    "return" => Return,

    /// `in`
    "read" => Read,

    /// `write`
    "write" => Write,

    // types

    /// `u8`
    "u8" => U8,

    /// `i8`
    "i8" => I8,

    // asm registers

    /// `%a`
    "%a" => A,

    /// `%f`
    "%f" => F,

    /// `%af`
    "%af" => AF,

    /// `%b`
    "%b" => B,

    /// `%c`
    "%c" => C,

    /// `%bc`
    "%bc" => BC,

    /// `%d`
    "%d" => D,

    /// `%e`
    "%e" => E,

    /// `%de`
    "%de" => DE,

    /// `%h`
    "%h" => H,

    /// `%L`
    "%l" => L,

    /// `%hl`
    "%hl" => HL,

    /// `%sp`
    "%sp" => SP,

    /// `%pc`
    "%pc" => PC,

    // asm misc/control

    /// `.nop`
    ".nop" => Nop,

    /// `.stop`
    ".stop" => Stop,

    /// `.halt`
    ".halt" => Halt,

    /// `.di`
    ".di" => Di,

    /// `.ei`
    ".ei" => Ei,

    /// `.ld`
    ".ld" => Ld,

    /// `.ldh`
    ".ldh" => Ldh,

    /// `.push`
    ".push" => Push,

    /// `.pop`
    ".pop" => Pop,

    /// `.inc`
    ".inc" => Inc,

    /// `.dec`
    ".dec" => Dec,

    /// `.daa`
    ".daa" => Daa,

    /// `.scf`
    ".scf" => Scf,

    /// `.cpl`
    ".cpl" => Cpl,

    /// `.ccf`
    ".ccf" => Ccf,

    /// `.add`
    ".add" => Add,

    /// `.adc`
    ".adc" => Adc,

    /// `.sub`
    ".sub" => Sub,

    /// `.sbc`
    ".sbc" => Sbc,

    /// `.and`
    ".and" => And,

    /// `.xor`
    ".xor" => Xor,

    /// `.or`
    ".or" => Or,

    /// `.cp`
    ".cp" => Cp,

    // variables

    /// Identifier
    "" => Ident,

    /// Literal
    "" => Lit,

    // misc tokens

    /// `EOF`
    "" => Eof,
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

    fn next_token(&mut self) -> Option<Result<Token<'a>, Error<'a>>> {
        if self.ended {
            return None;
        }
        loop {
            match self.raw.next() {
                Some(ts) if ts.0.is_unexpected() => {
                    // TODO
                    self.ended = true;
                    return Some(Err(Error::UnexpectedByte {
                        byte: 0,
                        span: ts.1,
                    }));
                }
                Some(ts) if ts.0.is_ident() => return Some(Ok(Token::Ident(Ident(ts)))),
                Some(ts) if ts.0.is_lit() => return Some(Ok(Token::Lit(Lit(ts)))),
                Some(ts) if ts.0.is_eof() => {
                    self.ended = true;
                    return Some(Ok(Token::Eof(Eof(ts))));
                }
                Some((raw::RawToken::Keyword(keyword), span)) => {
                    return match_token(self, keyword, span)
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
