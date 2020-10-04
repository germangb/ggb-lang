use crate::{
    ast::{expressions::Path, Statement, Type},
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
            path: Vec::new(),
            paths_in_scope: vec![vec![]],
            parent: None,
        }
    }
}

pub struct Context<'a, 'b> {
    level: usize,
    // FIXME
    //  (stack) Paths in scope from the current scope level.
    //  This should be replaced by a tree of identifiers.
    paths_in_scope: Vec<Vec<Vec<lex::Ident<'a>>>>,
    // Current symbol path.
    // Used when parser is visiting symbols (nested in structs and unions).
    path: Vec<lex::Ident<'a>>,
    parent: Option<&'b mut Self>,
}

impl<'a, 'b> Context<'a, 'b> {
    // when parsing a function, you enter a new scope with no visible symbols other
    // than the static ones.
    pub(crate) fn push_scope_empty(&mut self) {
        self.paths_in_scope.push(Vec::new());
        self.level += 1;
    }

    pub(crate) fn push_scope(&mut self) {
        let paths = self.paths_in_scope[self.level].clone();
        self.paths_in_scope.push(paths);
        self.level += 1;
    }

    pub(crate) fn pop_scope(&mut self) {
        self.paths_in_scope.pop();
        self.level -= 1;
    }

    pub(crate) fn push_path(&mut self, ident: lex::Ident<'a>) {
        self.path.push(ident);
    }

    pub(crate) fn pop_path(&mut self) {
        print!("in level = {} def ", self.level);
        for (i, ident) in self.path.iter().enumerate() {
            if i > 0 {
                print!("::{}", ident);
            } else {
                print!("{}", ident);
            }
        }
        println!();

        // add current path to the list of paths in scope.
        let new_path = self.path.clone();
        self.paths_in_scope[self.level].push(new_path);
        self.path.pop().unwrap();
    }

    pub(crate) fn is_defined(&self, path: &Path) -> bool {
        println!(
            "in level = {} is_def {}",
            self.level,
            path.iter()
                .fold(String::new(), |s, i| format!("{}::{}", s, i))
        );
        for scoped in self.paths_in_scope[self.level]
            .iter()
            .filter(|p| p.len() == path.len())
        {
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
