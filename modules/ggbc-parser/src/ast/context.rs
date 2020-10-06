use crate::{
    ast::{expressions::Path, Statement, Type},
    error::Error,
    lex,
};
use std::{borrow::Cow, collections::HashSet, marker::PhantomData};

#[derive(Default, Debug)]
pub struct ContextBuilder {
    // TODO implement after Context
    _phantom: PhantomData<()>,
}

impl ContextBuilder {
    pub fn build<'a>(self) -> Context<'a> {
        Context {
            level: vec![0],
            mod_: vec![None],
            mod_path: vec![Vec::new()],
            scope_local: vec![Vec::new()],
            scope_global: vec![Vec::new()],
            global: false,
        }
    }
}

type Stack<T> = Vec<T>;

// FIXME temporary
//  - Replace path path resolution with a tree
pub struct Context<'a> {
    // Scope level stack. Scopes are relative to the current module.
    level: Stack<usize>,
    // module stack,
    // None corresponds to the root module.
    mod_: Stack<Option<lex::Ident<'a>>>,
    // Current symbol path, relative to current mod,
    // used to resolve inner fields of a struct/union.
    mod_path: Stack<Vec<lex::Ident<'a>>>,
    // Paths reachable at any given time.
    // These paths aren't reachable from functions though.
    scope_local: Stack<Vec<Vec<lex::Ident<'a>>>>,
    // Paths reachable from any point of the scope,
    // examples include: functions declared on root, static, const, etc
    scope_global: Stack<Vec<Vec<lex::Ident<'a>>>>,
    // currently parsing the inner fields of a global type.
    global: bool,
}

impl<'a> Context<'a> {
    // current scope level.
    // value relative to the current module.
    // example: `mod foo { mod bar {}}`.
    // both foo and bar are at scope level = 0.
    fn peek_level(&self) -> usize {
        *self.level.last().unwrap()
    }

    // increment scope level.
    // scopes levels are relative to the current module.
    fn push_level_incr(&mut self) {
        let last = *self.level.last().unwrap();
        self.level.push(last + 1);
    }

    // push level 0 to scope stack.
    fn push_level(&mut self, level: usize) {
        self.level.push(level);
    }

    // pop scope level.
    // scopes levels are relative to the current module.
    fn pop_level(&mut self) {
        self.level.pop().unwrap();
    }

    // when visiting a module.
    // begin visiting module.
    pub(crate) fn mod_begin(&mut self, ident: lex::Ident<'a>) -> Result<(), Error<'a>> {
        self.scope_local.push(Vec::new());
        self.scope_global.push(Vec::new());
        self.mod_path.push(Vec::new());
        self.mod_.push(Some(ident));
        self.push_level(0);
        Ok(())
    }

    // when visiting a module.
    // nd visiting module.
    pub(crate) fn mod_end(&mut self) -> Result<(), Error<'a>> {
        self.pop_level();
        let _ = self.scope_local.pop().expect("scope_local stack is empty");
        let mod_global_path = self.scope_global.pop().expect("scope_local stack is empty");

        // add the globals of the mod and the mod name itself to the list of global
        // symbols (or local, if the module was declared in a scope other than the root)
        // to the scope of the parent module.
        let mod_name = self
            .mod_
            .pop()
            .expect("mod_ stack is empty")
            .expect("Called 'mod_end' on root mod");

        if self.peek_level() == 0 {
            let path = vec![mod_name];
            self.scope_global
                .last_mut()
                .expect("scope_global stack empty")
                .push(path.clone());
            self.scope_global.last_mut().unwrap().extend({
                // prepend scope
                // create an iterator of Vec<Vec<lex::Ident<'a>>>
                mod_global_path
                    .iter()
                    .map(|child_global| {
                        let mut path = path.clone();
                        path.extend_from_slice(&child_global[..]);
                        path
                    })
                    .into_iter()
            })
        } else {
            self.scope_local
                .last_mut()
                .expect("scope_local stack empty")
                .push(vec![mod_name]);
        }

        self.mod_path.pop().expect("mod_path stack is empty");
        Ok(())
    }

    // when parsing a function, you enter a new scope with no visible symbols other
    // than the static ones.
    pub(crate) fn function_begin(&mut self) -> Result<(), Error<'a>> {
        self.push_level_incr();
        self.scope_local.push(Vec::new());
        Ok(())
    }

    // when visiting a function
    // end visiting the current function
    pub(crate) fn function_end(&mut self) -> Result<(), Error<'a>> {
        self.pop_level();
        self.scope_local.pop().expect("scope_local stack empty");
        Ok(())
    }

    // when visiting a scope.
    // begin visiting a new scope.
    pub(crate) fn scope_begin(&mut self) -> Result<(), Error<'a>> {
        let parent_scope_paths = self
            .scope_local
            .last()
            .expect("scope_local stack empty")
            .clone();
        self.scope_local.push(parent_scope_paths);
        Ok(())
    }

    // when visiting a scope.
    // end visiting the current scope.
    pub(crate) fn scope_end(&mut self) -> Result<(), Error<'a>> {
        self.scope_local.pop().expect("scope_local stack empty");
        Ok(())
    }

    fn type_begin(&mut self, ident: lex::Ident<'a>, global: bool) -> Result<(), Error<'a>> {
        if global {
            self.global = true;
            self.mod_path
                .last_mut()
                .expect("mod_path stack empty")
                .push(ident);
        } else {
            self.mod_path
                .last_mut()
                .expect("mod_path stack empty")
                .push(ident);
        }
        Ok(())
    }

    // while visiting a type and its types.
    // begin visiting the current type.
    pub(crate) fn type_begin_local(&mut self, ident: lex::Ident<'a>) -> Result<(), Error<'a>> {
        self.type_begin(ident, false)
    }

    // same but for global types.
    pub(crate) fn type_begin_global(&mut self, ident: lex::Ident<'a>) -> Result<(), Error<'a>> {
        self.type_begin(ident, true)
    }

    pub(crate) fn type_end(&mut self, global: bool) -> Result<(), Error<'a>> {
        let last_path = self.mod_path.last().expect("mod_path stack empty").clone();
        if self.global && self.peek_level() == 0 {
            self.scope_global
                .last_mut()
                .expect("scope_global stack empty")
                .push(last_path);
        } else {
            self.scope_local
                .last_mut()
                .expect("scope_local stack empty")
                .push(last_path);
        }
        if global {
            self.global = false;
        }
        self.mod_path
            .last_mut()
            .expect("mod_path empty")
            .pop()
            .expect("mod_path stack empty");
        Ok(())
    }

    // while visiting a type and its types.
    // end visiting the current type.
    pub(crate) fn type_end_local(&mut self) -> Result<(), Error<'a>> {
        self.type_end(false)
    }

    // same but for global types.
    pub(crate) fn type_end_global(&mut self) -> Result<(), Error<'a>> {
        self.type_end(true)
    }

    pub(crate) fn paths_in_scope(&self) -> impl Iterator<Item = &Vec<lex::Ident<'a>>> {
        self.scope_local
            .last()
            .expect("scope_local stack empty")
            .iter()
            .chain(
                self.scope_global
                    .last()
                    .expect("scope_global stack empty")
                    .iter(),
            )
    }

    pub(crate) fn is_defined(&self, path: &Path) -> bool {
        for scoped in self.paths_in_scope().filter(|p| p.len() == path.len()) {
            if path
                .iter()
                .zip(scoped)
                .all(|(l, r)| format!("{}", l) == format!("{}", r))
            {
                return true;
            }
        }
        false
    }
}
