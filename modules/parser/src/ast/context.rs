use crate::{ast::Type, error::Error, lex};
use std::{borrow::Cow, collections::HashSet};

#[derive(Debug)]
pub struct ContextBuilder {}

enum Node<'a> {
    /// Module symbol.
    Mod {
        /// Module identifier,
        ident: lex::Ident<'a>,
        /// Nested symbold.
        nodes: Vec<Node<'a>>,
    },
    /// Typed symbol (vars, structs, static, const, etc).
    Typed {
        /// Typed symbol identifier.
        ident: lex::Ident<'a>,
        /// Type of the symbol.
        type_: Type<'a>,
    },
}

#[derive(Default)]
pub struct Context<'a, 'b> {
    level: usize,
    parent: Option<&'b Self>,
    idents: Vec<Node<'a>>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub(super) fn level(&self) -> usize {
        self.level
    }

    // Define module symbol.
    pub(super) fn define_mod(&mut self, ident: lex::Ident<'a>) {
        unimplemented!("define_mod")
    }

    // Define new symbol. For composite types (struct, unions) all nested
    // identifiers will also be added.
    pub(super) fn define_typed(&mut self, ident: lex::Ident<'a>, type_: Type<'a>) {
        unimplemented!("define_typed")
    }

    // Return the type of a given identifier of the given path, or returns an Err if
    // the identifier is resolved to either a Module or an undefined symbol.
    pub(super) fn type_of(&self, path: &()) -> Result<&Type<'a>, Error> {
        unimplemented!("type_of")
    }

    pub(super) fn push(&'a self) -> Self {
        Self {
            level: self.level + 1,
            parent: Some(self),
            idents: Vec::new(),
        }
    }

    pub(super) fn pop(self) {
        assert_ne!(0, self.level);
    }
}

impl Drop for Context<'_, '_> {
    fn drop(&mut self) {
        if self.level != 0 {
            panic!("You forgot to call pop on the scope");
        }
    }
}
