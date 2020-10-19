use crate::parser::{
    ast::{Expression, Field, Fn, Type},
    lex::Ident,
};
use std::{
    collections::{HashMap, HashSet},
    io::Cursor,
};

/// Virtual memory space of a symbol.
#[derive(Debug, Clone, Copy)]
pub enum Space {
    Static,
    Const,
    Stack,
    Absolute,
}

#[derive(Debug, Clone)]
pub struct Symbol<'a> {
    /// Symbolic name.
    pub name: String,
    /// Offset in virtual memory.
    pub offset: u16,
    /// Size of the symbol itself.
    pub size: u16,
    /// The type of the symbol.
    pub type_: &'a Type<'a>,
    /// Virtual memory space.
    pub space: Space,
}

#[derive(Debug, Default, Clone)]
struct Allocated {
    const_: u16,
    static_: u16,
    stack_: u16,
}

/// Static, Const, and Stack allocation.
#[derive(Debug, Default, Clone)]
pub struct Alloc<'a> {
    absolute_symbols: Vec<Symbol<'a>>,
    const_symbols: Vec<Symbol<'a>>,
    static_symbols: Vec<Symbol<'a>>,
    stack_symbols: Vec<Symbol<'a>>,
    allocated: Allocated,
}

impl<'a> Alloc<'a> {
    /// Allocate const address.
    pub fn alloc_const(&mut self, field: &'a Field<'a>, expr: &Expression) {
        assert!(self.is_undefined(&field.ident));
        let size = Self::compute_all_symbols(
            &String::new(),
            self.allocated.const_,
            field,
            Space::Const,
            &mut self.const_symbols,
        );
        self.allocated.const_ += size;
    }

    /// Allocate static address.
    pub fn alloc_static(&mut self, field: &'a Field<'a>) {
        assert!(self.is_undefined(&field.ident));
        let size = Self::compute_all_symbols(
            &String::new(),
            self.allocated.static_,
            field,
            Space::Static,
            &mut self.static_symbols,
        );
        self.allocated.static_ += size;
    }

    /// Declares a symbol located at the given offset.
    /// Note that it is possible to overlap two symbols, as long as the language
    /// frontend allows it... (the IR doesn't really care about memory aliasing)
    pub fn alloc_absolute(&mut self, field: &'a Field<'a>, offset: u16) {
        assert!(self.is_undefined(&field.ident));
        Self::compute_all_symbols(
            &String::new(),
            offset,
            field,
            Space::Absolute,
            &mut self.absolute_symbols,
        );
    }

    /// Allocate block with the given size in the stack, meant for storing
    /// intermediate results on the stack.
    /// Returns the address of the allocated block.
    pub fn alloc_stack(&mut self, size: u16) -> u16 {
        let address = self.allocated.stack_;
        self.allocated.stack_ += size;
        address
    }

    /// Allocate stack address, associated to the given field.
    /// Returns the first allocated address.
    pub fn alloc_stack_field(&mut self, field: &'a Field<'a>) -> u16 {
        assert!(self.is_undefined(&field.ident));
        let size = Self::compute_all_symbols(
            &String::new(),
            self.allocated.stack_,
            field,
            Space::Stack,
            &mut self.stack_symbols,
        );
        let alloc = self.allocated.stack_;
        self.allocated.stack_ += size;
        alloc
    }

    pub fn symbol(&self, name: &str) -> &Symbol {
        self.stack_symbols
            .iter()
            .chain(self.static_symbols.iter())
            .chain(self.const_symbols.iter())
            .chain(self.absolute_symbols.iter())
            .find(|s| &s.name == name)
            .expect("Undefined symbol")
    }

    fn is_undefined(&self, ident: &Ident) -> bool {
        !(Self::_is_undefined(ident, &self.absolute_symbols)
            || Self::_is_undefined(ident, &self.static_symbols)
            || Self::_is_undefined(ident, &self.const_symbols)
            || Self::_is_undefined(ident, &self.stack_symbols))
    }

    fn _is_undefined(ident: &Ident, symbols: &Vec<Symbol<'a>>) -> bool {
        symbols.iter().find(|s| &s.name == ident.as_str()).is_some()
    }

    // TODO optimize because I'm far too sleepy to do this now.
    //  No need to be calling size_of all over the place here.
    fn compute_all_symbols(
        prefix: &String,
        offset: u16,
        field: &'a Field<'a>,
        space: Space,
        symbols: &mut Vec<Symbol<'a>>,
    ) -> u16 {
        use Type::*;

        // append field identifier to the queried field.
        let name = if prefix.is_empty() {
            field.ident.to_string()
        } else {
            let mut prefix = prefix.clone();
            prefix.push_str(&format!("::{}", field.ident));
            prefix
        };

        // size of the entire field
        let size = super::utils::size_of(&field.type_);

        match &field.type_ {
            U8(_) | I8(_) | Array(_) | Pointer(_) | Fn(_) => {
                symbols.push(Symbol {
                    name,
                    offset,
                    size,
                    type_: &field.type_,
                    space,
                });
            }
            Struct(struct_) => {
                let mut offset = offset;
                for field in struct_.fields.iter() {
                    offset += Self::compute_all_symbols(&name, offset, field, space, symbols);
                }
            }
            Union(union) => {
                for field in union.fields.iter() {
                    Self::compute_all_symbols(&name, offset, field, space, symbols);
                }
            }
            _ => unreachable!(),
        }

        size
    }
}

/// Infallible function allocator.
///
/// Panics instead of returning Optionals or Results, therefore a panic means a
/// bug somewhere in the compiler (likely in the frontend).
#[derive(Default)]
pub struct FnAlloc<'a> {
    fns: HashMap<String, &'a Fn<'a>>,
}

impl<'a> FnAlloc<'a> {
    /// Allocated a function from it's statement.
    /// Panics if a function of the same name is already allocated.
    pub fn alloc(&mut self, fn_: &'a Fn<'a>) {
        assert!(self.fns.insert(fn_.ident.to_string(), fn_).is_none())
    }

    /// Returns the function with the given name.
    /// Panics if it's not defined.
    pub fn get(&self, name: &str) -> &'a Fn<'a> {
        self.fns[name]
    }
}
