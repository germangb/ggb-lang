pub use crate::{error::Error, ir::Compiled};
use ggbc_parser::{
    ast::{Expression, Statement, Static, Type},
    Ast,
};
use std::collections::HashMap;

pub mod error;
pub mod ir;
mod utils;

enum StaticOffset {
    /// Address relative to the lowest reserved static address.
    /// This is RAM by default, but may be configured by the programmer.
    Relative(u16),
    /// Absolute offset (for then static statements are declared as
    /// `static@<offset>`.
    Absolute(u16),
}

// ROM [0x0100, 0x7fff] (32768 bytes)
// RAM [0xc000, 0xdfff] ( 8192 bytes)
// RAM [0xff80, 0xfffe] (  127 bytes) (stack) (fastest memory)
#[derive(Default)]
struct Context {
    // static memory allocation.
    // memory declared using the `static` statement.
    static_: HashMap<String, StaticOffset>, /* mangled name and offset of each static relative
                                             * to the lowest of the dedicated addresses (RAM
                                             * region by default) */
    static_alloc: u16, /* # of bytes of static memory allocated so far (this the memory
                        * allocated at compile time) */
}

pub fn compile(input: &str) -> Result<Compiled, Error> {
    let ast = ggbc_parser::parse(input).expect("Parsing error");
    let mut context = Context::default();

    // allocate static memory (RAM)
    let ast = alloc_static_memory(ast, &mut context)?;
    // compile functions
    // compile entry point (main)
    unimplemented!()
}

// Allocate memory declared using the "static" statement.
// There are two variants of static statements, one tells the compiler to
// allocate, the other doesn't.
// - Allocated:     static FOO:[u8 0x1000]
// - Non-Allocated: static BAR@0x8800:u8
fn alloc_static_memory<'a>(ast: Ast<'a>, context: &mut Context) -> Result<Ast<'a>, Error<'a>> {
    for statement in &ast.inner {
        match statement {
            // compiler does NOT allocate the memory, but rather, it is up to the programmer to
            // assign it a specific location in memory.
            Statement::Static(Static {
                offset: Some(offset),
                field,
                ..
            }) => {
                let name = utils::compute_symbol_name(&field.ident)?;
                let offset = utils::compute_const_expression(&offset.expression)?;
                let type_size = utils::compute_type_size(&field.type_)?;
                assert!(context
                    .static_
                    .insert(name.clone(), StaticOffset::Absolute(offset))
                    .is_none());
                eprintln!("Allocated {} @ {:x} ({} bytes)", name, offset, type_size);
            }
            // compiler allocated the memory.
            Statement::Static(Static { field, .. }) => {
                let name = utils::compute_symbol_name(&field.ident)?;
                let offset = context.static_alloc;
                let type_size = utils::compute_type_size(&field.type_)?;
                assert!(context
                    .static_
                    .insert(name.clone(), StaticOffset::Relative(offset))
                    .is_none());

                eprintln!(
                    "Allocated {} @ {:x} ({} bytes)",
                    name, context.static_alloc, type_size
                );

                context.static_alloc += type_size;
            }
            _ => {}
        }
    }
    Ok(ast)
}
