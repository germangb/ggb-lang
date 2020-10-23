//! Definition and compilation of IR.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod alloc;
mod compile;
mod layout;
mod utils;

pub use compile::compile;

pub type Address = u16;
pub type Register = usize;

/// Intermediate representation of a program.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Routine {
    /// Instructions of the routine.
    pub statements: Vec<Statement>,
}

/// Virtual memory pointers.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Pointer {
    /// Absolute pointer.
    Absolute(Address),
    /// Pointer in virtual static memory.
    Static(Address),
    /// Pointer in virtual const (ROM) memory.
    Const(Address),
    /// Pointer in virtual stack memory.
    Stack(Address),
}

/// Source from where to pull a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Source<T> {
    /// Data at the given address.
    Pointer(Pointer),
    /// Data at the given register.
    Register(Register),
    /// Literal data.
    Literal(T),
}

/// Destination where to store a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Destination {
    /// Store at the given address
    Pointer(Pointer),
    /// Store at the given register.
    Register(Register),
}

/// Jump location of `Jmp` and `Cmp` statements.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Target {
    /// Jump relative to the current program pointer.
    Relative(i8),
}

/// The statements, or instruction set of the IR.
#[rustfmt::skip]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum Statement {
    Nop,

    // move/load instructions
    Ld     { source: Source<u8>,  destination: Destination },
    Ld16   { source: Source<u16>, destination: Destination },
    // move/load of memory addresses
    LdAddr { source: Source<u16>, destination: Destination },

    // arithmetic (unary)
    Inc   { destination: Destination },
    Inc16 { destination: Destination },
    Dec   { destination: Destination },
    Dec16 { destination: Destination },

    // arithmetic (binary)
    Add { left: Source<u8>, right: Source<u8>, destination: Destination },
    Sub { left: Source<u8>, right: Source<u8>, destination: Destination },
    And { left: Source<u8>, right: Source<u8>, destination: Destination },
    Xor { left: Source<u8>, right: Source<u8>, destination: Destination },
    Or  { left: Source<u8>, right: Source<u8>, destination: Destination },

    // The following instructions may not be available in target architectures through a native instruction
    // In those cases, codegen must decompose to the corresponding equivalent bitwise ops (shifts & ands)
    // I haven't decided yet, but I may just reject the statement if none of the factors is a constexpr...
    Mul { left: Source<u8>, right: Source<u8>, destination: Destination },  // multiply
    Div { left: Source<u8>, right: Source<u8>, destination: Destination },  // division
    Rem { left: Source<u8>, right: Source<u8>, destination: Destination },  // Remainder

    // flow control
    Jmp { target: Target },
    /// jump to target if source == 0
    Cmp    { target: Target, source: Source<u8> },
    /// jump to target if source != 0
    CmpNot { target: Target, source: Source<u8> },

    /// Push stack frame.
    Push,
    /// Pop stack frame.
    Pop,

    // routines
    Call { routine: usize, args: Vec<Address>, destination: Option<Destination> },
    Ret,
}
