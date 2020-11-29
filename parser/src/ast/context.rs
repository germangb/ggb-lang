use crate::ast::Path;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct ContextBuilder {
    _phantom: std::marker::PhantomData<()>,
}

impl ContextBuilder {
    pub fn build<'a>(self) -> Context<'a> {
        Context {
            paths: HashSet::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[allow(unused)]
pub struct Context<'a> {
    paths: HashSet<String>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    #[allow(unused)]
    pub(crate) fn is_type(&self, path: &Path<'a>) -> bool {
        true
    }

    #[allow(unused)]
    pub(crate) fn is_defined(&self, path: &Path<'a>) -> bool {
        true
    }
}
