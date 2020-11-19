//! Intermediate representation language.
use crate::{
    byteorder::{ByteOrder, NativeEndian},
    parser::ast,
};
use alloc::{FnAlloc, RegisterAlloc, SymbolAlloc};
pub use opcodes::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

mod alloc;
mod expression;
mod layout;
mod opcodes;
mod optimize;
mod statements;

/// Intermediate representation of a program.
///
/// Generic over the byte ordering `B` of the bytes in `const_`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Ir<B: ByteOrder = NativeEndian> {
    /// ROM memory.
    /// Compiled from `Const` AST statements.
    pub const_: Box<[u8]>,

    /// Compiled routines.
    pub routines: Box<[Routine]>,

    /// Handlers for entry point and interrupts.
    pub handlers: Handlers,

    _phantom: PhantomData<B>,
}

impl<B: ByteOrder> Ir<B> {
    /// Convert AST into IR intermediate code.
    pub fn new(ast: &ast::Ast<'_>) -> Self {
        use Statement::{Nop, Stop};

        //let mut context = Context::default();
        let mut routines = Vec::new();
        let mut register_alloc = RegisterAlloc::default();
        let mut symbol_alloc: SymbolAlloc<B> = SymbolAlloc::default();
        let mut fn_alloc = FnAlloc::default();
        let mut statements = Vec::new();

        // begin program with a NOP statement, just in case the first statement happens
        // to be a loop. otherwise the PC might lang in an out of bounds location.
        statements.push(Nop(NOP_PERSIST));
        statements::compile_statements(&ast.inner,
                                       None,
                                       true,
                                       &mut register_alloc,
                                       &mut symbol_alloc,
                                       &mut fn_alloc,
                                       &mut statements,
                                       &mut routines);
        statements.push(Stop);

        // optimize main statements
        optimize::optimize(&mut statements);

        let main = routines.len();
        routines.push(Routine { debug_name: None,
                                statements });

        Self { const_: symbol_alloc.into_const_data().into_boxed_slice(),
               routines: routines.into_boxed_slice(),
               handlers: Handlers { main,
                                    ..Default::default() },
               _phantom: PhantomData }
    }
}

/// Handlers for the main routine and interrupt handlers.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct Handlers {
    /// Index of the entry point routine.
    pub main: usize,

    /// VBLANK interrupt handler.
    pub vblank: Option<usize>,

    /// LCD-STAT interrupt handler.
    pub lcd_stat: Option<usize>,

    /// TIMER interrupt handler.
    pub timer: Option<usize>,

    /// SERIAL interrupt handler.
    pub serial: Option<usize>,

    /// JOYPAD interrupt handler.
    pub joypad: Option<usize>,
}

/// Data associated with a compiled IR routine.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Routine {
    /// Optional routine name (for debugging purposes).
    pub debug_name: Option<String>,

    /// Instructions of the routine.
    pub statements: Vec<Statement>,
}
