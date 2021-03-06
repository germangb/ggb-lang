use crate::{
    byteorder::ByteOrder,
    ir::{
        compile::{expression::const_expr, layout::Layout},
        opcodes::Pointer,
    },
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
        let fn_ = Fn {
            arg_layout: fn_
                .fn_arg
                .iter()
                .flat_map(|a| &a.inner)
                .map(|field| Layout::new(&field.type_))
                .collect(),
            ret_layout: fn_.fn_return.as_ref().map(|r| Layout::new(&r.type_)),
        };
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
pub enum SymbolMemorySpace {
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
    pub memory_space: SymbolMemorySpace,
}

impl Symbol {
    pub(crate) fn pointer(&self) -> Pointer {
        match self.memory_space {
            SymbolMemorySpace::Static => Pointer::Static(self.offset),
            SymbolMemorySpace::Const => Pointer::Const(self.offset),
            SymbolMemorySpace::Stack => Pointer::Stack(self.offset),
            SymbolMemorySpace::Absolute => Pointer::Absolute(self.offset),
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

    pub fn static_usage(&self) -> u16 {
        self.static_symbols_alloc
    }

    pub fn stack_usage(&self) -> u16 {
        self.stack_symbols_alloc
    }

    /// override static memory usage.
    ///
    /// Used for then the symbol allocator is cloned when compiling a child
    /// scope, but you still want to keep the static memory allocated within it.
    pub fn set_static_usage(&mut self, usage: u16) {
        assert!(usage >= self.static_symbols_alloc);
        self.static_symbols_alloc = usage;
    }

    pub fn set_const(&mut self, const_: Vec<u8>) -> Vec<u8> {
        std::mem::replace(&mut self.const_, const_)
    }

    /// Clear stack symbols
    pub fn clear_stack(&mut self) {
        self.stack_symbols.clear();
        self.stack_symbols_alloc = 0;
    }

    /// Allocate const address.
    pub fn alloc_const(&mut self, field: &Field<'_>, expression: &Expression<'_>) {
        assert!(self.is_undefined(&field.ident));

        Self::compute_all_symbols(
            &String::new(),
            self.const_.len() as _,
            field,
            SymbolMemorySpace::Const,
            &mut self.const_symbols,
        );

        // compute constant expression value
        let symbol_alloc = self.clone();
        compute_const_expr_into_vec::<B>(
            &Layout::new(&field.type_),
            expression,
            &symbol_alloc,
            &mut self.const_,
        );
    }

    /// Allocate static address.
    pub fn alloc_static(&mut self, field: &Field<'_>) {
        assert!(self.is_undefined(&field.ident));

        let size = Self::compute_all_symbols(
            &String::new(),
            self.static_symbols_alloc,
            field,
            SymbolMemorySpace::Static,
            &mut self.static_symbols,
        );
        self.static_symbols_alloc += size;
    }

    /// Declares a symbol located at the given offset.
    /// Note that it is possible to overlap two symbols, as long as the language
    /// frontend allows it... (the IR doesn't really care about memory aliasing)
    pub fn alloc_absolute(&mut self, field: &Field<'_>, offset: u16) {
        assert!(self.is_undefined(&field.ident));

        Self::compute_all_symbols(
            &String::new(),
            offset,
            field,
            SymbolMemorySpace::Absolute,
            &mut self.absolute_symbols,
        );
    }

    pub fn stack_address(&self) -> u16 {
        self.stack_symbols_alloc
    }

    /// Allocate stack address, associated to the given field.
    /// Returns the first allocated address.
    pub fn alloc_stack_field(&mut self, field: &Field<'_>) -> u16 {
        assert!(self.is_undefined(&field.ident));

        let size = Self::compute_all_symbols(
            &String::new(),
            self.stack_symbols_alloc,
            field,
            SymbolMemorySpace::Stack,
            &mut self.stack_symbols,
        );

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
    fn compute_all_symbols(
        prefix: &str,
        offset: u16,
        field: &Field<'_>,
        memory_space: SymbolMemorySpace,
        symbols: &mut Vec<Symbol>,
    ) -> u16 {
        // append field identifier to the queried field.
        let name = if prefix.is_empty() {
            field.ident.to_string()
        } else {
            let mut prefix = prefix.to_string();
            prefix.push_str(&format!("::{}", field.ident));
            prefix
        };

        let size = Layout::new(&field.type_).size();
        match &field.type_ {
            Type::U8(_) | Type::I8(_) | Type::Array(_) | Type::Pointer(_) => {
                let layout = Layout::new(&field.type_);
                symbols.push(Symbol {
                    name,
                    offset,
                    size,
                    layout,
                    memory_space,
                });
            }
            Type::Struct(struct_) => {
                let mut offset = offset;
                for field in struct_.fields.iter() {
                    offset +=
                        Self::compute_all_symbols(&name, offset, field, memory_space, symbols);
                }
            }
            Type::Union(union) => {
                for field in union.fields.iter() {
                    Self::compute_all_symbols(&name, offset, field, memory_space, symbols);
                }
            }
            _ => unreachable!(),
        }
        size
    }
}

#[deprecated]
pub fn compute_const_expr_into_vec<B: ByteOrder>(
    layout: &Layout,
    expression: &Expression<'_>,
    symbol_alloc: &SymbolAlloc<B>,
    out: &mut Vec<u8>,
) {
    match (layout, expression) {
        (Layout::U8, expression) => {
            let lit = const_expr(expression, Some(symbol_alloc)).unwrap();
            assert!(lit <= 0xff);
            out.push(lit as u8);
        }
        (Layout::I8, expression) => {
            let lit = const_expr(expression, Some(symbol_alloc)).unwrap();
            assert!(lit <= i8::max_value() as u16 && lit >= i8::min_value() as u16);
            out.push(unsafe { std::mem::transmute(lit as i8) });
        }
        (Layout::Pointer(_), expr @ Expression::Lit(_)) => {
            let lit = const_expr(expr, Some(symbol_alloc)).unwrap();
            let offset = out.len();
            // append value with the correct endianness
            out.push(0);
            out.push(0);
            B::write_u16(&mut out[offset..], lit);
        }
        (Layout::Array { inner, len }, Expression::Array(array)) => {
            assert_eq!(*len as usize, array.inner.len());
            for item in &array.inner {
                compute_const_expr_into_vec::<B>(inner, item, symbol_alloc, out);
            }
        }
        _ => panic!(),
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
