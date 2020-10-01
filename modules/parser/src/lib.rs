use crate::{ast::Grammar, lex::Tokens};
pub use crate::{
    ast::{Context, Program},
    error::Error,
};

#[macro_use]
mod macros;

pub mod ast;
pub mod error;
#[cfg(feature = "ir")]
pub mod ir;
pub mod lex;

#[doc(inline)]
pub use crate::ast::{parse, parse_with_context};
#[doc(inline)]
#[cfg(feature = "ir")]
pub use crate::ir::compile;
