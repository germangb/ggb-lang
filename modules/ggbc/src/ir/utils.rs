use crate::parser::{
    ast::{Expression, Path, Type},
    lex::Lit,
};

/// Convert path into symbol name.
pub fn path_to_symbol_name(path: &Path) -> String {
    let mut items = path.iter();
    let name = items.next().unwrap().to_string();
    items.fold(name, |mut o, ident| {
        o.push_str("::");
        o.push_str(ident.as_str());
        o
    })
}

pub fn compute_literal_as_numeric(lit: &Lit) -> u16 {
    let num = lit.to_string();
    if num.starts_with("0x") {
        u16::from_str_radix(&num[2..], 16).expect("Not a hex number")
    } else if num.as_bytes()[0].is_ascii_digit() {
        num.parse().expect("Not a number")
    } else {
        panic!("Not a number literal")
    }
}

/// Compute the size of a given constant (numeric) expression.
/// Panics if the expression is not a constant expression nor numeric.
pub fn compute_const_expression(expr: &Expression) -> u16 {
    use Expression::*;

    match expr {
        Lit(e) => compute_literal_as_numeric(e),
        Minus(_e) => unimplemented!("TODO"),
        Not(e) => !compute_const_expression(&e.inner),
        Add(e) => {
            compute_const_expression(&e.inner.left) + compute_const_expression(&e.inner.right)
        }
        Sub(e) => {
            compute_const_expression(&e.inner.left) - compute_const_expression(&e.inner.right)
        }
        Mul(e) => {
            compute_const_expression(&e.inner.left) * compute_const_expression(&e.inner.right)
        }
        Div(e) => {
            compute_const_expression(&e.inner.left) / compute_const_expression(&e.inner.right)
        }
        And(e) => {
            compute_const_expression(&e.inner.left) & compute_const_expression(&e.inner.right)
        }
        Or(e) => compute_const_expression(&e.inner.left) | compute_const_expression(&e.inner.right),
        Xor(e) => {
            compute_const_expression(&e.inner.left) ^ compute_const_expression(&e.inner.right)
        }
        LeftShift(e) => {
            compute_const_expression(&e.inner.left) << compute_const_expression(&e.inner.right)
        }
        RightShift(e) => {
            compute_const_expression(&e.inner.left) >> compute_const_expression(&e.inner.right)
        }
        _ => panic!("Not a constant expression"),
    }
}
