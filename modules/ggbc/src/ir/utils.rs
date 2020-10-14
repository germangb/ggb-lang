use crate::parser::{
    ast::{Expression, Field, Type},
    lex::Ident,
};
use byteorder::WriteBytesExt;
use std::{
    collections::{HashMap, HashSet},
    io::Cursor,
};

fn offsets_private(
    prefix: &String,
    mut offset: u16,
    field: &Field,
    offsets: &mut HashSet<SymbolData>,
) -> u16 {
    // append field identifier to the queried field.
    let prefix = if prefix.is_empty() {
        field.ident.to_string()
    } else {
        let mut prefix = prefix.clone();
        prefix.push_str(&format!("::{}", field.ident));
        prefix
    };

    let type_size = size_of(&field.type_);

    // TODO optimize because I'm far too sleepy to do this now.
    //  No need to be calling size_of all over the place here.
    match &field.type_ {
        Type::U8(_) | Type::I8(_) | Type::Array(_) | Type::Pointer(_) | Type::Fn(_) => {
            offsets.insert(SymbolData {
                name: prefix,
                offset,
                size: type_size,
            });
        }
        Type::Struct(struct_) => {
            for field in struct_.fields.iter() {
                offset += offsets_private(&prefix, offset, field, offsets);
            }
        }
        Type::Union(union) => {
            for field in union.fields.iter() {
                offsets_private(&prefix, offset, field, offsets);
            }
        }
        _ => unreachable!(),
    }

    type_size
}

// Compute size of the given type.
fn size_of(type_: &Type) -> u16 {
    use Type::*;
    match type_ {
        U8(_) | I8(_) => 1,
        Pointer(_) | Fn(_) => 2,
        Array(array) => size_of(&array.type_) * compute_const_expr(&array.len),
        Struct(struct_) => struct_
            .fields
            .iter()
            .fold(0, |size, field| size.max(size_of(&field.type_))),
        Union(union) => union
            .fields
            .iter()
            .fold(0, |size, field| size + size_of(&field.type_)),
        _ => unreachable!(),
    }
}

/// Compute the size of a given constant expression.
/// Panics if the expression is not a constant expression nor numeric.
pub fn compute_const_expr(expr: &Expression) -> u16 {
    use Expression::*;
    match expr {
        Lit(e) => {
            let num = e.to_string();
            if num.starts_with("0x") {
                u16::from_str_radix(&num[2..], 16).expect("Not a hex number")
            } else if num.as_bytes()[0].is_ascii_digit() {
                num.parse().expect("Not a number")
            } else {
                panic!("Not a number literal")
            }
        }
        Minus(e) => unimplemented!("TODO"),
        Not(e) => !compute_const_expr(&e.inner),
        Add(e) => compute_const_expr(&e.inner.left) + compute_const_expr(&e.inner.right),
        Sub(e) => compute_const_expr(&e.inner.left) - compute_const_expr(&e.inner.right),
        #[cfg(feature = "mul")]
        Mul(e) => compute_const_expr(&e.inner.left) * compute_const_expr(&e.inner.right),
        #[cfg(feature = "div")]
        Div(e) => compute_const_expr(&e.inner.left) / compute_const_expr(&e.inner.right),
        And(e) => compute_const_expr(&e.inner.left) & compute_const_expr(&e.inner.right),
        Or(e) => compute_const_expr(&e.inner.left) | compute_const_expr(&e.inner.right),
        Xor(e) => compute_const_expr(&e.inner.left) ^ compute_const_expr(&e.inner.right),
        LeftShift(e) => compute_const_expr(&e.inner.left) << compute_const_expr(&e.inner.right),
        RightShift(e) => compute_const_expr(&e.inner.left) >> compute_const_expr(&e.inner.right),
        _ => panic!("Not a constant expression"),
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SymbolData {
    name: String,
    offset: u16,
    size: u16,
}

pub enum Symbol<'a> {
    /// Symbol in Const memory.
    Const(&'a SymbolData),
    /// Symbol in Static memory.
    Static(&'a SymbolData),
    /// Symbol in Stack memory.
    Stack(&'a SymbolData),
}

impl Symbol<'_> {
    pub fn symbol_data(&self) -> &SymbolData {
        match self {
            Symbol::Const(data) => data,
            Symbol::Static(data) => data,
            Symbol::Stack(data) => data,
        }
    }
}

#[derive(Debug, Default)]
pub struct Allocated {
    /// Total addresses (& data) allocated for const memory (ROM).
    pub const_: u16,
    /// Total addresses allocated for static memory.
    pub static_: u16,
    /// Total addresses allocated for the stack.
    pub stack_: u16,
}

/// Static, Const, and Stack allocator.
#[derive(Debug, Default)]
pub struct Alloc {
    const_symbols: HashSet<SymbolData>,
    const_: Cursor<Vec<u8>>,
    static_symbols: HashSet<SymbolData>,
    stack_symbols: HashSet<SymbolData>,
    allocated: Allocated,
}

impl Alloc {
    /// Return allocation numbers.
    pub fn allocated(&self) -> &Allocated {
        &self.allocated
    }

    pub fn alloc_const(&mut self, field: &Field, expr: &Expression) -> u16 {
        assert!(self.is_undefined(&field.ident));
        let size = offsets_private(
            &String::new(),
            self.allocated.const_,
            field,
            &mut self.const_symbols,
        );
        self.allocated.const_ += size;
        size
    }

    pub fn alloc_static(&mut self, field: &Field) -> u16 {
        assert!(
            self.is_undefined(&field.ident),
            "identifier {} is already defined",
            field.ident
        );
        let size = self.alloc_static_at(field, self.allocated.static_);
        self.allocated.static_ += size;
        size
    }

    pub fn alloc_static_at(&mut self, field: &Field, offset: u16) -> u16 {
        assert!(
            self.is_undefined(&field.ident),
            "identifier {} is already defined",
            field.ident
        );
        offsets_private(&String::new(), offset, field, &mut self.static_symbols)
    }

    pub fn alloc_stack(&mut self, field: &Field) -> u16 {
        assert!(
            self.is_undefined(&field.ident),
            "identifier {} is already defined",
            field.ident
        );
        let size = offsets_private(
            &String::new(),
            self.allocated.stack_,
            field,
            &mut self.stack_symbols,
        );
        self.allocated.stack_ += size;
        size
    }

    /// Query a symbol by name.
    pub fn symbol(&self, name: &str) -> Option<Symbol> {
        self.stack_symbols
            .iter()
            .map(Symbol::Stack)
            .chain(self.static_symbols.iter().map(Symbol::Static))
            .chain(self.const_symbols.iter().map(Symbol::Const))
            .find(|s| &s.symbol_data().name == name)
    }

    fn is_undefined(&self, ident: &Ident) -> bool {
        !(self
            .const_symbols
            .iter()
            .find(|s| &s.name == ident.as_str())
            .is_some()
            || self
                .static_symbols
                .iter()
                .find(|s| &s.name == ident.as_str())
                .is_some()
            || self
                .stack_symbols
                .iter()
                .find(|s| &s.name == ident.as_str())
                .is_some())
    }
}
