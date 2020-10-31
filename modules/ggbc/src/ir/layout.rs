use crate::parser::ast;

/// Type layout.
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
    pub fn from_type(ty: &ast::Type) -> Self {
        use ast::Type::*;
        match ty {
            U8(_) => Layout::U8,
            I8(_) => Layout::I8,
            Array(array) => {
                Layout::Array { inner: Box::new(Self::from_type(&array.type_)),
                                len: super::expression::compute_const_expression(&array.len) }
            }
            Pointer(ptr) => Layout::Pointer(Box::new(Layout::from_type(&ptr.type_))),
            Struct(struct_) => Layout::Struct(struct_.fields
                                                     .iter()
                                                     .map(|f| &f.type_)
                                                     .map(Layout::from_type)
                                                     .collect()),
            Union(union) => Layout::Struct(union.fields
                                                .iter()
                                                .map(|f| &f.type_)
                                                .map(Layout::from_type)
                                                .collect()),
            _ => panic!("Type noy yet supported!"),
        }
    }

    /// Compute size of the type layout.
    pub fn compute_size(&self) -> u16 {
        match self {
            Layout::U8 | Layout::I8 => 1,
            Layout::Pointer(_) => 2,
            Layout::Array { inner, len } => len * inner.compute_size(),
            Layout::Struct(inner) => inner.iter().fold(0, |o, l| o + l.compute_size()),
            Layout::Union(inner) => inner.iter().fold(0, |o, l| o.max(l.compute_size())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ir::layout::Layout, parser::ast};

    #[test]
    fn test() {
        let type_ = match crate::parser::parse("let _:[&u8 (+ 2 2)] = 0").unwrap()
                                                                         .inner
                                                                         .pop()
                                                                         .unwrap()
        {
            ast::Statement::Let(let_) => let_.field.type_,
            _ => panic!(),
        };
        let layout = Layout::from_type(&type_);

        assert_eq!(Layout::Array { inner: Box::new(Layout::Pointer(Box::new(Layout::U8))),
                                   len: 4 },
                   layout);
    }
}
