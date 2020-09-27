use crate::{ast::Parse, lex};

/// Marker trait.
pub trait ParseAsm<'a>: Parse<'a> {}

impl<'a> ParseAsm<'a> for Address<'a> {}

pub enum Asm<'a> {
    Address(Address<'a>),
}

parse! {
    /// `ident :`
    pub struct Address<'a> {
        pub ident: lex::Ident<'a>,
        pub colon: lex::Colon<'a>,
    }
}
