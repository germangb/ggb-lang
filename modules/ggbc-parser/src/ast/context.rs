use crate::{error::Error, lex};

type Stack<T> = Vec<T>;

#[derive(Default, Debug)]
pub struct ContextBuilder {}

impl ContextBuilder {
    pub fn build<'a>(self) -> Context<'a> {
        Context {
            scope_level: vec![0],
            mod_: vec![None],
            mod_path: vec![Vec::new()],
            scope_local: vec![Vec::new()],
            scope_global: vec![Vec::new()],
            global: false,
            let_: false,
            loop_level: vec![0],
            fn_level: vec![0],
        }
    }
}

// FIXME temporary
//  - Replace path path resolution with a tree
pub struct Context<'a> {
    // Scope level stack. Scopes are relative to the current module.
    scope_level: Stack<usize>,
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
    // a static or const symbol
    global: bool,
    // let statement being parsed, if any.
    let_: bool,
    // current loop, relative to the current module.
    loop_level: Stack<usize>,
    // Nested function level, relative to the module root.
    fn_level: Stack<usize>,
}

impl<'a> Context<'a> {
    fn scope_level(&self) -> usize {
        *self.scope_level.last().unwrap()
    }

    // when visiting a module.
    // begin visiting module.
    pub(crate) fn begin_mod(&mut self, ident: lex::Ident<'a>) -> Result<(), Error<'a>> {
        self.scope_local.push(Vec::new());
        self.scope_global.push(Vec::new());
        self.mod_path.push(Vec::new());
        self.mod_.push(Some(ident));

        self.scope_level.push(0);
        self.loop_level.push(0);
        self.fn_level.push(0);
        Ok(())
    }

    // when visiting a module.
    // nd visiting module.
    pub(crate) fn end_mod(&mut self) -> Result<(), Error<'a>> {
        self.scope_level.pop().unwrap();
        self.loop_level.pop().unwrap();
        self.fn_level.pop().unwrap();

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

        if self.scope_level() == 0 {
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
    pub(crate) fn begin_fn(&mut self) -> Result<(), Error<'a>> {
        self.loop_level.push(0);
        self.scope_level.push(*self.scope_level.last().unwrap() + 1);
        self.fn_level.push(*self.fn_level.last().unwrap() + 1);
        self.scope_local.push(Vec::new());
        Ok(())
    }

    // when visiting a function
    // end visiting the current function
    pub(crate) fn end_fn(&mut self) -> Result<(), Error<'a>> {
        self.loop_level.pop().unwrap();
        self.scope_level.pop().unwrap();
        self.fn_level.pop().unwrap();
        self.scope_local.pop().unwrap();
        Ok(())
    }

    // when visiting a scope.
    // begin visiting a new scope.
    pub(crate) fn begin_scope(&mut self) -> Result<(), Error<'a>> {
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
    pub(crate) fn end_scope(&mut self) -> Result<(), Error<'a>> {
        self.scope_local.pop().unwrap();
        Ok(())
    }

    fn begin_symbol(&mut self, ident: lex::Ident<'a>) -> Result<(), Error<'a>> {
        if format!("{}", ident) == "fuck" {
            return Err(Error::ForbiddenIdent {
                ident,
                reason: Some("Not the F-word, please..."),
            });
        }

        self.mod_path
            .last_mut()
            .expect("mod_path stack empty")
            .push(ident);

        // check if adding ident causes `mod_path` to shadow a scoped symbol.
        let path = self.mod_path.last().unwrap();
        let path_refs: Vec<_> = path.iter().collect();
        if self.is_defined(&path_refs[..]) {
            Err(Error::ShadowIdent {
                // TODO
                ident: path.last().cloned().unwrap(),
                shadow: path.last().cloned().unwrap(),
            })
        } else {
            Ok(())
        }
    }

    fn end_symbol(&mut self) -> Result<(), Error<'a>> {
        let last_path = self.mod_path.last().expect("mod_path stack empty").clone();
        if self.global && self.scope_level() == 0 {
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
        self.mod_path
            .last_mut()
            .expect("mod_path empty")
            .pop()
            .expect("mod_path stack empty");
        Ok(())
    }

    // while visiting a type and its types.
    // begin visiting the current type.
    pub(crate) fn begin_local_symbol(&mut self, ident: lex::Ident<'a>) -> Result<(), Error<'a>> {
        self.begin_symbol(ident)
    }

    // same but for global types.
    pub(crate) fn begin_global_symbol(&mut self, ident: lex::Ident<'a>) -> Result<(), Error<'a>> {
        assert!(!self.global);
        self.begin_symbol(ident)?;
        self.global = true;
        Ok(())
    }

    // while visiting a type and its types.
    // end visiting the current type.
    pub(crate) fn end_local_symbol(&mut self) -> Result<(), Error<'a>> {
        self.end_symbol()
    }

    // same but for global types.
    pub(crate) fn end_global_symbol(&mut self) -> Result<(), Error<'a>> {
        assert!(self.global);
        self.end_symbol()?;
        self.global = false;
        Ok(())
    }

    // begin parsing Let statement
    pub(crate) fn begin_let(&mut self) -> Result<(), Error<'a>> {
        assert!(!self.let_);
        self.let_ = true;
        Ok(())
    }

    // end parsing Let statement
    pub(crate) fn end_let(&mut self) -> Result<(), Error<'a>> {
        assert!(self.let_);
        self.let_ = false;
        Ok(())
    }

    pub(crate) fn begin_loop(&mut self) -> Result<(), Error<'a>> {
        let level = *self.loop_level.last().unwrap();
        self.loop_level.push(level + 1);
        Ok(())
    }

    pub(crate) fn end_loop(&mut self) -> Result<(), Error<'a>> {
        self.loop_level.pop().unwrap();
        Ok(())
    }

    fn paths_in_scope(&self) -> impl Iterator<Item = &Vec<lex::Ident<'a>>> {
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

    pub(crate) fn is_defined(&self, path: &[&lex::Ident<'a>]) -> bool {
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

    // Return true if the parser is inside of a loop.
    pub(crate) fn in_loop(&self) -> bool {
        *self.loop_level.last().unwrap() > 0
    }

    // Return true if the parser is inside of a function.
    pub(crate) fn in_fn(&self) -> bool {
        *self.fn_level.last().unwrap() > 0
    }

    // Return true if the parser is parsing the root of a module (i.e. not in a
    // member fn).
    pub(crate) fn in_mod_root(&self) -> bool {
        let current_mod = self.mod_.last().unwrap();
        match (current_mod, self.scope_level()) {
            (Some(_), 0) => true,
            (Some(_), _) => false,
            (None, _) => false,
        }
    }

    // Returns true if the parser is inside the entry point of the program.
    pub(crate) fn in_entry_point(&self) -> bool {
        self.mod_.last().unwrap().is_none()
    }
}
