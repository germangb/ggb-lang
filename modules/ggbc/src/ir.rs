//! Definition and compilation of IR.
use crate::{
    ir::{
        alloc::{Alloc, FnAlloc, Space},
        alloc_register::RegisterAlloc,
    },
    parser::{
        ast,
        ast::{Const, Expression, Fn, Let, Scope, Static, Type},
        Ast,
    },
};

mod alloc;
mod alloc_register;
mod utils;

pub type Address = u16;
pub type Register = usize;

/// Intermediate representation of a program.
pub struct Ir {
    /// Const memory space.
    pub const_: Vec<u8>,
    /// Compiled program routines.
    pub routines: Vec<Routine>,
    /// Interrupt handler routines.
    pub interrupts: Interrupts,
    /// Program entry point index.
    pub main: usize,
}

/// Routine handles for each type of interrupt.
#[derive(Default)]
pub struct Interrupts {
    /// VBlank interrupt.
    pub vblank: Option<usize>,
    /// LCD Status interrupt.
    pub lcd_stat: Option<usize>,
    /// Timer interrupt.
    pub timer: Option<usize>,
    /// Serial interrupt handler.
    pub serial: Option<usize>,
    /// Joypad interrupt.
    pub joypad: Option<usize>,
}

/// Data associated with a compiled IR routine.
pub struct Routine {
    /// Instructions of the routine.
    pub block: Vec<Statement>,
}

/// Virtual memory pointers.
#[derive(Debug)]
pub enum Pointer {
    Absolute(Address),
    Static(Address),
    Const(Address),
    Stack(Address),
}

/// Location of values referenced by IR statements.
#[derive(Debug)]
pub enum Data<T> {
    /// Data at the given address.
    Pointer(Pointer),
    /// Literal value.
    Literal(T),
}

/// Jump location of `Jmp` and `Cmp` statements.
#[derive(Debug)]
pub enum Jump {
    /// Jump relative to the current program pointer.
    Relative(i8),
}

/// The statements, or instruction set of the IR.
#[rustfmt::skip]
#[derive(Debug)]
pub enum Statement {
    // move data
    Ld     { source: Data<u8>,  destination: Data<u8> },
    Ld16   { source: Data<u16>, destination: Data<u16> },
    // move pointer
    // the variant Value::Literal should be illegal
    // TODO rethink Value enum variants.
    LdAddr { source: Pointer, destination: Pointer },

    // arithmetic (unary)
    Inc   { destination: Data<u8> },
    Inc16 { destination: Data<u8> },
    Dec   { destination: Data<u8> },
    Dec16 { destination: Data<u8> },

    // arithmetic (binary)
    Add { left: Data<u8>, right: Data<u8>, destination: Data<u8> },
    Sub { left: Data<u8>, right: Data<u8>, destination: Data<u8> },
    And { left: Data<u8>, right: Data<u8>, destination: Data<u8> },
    Xor { left: Data<u8>, right: Data<u8>, destination: Data<u8> },
    Or  { left: Data<u8>, right: Data<u8>, destination: Data<u8> },

    // flow control
    Jmp { target: Jump },
    Cmp { target: Jump, source: Data<u8> },

    // routines
    Call   {
        /// Routine index.
        routine: usize,
        /// Stack pointers (in the order they are declared).
        args: Vec<Address>,
    },
    Ret,
}

// TODO
pub fn compile(ast: &Ast) -> Ir {
    use ast::Statement::*;

    let mut alloc = Alloc::default();
    let mut routines = Vec::new();
    let mut statements = Vec::new();
    let mut fn_alloc = FnAlloc::default();

    for statement in &ast.inner {
        match statement {
            Static(static_) => compile_static(static_, &mut alloc),
            Const(const_) => compile_const(const_, &mut alloc),
            Let(let_) => compile_let(let_, &mut alloc, &fn_alloc, &mut statements),
            Fn(fn_) => compile_fn(fn_, &mut alloc, &mut fn_alloc),
            _ => {}
        }
    }

    for s in statements.iter() {
        println!("{:x?}", s);
    }
    println!("---");
    println!("{:?}", alloc);
    Ir {
        const_: Vec::new(),
        routines,
        interrupts: Interrupts::default(),
        main: 0,
    }
}

// compile "static" statement
fn compile_static<'a>(static_: &'a Static<'a>, alloc: &mut Alloc<'a>) {
    if let Some(offset) = &static_.offset {
        let offset = utils::compute_const_expression(&offset.expression);
        alloc.alloc_absolute(&static_.field, offset);
    } else {
        alloc.alloc_static(&static_.field);
    }
}

// compile "const" statement
fn compile_const<'a>(const_: &'a Const<'a>, alloc: &mut Alloc<'a>) {
    alloc.alloc_const(&const_.field, &const_.expr);
}

// compile "let" statement
fn compile_let<'a>(
    let_: &'a Let<'a>,
    alloc: &mut Alloc<'a>,
    fn_alloc: &FnAlloc,
    statements: &mut Vec<Statement>,
) {
    // allocate memory on the stack for this field
    // the compiled expression should store the result on the stack
    let stack_address = alloc.alloc_stack_field(&let_.field);
    compile_expression_into_stack(
        &let_.expr,
        alloc,
        fn_alloc,
        stack_address,
        &let_.field.type_,
        statements,
    );
}

// compile computation of expression represented by `expression` and store the
// result in the given stack address. Assume that the expression fits the
// allocated space in `stack`, and the size of the type is exactly `size` bytes.
//
// The memory space in the stack was allocated with the `alloc` allocator. In
// order to perform any intermediate expression compilations that require usage
// of the stack (such as a function call), you must crate a new allocator by
// cloning it, perform the compilation, then drop it, so the original stack
// pointer is preserved.
fn compile_expression_into_stack(
    expression: &Expression,
    alloc: &mut Alloc,
    fn_alloc: &FnAlloc,
    stack_address: u16,
    type_: &Type,
    statements: &mut Vec<Statement>,
) {
    match expression {
        // compile literal expression by simply move a literal value unto the stack address.
        // the size must be either a u8 or a u16 at this point. Any other value is wrong and the
        // compiler frontend should've caught it by now, hence the panic.
        Expression::Lit(lit) => {
            let lit = utils::compute_literal_as_numeric(lit);
            match type_ {
                Type::U8(_) => statements.push(Statement::Ld {
                    source: Data::Literal(lit as u8),
                    destination: Data::Pointer(Pointer::Stack(stack_address)),
                }),
                Type::I8(_) => unimplemented!("TODO i8"),
                Type::Pointer(_) => statements.push(Statement::Ld16 {
                    source: Data::Literal(lit),
                    destination: Data::Pointer(Pointer::Stack(stack_address)),
                }),
                _ => panic!(),
            }
        }
        Expression::Path(_) => {}
        Expression::Array(value) => match type_ {
            Type::Array(array) => {
                let array_type_size = utils::size_of(&array.type_);
                let array_len = utils::compute_const_expression(&array.len);

                assert_eq!(
                    array_len as usize,
                    value.inner.len(),
                    "array value length doesn't match the length in the type annotation"
                );

                for (i, expr) in value.inner.iter().enumerate() {
                    let stack_address = stack_address + array_type_size * (i as u16);
                    compile_expression_into_stack(
                        expr,
                        alloc,
                        fn_alloc,
                        stack_address,
                        &array.type_,
                        statements,
                    );
                }
            }
            _ => panic!(),
        },
        Expression::Minus(_) => {}
        Expression::AddressOf(address_of) => {
            match &address_of.inner {
                Expression::Path(path) => match type_ {
                    Type::Pointer(ptr) => {
                        let name = utils::path_to_symbol_name(&path);
                        let symbol = alloc.symbol(&name);
                        assert!(utils::equivalent_types(&ptr.type_, &symbol.type_));
                        let source = match symbol.space {
                            Space::Stack => Pointer::Stack(symbol.offset),
                            Space::Static => Pointer::Static(symbol.offset),
                            Space::Const => Pointer::Const(symbol.offset),
                            Space::Absolute => Pointer::Absolute(symbol.offset),
                        };
                        statements.push(Statement::LdAddr {
                            destination: Pointer::Stack(stack_address),
                            source,
                        });
                    }
                    _ => panic!(),
                },
                // TODO generalise (allow taking a pointer of something other than just a symbol)
                _ => unimplemented!(),
            }
        }
        Expression::Deref(_) => {}
        Expression::Not(_) => {}
        Expression::Add(_) => {}
        Expression::Sub(_) => {}
        Expression::Mul(_) => {}
        Expression::Div(_) => {}
        Expression::And(_) => {}
        Expression::Or(_) => {}
        Expression::Xor(_) => {}
        Expression::Assign(_) => {}
        Expression::PlusAssign(_) => {}
        Expression::MinusAssign(_) => {}
        Expression::MulAssign(_) => {}
        Expression::DivAssign(_) => {}
        Expression::AndAssign(_) => {}
        Expression::OrAssign(_) => {}
        Expression::XorAssign(_) => {}
        Expression::Index(_) => {}
        Expression::LeftShift(_) => {}
        Expression::RightShift(_) => {}
        Expression::Eq(_) => {}
        Expression::NotEq(_) => {}
        Expression::LessEq(_) => {}
        Expression::GreaterEq(_) => {}
        Expression::Less(_) => {}
        Expression::Greater(_) => {}
        Expression::Call(call) => match &call.inner.left {
            Expression::Path(ident) => {
                let fn_ = fn_alloc.get(&utils::path_to_symbol_name(&ident));
                let call_args = &call.inner.args;
                if let Some(sign_args) = &fn_.fn_arg {
                    let sign_args = &sign_args.inner;
                    assert_eq!(call_args.len(), sign_args.len());
                    let mut args = Vec::with_capacity(sign_args.len());
                    let mut offset = stack_address;

                    // lay functions arguments in the stack
                    let mut alloc = alloc.clone();
                    for (call_arg, sign_arg) in call_args.iter().zip(sign_args) {
                        compile_expression_into_stack(
                            call_arg,
                            &mut alloc,
                            fn_alloc,
                            offset,
                            &sign_arg.type_,
                            statements,
                        );
                        let type_size = utils::size_of(&sign_arg.type_);
                        args.push(offset);
                        offset += type_size;
                    }

                    statements.push(Statement::Call { routine: 0, args })
                } else {
                    assert!(call.inner.args.is_empty());
                    statements.push(Statement::Call {
                        routine: 0,
                        args: Vec::new(),
                    })
                }
            }
            _ => panic!(),
        },
    }
}

fn compile_fn<'a>(fn_: &'a Fn, alloc: &mut Alloc, fn_alloc: &mut FnAlloc<'a>) {
    fn_alloc.alloc(fn_);
}
