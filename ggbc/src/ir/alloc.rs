use crate::{
    byteorder::ByteOrder,
    ir::{expression::compute_const_expr_into_vec, layout::Layout, Pointer},
    parser::{
        ast,
        ast::{Expression, Field, Type},
        lex::Ident,
    },
};
use std::{collections::HashMap, marker::PhantomData};

pub struct Fn {
    pub arg_layout: Vec<Layout>,
    pub ret_layout: Option<Layout>,
}

/// Infallible function allocator.
///
/// Panics instead of returning Optionals or Results, therefore a panic means a
/// bug somewhere in the compiler (likely in the frontend).
#[derive(Default)]
pub struct FnAlloc {
    fns: HashMap<String, (Fn, usize)>,
}

impl FnAlloc {
    /// Allocated a function from it's statement.
    /// Panics if a function of the same name is already allocated.
    pub fn alloc(&mut self, fn_: &ast::Fn<'_>) -> usize {
        let id = self.fns.len();
        let name = fn_.ident.to_string();
        let fn_ = Fn { arg_layout: fn_.fn_arg
                                      .iter()
                                      .flat_map(|a| &a.inner)
                                      .map(|field| Layout::new(&field.type_))
                                      .collect(),
                       ret_layout: fn_.fn_return.as_ref().map(|r| Layout::new(&r.type_)) };
        assert!(self.fns.insert(name, (fn_, id)).is_none());
        id
    }

    /// Returns the function with the given name.
    /// Panics if it's not defined.
    pub fn get(&self, name: &str) -> (&Fn, usize) {
        self.fns.get(name).map(|(fn_, id)| (fn_, *id)).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Space {
    /// Static memory.
    Static,

    /// Const memory (ROM).
    Const,

    /// Stack memory.
    Stack,

    /// Absolute memory space.
    Absolute,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbolic name.
    pub name: String,

    /// Offset in virtual memory.
    pub offset: u16,

    /// Size of the symbol itself.
    pub size: u16,

    /// The type of the symbol.
    pub layout: Layout,

    /// Virtual memory space.
    pub space: Space,
}

impl Symbol {
    pub(crate) fn pointer(&self) -> Pointer {
        match self.space {
            Space::Static => Pointer::Static(self.offset),
            Space::Const => Pointer::Const(self.offset),
            Space::Stack => Pointer::Stack(self.offset),
            Space::Absolute => Pointer::Absolute(self.offset),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SymbolAlloc<B: ByteOrder> {
    const_: Vec<u8>,
    absolute_symbols: Vec<Symbol>,
    const_symbols: Vec<Symbol>,
    static_symbols: Vec<Symbol>,
    stack_symbols: Vec<Symbol>,
    absolute_symbols_alloc: u16,
    const_symbols_alloc: u16,
    static_symbols_alloc: u16,
    stack_symbols_alloc: u16,
    _phantom: PhantomData<B>,
}

impl<B: ByteOrder> SymbolAlloc<B> {
    pub fn into_const_data(self) -> Vec<u8> {
        self.const_
    }

    /// const data so far.
    pub fn const_data(&self) -> &[u8] {
        &self.const_
    }

    /// Clear stack symbols
    pub fn free_stack(&mut self) {
        self.stack_symbols.clear();
        self.stack_symbols_alloc = 0;
    }

    /// Allocate const address.
    pub fn alloc_const(&mut self, field: &Field<'_>, expression: &Expression<'_>) {
        assert!(self.is_undefined(&field.ident));

        let size = Self::compute_all_symbols(&String::new(),
                                             self.const_symbols_alloc,
                                             field,
                                             Space::Const,
                                             &mut self.const_symbols);

        // compute constant expression value
        let symbol_alloc = self.clone();
        compute_const_expr_into_vec::<B>(&Layout::new(&field.type_),
                                         expression,
                                         &symbol_alloc,
                                         &mut self.const_);
        self.const_symbols_alloc += size;
    }

    /// Allocate static address.
    pub fn alloc_static(&mut self, field: &Field<'_>) {
        assert!(self.is_undefined(&field.ident));

        let size = Self::compute_all_symbols(&String::new(),
                                             self.static_symbols_alloc,
                                             field,
                                             Space::Static,
                                             &mut self.static_symbols);

        self.static_symbols_alloc += size;
    }

    /// Declares a symbol located at the given offset.
    /// Note that it is possible to overlap two symbols, as long as the language
    /// frontend allows it... (the IR doesn't really care about memory aliasing)
    pub fn alloc_absolute(&mut self, field: &Field<'_>, offset: u16) {
        assert!(self.is_undefined(&field.ident));

        Self::compute_all_symbols(&String::new(),
                                  offset,
                                  field,
                                  Space::Absolute,
                                  &mut self.absolute_symbols);
    }

    #[warn(unused)]
    pub fn stack_address(&self) -> u16 {
        self.stack_symbols_alloc
    }

    /// Allocate stack address, associated to the given field.
    /// Returns the first allocated address.
    pub fn alloc_stack_field(&mut self, field: &Field<'_>) -> u16 {
        assert!(self.is_undefined(&field.ident));

        let size = Self::compute_all_symbols(&String::new(),
                                             self.stack_symbols_alloc,
                                             field,
                                             Space::Stack,
                                             &mut self.stack_symbols);

        let alloc = self.stack_symbols_alloc;
        self.stack_symbols_alloc += size;
        alloc
    }

    /// Locates a symbol by name.
    /// Panics if the symbol is not defined.
    pub fn get(&self, name: &str) -> &Symbol {
        self.stack_symbols
            .iter()
            .chain(self.static_symbols.iter())
            .chain(self.const_symbols.iter())
            .chain(self.absolute_symbols.iter())
            .find(|s| s.name == name)
            .expect(&format!("Undefined symbol: {}", name))
    }

    fn is_undefined(&self, ident: &Ident<'_>) -> bool {
        !(Self::_is_undefined(ident, &self.absolute_symbols)
          || Self::_is_undefined(ident, &self.static_symbols)
          || Self::_is_undefined(ident, &self.const_symbols)
          || Self::_is_undefined(ident, &self.stack_symbols))
    }

    fn _is_undefined(ident: &Ident<'_>, symbols: &[Symbol]) -> bool {
        symbols.iter().any(|s| s.name == ident.to_string())
    }

    // TODO optimize because I'm far too sleepy to do this now.
    //  No need to be calling size_of all over the place here.
    fn compute_all_symbols(prefix: &str,
                           offset: u16,
                           field: &Field<'_>,
                           space: Space,
                           symbols: &mut Vec<Symbol>)
                           -> u16 {
        use Type::{Array, Pointer, Struct, Union, I8, U8};

        // append field identifier to the queried field.
        let name = if prefix.is_empty() {
            field.ident.to_string()
        } else {
            let mut prefix = prefix.to_string();
            prefix.push_str(&format!("::{}", field.ident));
            prefix
        };

        // size of the entire field
        let field_type = Layout::new(&field.type_);
        let size = field_type.size();

        match &field.type_ {
            U8(_) | I8(_) | Array(_) | Pointer(_) => {
                symbols.push(Symbol { name,
                                      offset,
                                      size,
                                      layout: Layout::new(&field.type_),
                                      space });
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

/// Virtual register allocator.
#[derive(Default)]
pub struct RegisterAlloc {
    bitset: u64,
}

impl Drop for RegisterAlloc {
    fn drop(&mut self) {
        assert_eq!(0, self.bitset, "register leak");
    }
}

impl RegisterAlloc {
    #[warn(unused)]
    /// Returns number of allocated registers.
    pub fn len(&self) -> u32 {
        self.bitset.count_ones()
    }

    /// Allocate register.
    pub fn alloc(&mut self) -> usize {
        let min = self.min();
        self.set(min, true);
        min
    }

    /// Free register being used.
    pub fn free(&mut self, index: usize) {
        assert!(self.get(index));
        self.set(index, false);
    }

    fn min(&self) -> usize {
        (!self.bitset).trailing_zeros() as _
    }

    fn get(&self, index: usize) -> bool {
        let bit = 1 << (index as u64);
        (self.bitset & bit) != 0
    }

    fn set(&mut self, index: usize, value: bool) -> bool {
        let bit = 1 << (index as u64);
        let old = (self.bitset | bit) != 0;
        if value {
            self.bitset |= bit;
        } else {
            self.bitset &= !bit;
        }
        old
    }
}

#[cfg(test)]
mod test {
    use super::RegisterAlloc;

    #[test]
    fn alloc() {
        let mut alloc = RegisterAlloc::default();

        alloc.alloc();
        alloc.alloc();
        alloc.alloc();
        alloc.alloc();
        assert_eq!(0b1111, alloc.bitset);
        alloc.set(1, false);
        assert_eq!(0b1101, alloc.bitset);
        alloc.alloc();
        assert_eq!(0b1111, alloc.bitset);
        alloc.bitset = 0;
    }

    #[test]
    fn set() {
        let mut alloc = RegisterAlloc::default();
        alloc.set(0, true);
        alloc.set(2, true);
        alloc.set(4, true);
        alloc.set(6, true);
        assert!(alloc.get(0));
        assert!(!alloc.get(1));
        assert!(alloc.get(2));
        assert!(!alloc.get(3));
        assert!(alloc.get(4));
        assert!(!alloc.get(5));
        assert!(alloc.get(6));
        assert_eq!(0b1010101, alloc.bitset);
        alloc.set(2, false);
        alloc.set(4, false);
        alloc.set(6, false);
        assert_eq!(0b1, alloc.bitset);
        assert_eq!(1, alloc.min());
        alloc.bitset = 0;
    }
}
