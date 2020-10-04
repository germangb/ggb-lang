use crate::{
    ast::{Statement, Type},
    error::Error,
    lex,
};
use std::{borrow::Cow, collections::HashSet};

#[derive(Default, Debug)]
pub struct ContextBuilder {}

impl ContextBuilder {
    pub fn build<'a, 'b>(self) -> Context<'a, 'b> {
        Context {
            level: 0,
            parent: None,
        }
    }
}

pub struct Context<'a, 'b> {
    level: usize,
    parent: Option<&'b mut Self>,
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

    pub(super) fn push(&'a mut self) -> Self {
        Self {
            level: self.level + 1,
            parent: Some(self),
        }
    }

    pub(super) fn pop(mut self) {
        assert_ne!(0, self.level);
        //let parent_ctx = self.parent.unwrap();
        //parent_ctx.visit = self.visit.take();
    }
}

impl Drop for Context<'_, '_> {
    fn drop(&mut self) {
        if self.level != 0 {
            panic!("Did you forget to call the 'pop' method on this context?");
        }
    }
}
