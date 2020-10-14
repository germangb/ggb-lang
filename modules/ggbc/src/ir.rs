//! Intermediate representation.
use crate::parser::{ast, Ast};

pub type Pointer = u16;

pub struct Ir {
    /// Const memory space.
    pub const_: Vec<u8>,
    /// Compiled program routines.
    pub routines: Vec<Routine>,
    /// Interrupt handler routines.
    pub interrupt_handlers: Handlers,
    /// Program entry point index.
    pub main: usize,
}

#[derive(Default)]
pub struct Handlers {
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

pub struct Routine {
    /// Instructions of the routine.
    pub block: Vec<Instruction>,
}

pub enum Location<T> {
    /// Pointer within static memory space.
    Static(Pointer),
    /// Pointer within const (ROM) memory space.
    Const(Pointer),
    /// Address relative to the current stack pointer.
    Stack(Pointer),
    /// Virtual physical register.
    Physical(usize),
    /// Literal value.
    Literal(T),
}

pub enum Jump {
    /// Jump relative to the current program pointer.
    Relative(i8),
}

#[rustfmt::skip]
pub enum Instruction {
    // move
    Ld   { source: Location<u8>,  destination: Location<u8> },
    Ld16 { source: Location<u16>, destination: Location<u16> },

    // arithmetic (unary)
    Inc   { destination: Location<u8> },
    Inc16 { destination: Location<u8> },
    Dec   { destination: Location<u8> },
    Dec16 { destination: Location<u8> },

    // arithmetic (binary)
    Add { left: Location<u8>, right: Location<u8>, destination: Location<u8> },
    Sub { left: Location<u8>, right: Location<u8>, destination: Location<u8> },
    And { left: Location<u8>, right: Location<u8>, destination: Location<u8> },
    Xor { left: Location<u8>, right: Location<u8>, destination: Location<u8> },
    Or  { left: Location<u8>, right: Location<u8>, destination: Location<u8> },

    // stack
    Push   { source: Location<u8> },
    Push16 { source: Location<u16> },
    Pop    { destination: Location<u8> },
    Pop16  { destination: Location<u8> },

    // flow control
    Jmp { target: Jump },
    Cmp { target: Jump, source: Location<u8> },

    // routines
    Call   {
        /// Routine index.
        routine: usize,
        /// Stack pointers (in the order they are declared).
        args: Vec<Pointer>,
    },
    Ret,
}

pub mod utils;

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
            ast::Statement::Let(ast::Let { field, .. }) => {
                alloc.alloc_stack(field);
            }
            _ => {}
        }
    }

    println!("{:#04x?}", alloc);
    routines.push(Routine { block: main });
    Ir {
        const_: Vec::new(),
        routines,
        interrupt_handlers: Handlers::default(),
        main: 0,
    }
}

fn compile_routine(fn_: &ast::Fn, alloc: &utils::Alloc) -> Routine {
    let mut block = Vec::new();
    Routine { block }
}

fn compile_expr(expr: &ast::Expression, alloc: &utils::Alloc, instructions: &mut Vec<Instruction>) {
}
