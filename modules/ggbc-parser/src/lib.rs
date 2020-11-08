#![deny(clippy::all,
        clippy::doc_markdown,
        clippy::dbg_macro,
        clippy::todo,
        clippy::empty_enum,
        clippy::enum_glob_use,
        clippy::pub_enum_variant_names,
        clippy::mem_forget,
        clippy::use_self,
        clippy::filter_map_next,
        clippy::needless_continue,
        clippy::needless_borrow,
        unused,
        rust_2018_idioms,
        future_incompatible,
        nonstandard_style)]

//! Parser for the `GGB` (Great Game Boy) programming language.
//!
//! Tools to perform [syntax analysis] on input source code.
//!
//! This is part of the `GGBC` (Great Game Boy Compiler) toolchain.
//!
//! [syntax analysis]: https://en.wikipedia.org/wiki/Syntax_(programming_languages)

pub mod ast;
pub mod error;
pub mod lex;

#[doc(inline)]
pub use ast::{parse, parse_with_context, Ast, ContextBuilder};
