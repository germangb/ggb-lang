use crate::parser::ast;
use byteorder::NativeEndian;

const BYTE_SIZE: u16 = 1;
const WORD_SIZE: u16 = 2;

/// Memory layout.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Layout {
    /// Unsigned 8bit byte layout.
    U8,
    /// Signed 8bit byte layout.
    I8,
    /// Array layout.
    Array {
        /// Array inner type layout.
        inner: Box<Layout>,
        /// Array length.
        len: u16,
    },
    /// Pointer layout (16bits).
    Pointer(Box<Layout>),
    /// Struct memory layout.
    Struct(Vec<Layout>),
    /// Enum memory layout.
    Union(Vec<Layout>),
}

impl Layout {
    /// Create type layout from a type from the AST.
    pub fn new(ty: &ast::Type<'_>) -> Self {
        use ast::Type::{Array, Pointer, Struct, Union, I8, U8};
        match ty {
            U8(_) => Self::U8,
            I8(_) => Self::I8,
            Array(array) => {
                let inner = Box::new(Self::new(&array.type_));
                let len = super::expression::const_expr::<NativeEndian>(&array.len, None).unwrap();
                Self::Array { inner, len }
            }
            Pointer(ptr) => {
                let ptr = Box::new(Self::new(&ptr.type_));
                Self::Pointer(ptr)
            }
            Struct(struct_) => {
                let struct_ = struct_.fields.iter().map(|f| Self::new(&f.type_)).collect();
                Self::Struct(struct_)
            }
            Union(union) => {
                let union = union.fields.iter().map(|f| Self::new(&f.type_)).collect();
                Self::Union(union)
            }
            _ => panic!("Type noy yet supported!"),
        }
    }

    /// Compute size of the type layout.
    pub fn size(&self) -> u16 {
        use Layout::{Array, Pointer, Struct, Union, I8, U8};
        match self {
            U8 | I8 => BYTE_SIZE,
            Pointer(_) => WORD_SIZE,
            Array { inner, len } => len * inner.size(),
            Struct(inner) => {
                let fold = |o: u16, l: &Self| o + l.size();
                inner.iter().fold(0, fold)
            }
            Union(inner) => {
                let fold = |o: u16, l: &Self| o.max(l.size());
                inner.iter().fold(0, fold)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Layout::{Array, Pointer, U8};
    use crate::{
        ir::layout::Layout,
        parser::{ast::Grammar, lex::Tokens, ContextBuilder},
    };

    #[test]
    fn test() {
        let mut ctx = ContextBuilder::default().build();
        let mut tokens = Tokens::new("[&u8 (+ 2 2)]").peekable();
        let type_ = Grammar::parse(&mut ctx, &mut tokens).unwrap();

        assert_eq!(Array { inner: Box::new(Pointer(Box::new(U8))),
                           len: 4 },
                   Layout::new(&type_));
    }
}
