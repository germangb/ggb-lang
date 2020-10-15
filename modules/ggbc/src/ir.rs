//! Definition and compilation of IR.
use crate::parser::{ast, Ast};
use ggbc_parser::ast::Expression;

pub type Pointer = u16;

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
///
/// Each handle represents an index within a `Vec` of routines.
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

/// Location of values referenced by IR statements.
#[derive(Debug)]
pub enum Value<T> {
    /// Pointer within static memory space.
    Static(Pointer),
    /// Pointer within const (ROM) memory space.
    Const(Pointer),
    /// Address relative to the current stack pointer.
    Stack(Pointer),
    /// Virtual register.
    Virtual(usize),
    /// Literal value.
    Literal(T),
}

/// Jump location of `Jmp` and `Cmp` statements.
#[derive(Debug)]
pub enum JumpTo {
    /// Jump relative to the current program pointer.
    Relative(i8),
}

/// The statements, or instruction set of the IR.
#[rustfmt::skip]
#[derive(Debug)]
pub enum Statement {
    // move
    Ld   { source: Value<u8>,  destination: Value<u8> },
    Ld16 { source: Value<u16>, destination: Value<u16> },

    // arithmetic (unary)
    Inc   { destination: Value<u8> },
    Inc16 { destination: Value<u8> },
    Dec   { destination: Value<u8> },
    Dec16 { destination: Value<u8> },

    // arithmetic (binary)
    Add { left: Value<u8>, right: Value<u8>, destination: Value<u8> },
    Sub { left: Value<u8>, right: Value<u8>, destination: Value<u8> },
    And { left: Value<u8>, right: Value<u8>, destination: Value<u8> },
    Xor { left: Value<u8>, right: Value<u8>, destination: Value<u8> },
    Or  { left: Value<u8>, right: Value<u8>, destination: Value<u8> },

    // stack
    Push   { source: Value<u8> },
    Push16 { source: Value<u16> },
    Pop    { destination: Value<u8> },
    Pop16  { destination: Value<u8> },

    // flow control
    Jmp { target: JumpTo },
    Cmp { target: JumpTo, source: Value<u8> },

    // routines
    Call   {
        /// Routine index.
        routine: usize,
        /// Stack pointers (in the order they are declared).
        args: Vec<Pointer>,
    },
    Ret,
}

mod utils;

// TODO(german)
//  compilation to IR should be infallible. All the syntax checks must be
//  performed in the parser step, though (which is not the case yet as of yet)
pub fn compile(ast: &Ast) -> Ir {
    let mut alloc = utils::Alloc::default();
    let mut routines = Vec::new();
    let mut main = Vec::new();

    for statement in &ast.inner {
        match statement {
            ast::Statement::Fn(fn_) => {
                routines.push(compile_routine(fn_, &alloc));
            }
            ast::Statement::Static(ast::Static {
                field,
                offset: Some(offset),
                ..
            }) => {
                alloc.alloc_static_at(field, utils::compute_const_expr(&offset.expression));
            }
            ast::Statement::Static(ast::Static { field, .. }) => {
                alloc.alloc_static(field);
            }
            ast::Statement::Const(ast::Const { field, expr, .. }) => {
                alloc.alloc_const(field, expr);
            }
            ast::Statement::Let(ast::Let { field, expr, .. }) => {
                let mut register_alloc = utils::RegisterAlloc::default();
                alloc.alloc_stack(field);
                let register = compile_expression_8(expr, &alloc, &mut register_alloc, &mut main);
                main.push(Statement::Ld {
                    source: Value::Virtual(register),
                    destination: Value::Stack(0),
                });
            }
            _ => {}
        }
    }

    println!("{:#04x?}", alloc);
    for i in &main {
        println!("{:?}", i);
    }
    routines.push(Routine { block: main });
    Ir {
        const_: Vec::new(),
        routines,
        interrupts: Interrupts::default(),
        main: 0,
    }
}

fn compile_routine(fn_: &ast::Fn, alloc: &utils::Alloc) -> Routine {
    let mut block = Vec::new();
    Routine { block }
}

// assignment: E = E
//  - E = E
//  - E[E] = E
//  - *E = E
// binary: E + E
// function: E(E, ...)
fn compile_expression(
    expression: &ast::Expression,
    alloc: &utils::Alloc,
    register_alloc: &mut utils::RegisterAlloc,
    instructions: &mut Vec<Statement>,
) -> usize {
    unimplemented!()
}

/// Compile expression that resolves to a word (16bit).
fn compile_expression_16(
    expression: &ast::Expression,
    alloc: &utils::Alloc,
    register_alloc: &mut utils::RegisterAlloc,
    instructions: &mut Vec<Statement>,
) -> usize {
    unimplemented!()
}

/// Compile expression that resolves to a byte.
fn compile_expression_8(
    expression: &ast::Expression,
    alloc: &utils::Alloc,
    register_alloc: &mut utils::RegisterAlloc,
    instructions: &mut Vec<Statement>,
) -> usize {
    match expression {
        Expression::Path(_) => {}
        Expression::Lit(node) => {
            let lit: u8 = node.as_str().parse().unwrap();
            let register = register_alloc.min();
            register_alloc.set(register, true);
            instructions.push(Statement::Ld {
                source: Value::Literal(lit),
                destination: Value::Virtual(register),
            });
            return register;
        }
        Expression::Array(_) => {}
        Expression::Minus(_) => {}
        Expression::AddressOf(_) => {}
        Expression::Deref(_) => {}
        Expression::Not(_) => {}
        Expression::Add(node) => {
            let left = compile_expression_8(&node.inner.left, alloc, register_alloc, instructions);
            let right =
                compile_expression_8(&node.inner.right, alloc, register_alloc, instructions);
            register_alloc.set(left, false);
            register_alloc.set(right, false);
            let register = left.min(right);
            register_alloc.set(register, true);
            instructions.push(Statement::Add {
                destination: Value::Virtual(register),
                left: Value::Virtual(left),
                right: Value::Virtual(right),
            });
            return register;
        }
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
        Expression::Call(_) => {}
    }
    panic!()
}
