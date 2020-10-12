use crate::Error;
use ggbc_parser::{
    ast::{Expression, Type},
    lex::Ident,
};

const POINTER_SIZE: u16 = 2; // 2 bytes
const BYTE_SIZE: u16 = 1; // 1 byte

/// Compute the size of a given type in bytes.
pub fn compute_type_size<'a>(type_: &Type<'a>) -> Result<u16, Error<'a>> {
    let size = match type_ {
        Type::U8(_) | Type::I8(_) => BYTE_SIZE,
        Type::Array(array) => {
            compute_type_size(&array.type_)? * compute_const_expression(&array.len)?
        }
        // the sum of all fields
        Type::Struct(struct_) => struct_.fields.iter().fold(Ok(0), |accum, field| {
            accum.and_then(|accum| Ok(accum + compute_type_size(&field.type_)?))
        })?,
        // compute the max of all the fields within:
        Type::Union(union_) => union_.fields.iter().fold(Ok(0), |max, field| {
            max.and_then(|max| Ok(max.max(compute_type_size(&field.type_)?)))
        })?,
        Type::Path(_) => unimplemented!(),
        // functions are just pointers
        Type::Pointer(_) | Type::Fn(_) => POINTER_SIZE,
    };
    Ok(size)
}

/// Compute the value of a constant expression.
/// Used mainly to compute the size of array types.
pub fn compute_const_expression<'a>(expression: &Expression) -> Result<u16, Error<'a>> {
    match expression {
        Expression::Lit(lit) => {
            if lit.as_str().starts_with("0x") {
                Ok(u16::from_str_radix(&lit.as_str()[2..], 16).expect("Error parsing hex literal"))
            } else {
                Ok(lit.to_string().parse().expect("Error parsing as numeric"))
            }
        }
        _ => Err(Error::InvalidExpression {
            expression: unimplemented!(),
            reason: None,
        }),
    }
}

/// compute mangled name of the given identifier.
pub fn compute_symbol_name<'a>(ident: &Ident<'a>) -> Result<String, Error<'a>> {
    Ok(ident.as_str().to_string())
}
