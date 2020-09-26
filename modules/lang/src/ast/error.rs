#[derive(Debug)]
pub enum Error {
    /// Lexer error.
    Lexer(crate::lex::Error),
    /// Unexpected token.
    UnexpectedToken,
}
