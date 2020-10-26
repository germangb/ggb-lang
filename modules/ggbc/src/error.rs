//! Compilation errors.
use crate::{parser, target::Target};

#[derive(Debug)]
pub enum Error<'a, T: Target> {
    /// Error when parsing programs program (lexing and syntax analysis).
    Parser(parser::error::Error<'a>),
    /// Error when compiling to a given target.
    Codegen(T::Error),
}

impl<'a, T: Target> From<parser::error::Error<'a>> for Error<'a, T> {
    fn from(error: parser::error::Error<'a>) -> Self {
        Self::Parser(error)
    }
}

// conflicting implementation (too generic?)
#[cfg(FIXME)]
impl<T: Target> From<T::Error> for Error<'_, T> {
    fn from(error: T::Error) -> Self {
        Self::Codegen(error)
    }
}
