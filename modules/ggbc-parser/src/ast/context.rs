use crate::{ast::Path, error::Error, lex};

type Stack<T> = Vec<T>;

#[derive(Default, Debug)]
pub struct ContextBuilder {}

impl ContextBuilder {
    pub fn build<'a>(self) -> Context<'a> {
        Context {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct Context<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    pub(crate) fn is_type(&self, path: &Path<'a>) -> bool {
        false
    }
}
