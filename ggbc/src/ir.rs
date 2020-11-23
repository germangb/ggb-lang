//! Intermediate representation language.
use crate::{
    byteorder::{ByteOrder, NativeEndian},
    ir::compile::{Compile, Context},
    parser::ast,
};
pub use opcodes::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

mod alloc;
mod compile;
mod expression;
mod layout;
mod opcodes;
mod optimize;

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
        let mut context: Context<B> = Context::default();
        let mut main = Vec::new();

        context.optimize = true;
        ast.compile(&mut context, &mut main);

        let main_handle = context.routines.len();
        context.routines
               .push(Routine { debug_name: Some("main".to_string()),
                               statements: main });

        Self { const_: context.symbol_alloc.into_const_data().into_boxed_slice(),
               routines: context.routines.into_boxed_slice(),
               handlers: Handlers { main: main_handle,
                                    ..Default::default() },
               _phantom: PhantomData }
    }

    /// MAIN handler routine.
    pub fn main(&self) -> &Routine {
        &self.routines[self.handlers.main]
    }

    /// VBLANK handler routine.
    pub fn vblank(&self) -> Option<&Routine> {
        self.handlers.vblank.map(|i| &self.routines[i])
    }

    /// LCD STAT handler routine.
    pub fn lcd_stat(&self) -> Option<&Routine> {
        self.handlers.lcd_stat.map(|i| &self.routines[i])
    }

    /// TIMER handler routine.
    pub fn timer(&self) -> Option<&Routine> {
        self.handlers.timer.map(|i| &self.routines[i])
    }

    /// SERIAL handler routine.
    pub fn serial(&self) -> Option<&Routine> {
        self.handlers.serial.map(|i| &self.routines[i])
    }

    /// JOYPAD handler routine.
    pub fn joypad(&self) -> Option<&Routine> {
        self.handlers.joypad.map(|i| &self.routines[i])
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
