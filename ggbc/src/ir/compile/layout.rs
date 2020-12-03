use crate::{
    ir::compile::expression::const_expr,
    parser::{ast, ast::Type},
};
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
        match ty {
            Type::U8(_) => Self::U8,
            Type::I8(_) => Self::I8,
            Type::Array(array) => {
                let inner = Box::new(Self::new(&array.type_));
                let len = const_expr::<NativeEndian>(&array.len, None).unwrap();
                Self::Array { inner, len }
            }
            Type::Pointer(ptr) => {
                let ptr = Box::new(Self::new(&ptr.type_));
                Self::Pointer(ptr)
            }
            Type::Struct(struct_) => {
                let struct_ = struct_.fields.iter().map(|f| Self::new(&f.type_)).collect();
                Self::Struct(struct_)
            }
            Type::Union(union) => {
                let union = union.fields.iter().map(|f| Self::new(&f.type_)).collect();
                Self::Union(union)
            }
            _ => panic!("Type noy yet supported!"),
        }
    }

    /// Compute size of the type layout.
    pub fn size(&self) -> u16 {
        match self {
            Layout::U8 | Layout::I8 => BYTE_SIZE,
            Layout::Pointer(_) => WORD_SIZE,
            Layout::Array { inner, len } => len * inner.size(),
            Layout::Struct(inner) => inner.iter().fold(0, |o, l| o + l.size()),
            Layout::Union(inner) => inner.iter().fold(0, |o, l| l.size().max(o)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Layout;
    use crate::parser::{ast::Grammar, lex::Tokens, ContextBuilder};

    #[test]
    fn test_new() {
        let mut ctx = ContextBuilder::default().build();
        let mut tokens = Tokens::new("[&u8 (+ 2 2)]").peekable();
        let type_ = Grammar::parse(&mut ctx, &mut tokens).unwrap();

        assert_eq!(
            Layout::Array {
                inner: Box::new(Layout::Pointer(Box::new(Layout::U8))),
                len: 4
            },
            Layout::new(&type_)
        );
    }

    #[test]
    fn zero_size_types() {
        assert_eq!(
            0,
            Layout::Array {
                inner: Box::new(Layout::U8),
                len: 0
            }
            .size()
        );
        assert_eq!(0, Layout::Struct(vec![]).size());
        assert_eq!(0, Layout::Union(vec![]).size());
    }

    #[test]
    fn size_u8() {
        assert_eq!(1, Layout::U8.size());
    }

    #[test]
    fn size_i8() {
        assert_eq!(1, Layout::I8.size());
    }

    #[test]
    fn test_pointer() {
        assert_eq!(2, Layout::Pointer(Box::new(Layout::U8)).size());
        assert_eq!(
            2,
            Layout::Pointer(Box::new(Layout::Struct(vec![Layout::U8, Layout::I8]))).size()
        );
        assert_eq!(
            2,
            Layout::Pointer(Box::new(Layout::Struct(vec![Layout::Struct(vec![])]))).size()
        );
        assert_eq!(
            2,
            Layout::Pointer(Box::new(Layout::Array {
                inner: Box::new(Layout::U8),
                len: 16
            }))
            .size()
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            0,
            Layout::Array {
                inner: Box::new(Layout::U8),
                len: 0
            }
            .size()
        );
        assert_eq!(
            1,
            Layout::Array {
                inner: Box::new(Layout::U8),
                len: 1
            }
            .size()
        );
        assert_eq!(
            2,
            Layout::Array {
                inner: Box::new(Layout::U8),
                len: 2
            }
            .size()
        );
        assert_eq!(
            3,
            Layout::Array {
                inner: Box::new(Layout::U8),
                len: 3
            }
            .size()
        );
    }

    #[test]
    fn test_array_composite() {
        let inner = Layout::Array {
            inner: Box::new(Layout::U8),
            len: 4,
        };
        assert_eq!(
            0,
            Layout::Array {
                inner: Box::new(inner.clone()),
                len: 0
            }
            .size()
        );
        assert_eq!(
            4,
            Layout::Array {
                inner: Box::new(inner.clone()),
                len: 1
            }
            .size()
        );
        assert_eq!(
            8,
            Layout::Array {
                inner: Box::new(inner.clone()),
                len: 2
            }
            .size()
        );
        assert_eq!(
            12,
            Layout::Array {
                inner: Box::new(inner.clone()),
                len: 3
            }
            .size()
        );
    }

    #[test]
    fn test_struct() {
        assert_eq!(0, Layout::Struct(vec![]).size());
        assert_eq!(1, Layout::Struct(vec![Layout::U8]).size());
        assert_eq!(2, Layout::Struct(vec![Layout::U8, Layout::I8]).size());
        assert_eq!(
            6,
            Layout::Struct(vec![
                Layout::U8,
                Layout::I8,
                Layout::Array {
                    inner: Box::new(Layout::U8),
                    len: 4,
                }
            ])
            .size()
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(0, Layout::Union(vec![]).size());
        assert_eq!(1, Layout::Union(vec![Layout::U8]).size());
        assert_eq!(1, Layout::Union(vec![Layout::U8, Layout::I8]).size());
        assert_eq!(
            4,
            Layout::Union(vec![
                Layout::U8,
                Layout::I8,
                Layout::Array {
                    inner: Box::new(Layout::U8),
                    len: 4,
                }
            ])
            .size()
        );
    }
}
