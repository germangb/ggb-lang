use crate::{
    ast::{expressions::Path, Statement, Type},
    error::Error,
    lex,
};
use std::{borrow::Cow, collections::HashSet};

#[derive(Default, Debug)]
pub struct ContextBuilder {}

impl ContextBuilder {
    pub fn build<'a>(self) -> Context<'a> {
        Context {
            scope_local: vec![vec![]],
            scope_mod: vec![vec![]],
            level: vec![0],
            path: Vec::new(),
        }
    }
}

type Stack<T> = Vec<T>;

pub struct Context<'a> {
    // FIXME temporary
    //  - Paths in the current scope.
    //  - Paths in the current mod.
    //  - Replace with more sophisticated data structure.
    scope_local: Stack<Vec<Vec<lex::Ident<'a>>>>,
    scope_mod: Stack<Vec<Vec<lex::Ident<'a>>>>,
    // Scope level stack. Scopes are relative to the current module.
    level: Stack<usize>,
    // Current symbol path.
    // Used when parser is visiting symbols (nested in structs and unions).
    path: Stack<lex::Ident<'a>>,
}

impl<'a> Context<'a> {
    // current scope level.
    // value relative to the current module.
    // example: `mod foo { mod bar {}}`.
    // both foo and bar are at scope level = 0.
    fn level(&self) -> usize {
        *self.level.last().unwrap()
    }

    // increment scope level.
    // scopes levels are relative to the current module.
    fn level_push_incr(&mut self) {
        self.level.push(*self.level.last().unwrap() + 1);
    }

    // push level 0 to scope stack.
    fn level_push_reset(&mut self) {
        self.level.push(0);
    }

    // pop scope level.
    // scopes levels are relative to the current module.
    fn level_pop(&mut self) {
        self.level.pop().unwrap();
    }

    // when visiting a module.
    // begin visiting module.
    pub(crate) fn mod_begin(&mut self, ident: lex::Ident<'a>) {
        self.path.push(ident);
        self.scope_mod.push(Vec::new());
        self.level_push_reset();
    }

    // when visiting a module.
    // nd visiting module.
    pub(crate) fn mod_end(&mut self) {
        // all found paths will be visible by the parent mod.
        let new_path = self.path.clone();
        self.scope_local.last_mut().unwrap().push(new_path.clone());
        self.path.pop().unwrap();
        self.level_pop();

        // add types defined in the module root to the parent module scope.
        if self.level() == 0 {
            let paths_in_mod = self.scope_mod.pop().unwrap();
            self.scope_local
                .last_mut()
                .unwrap()
                .extend(paths_in_mod.clone());
            self.scope_mod.last_mut().unwrap().extend(paths_in_mod);
            self.scope_mod.last_mut().unwrap().push(new_path);
        } else {
            unimplemented!("edge case: module defined not in parent module root")
        }
    }

    // when parsing a function, you enter a new scope with no visible symbols other
    // than the static ones.
    pub(crate) fn function_begin(&mut self) {
        self.scope_local.push(Vec::new());
        self.level_push_incr();
    }

    // when visiting a function
    // end visiting the current function
    pub(crate) fn function_end(&mut self) {
        self.scope_local.pop();
        self.level_pop();
    }

    // when visiting a scope.
    // begin visiting a new scope.
    pub(crate) fn scope_begin(&mut self) {
        let paths = self.scope_local.last().unwrap().clone();
        self.scope_local.push(paths);
        self.level_push_incr();
    }

    // when visiting a scope.
    // end visiting the current scope.
    pub(crate) fn scope_end(&mut self) {
        self.scope_local.pop();
        self.level_pop();
    }

    // while visiting a type and its types.
    // begin visiting the current type.
    pub(crate) fn type_begin(&mut self, ident: lex::Ident<'a>) {
        self.path.push(ident);
    }

    // while visiting a type and its types.
    // end visiting the current type.
    pub(crate) fn type_end(&mut self) {
        // add current path to the list of paths in scope.
        let new_path = self.path.clone();

        // if type was defined in the root scope, add it to the module visibility.
        if self.level() == 0 {
            self.scope_mod.last_mut().unwrap().push(new_path.clone());
        }

        self.scope_local.last_mut().unwrap().push(new_path);
        self.path.pop().unwrap();
    }

    pub(crate) fn paths_in_scope(&self) -> impl Iterator<Item = &Vec<lex::Ident<'a>>> {
        self.scope_local
            .last()
            .unwrap()
            .iter()
            .chain(self.scope_mod.last().unwrap())
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
