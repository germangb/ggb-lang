use std::collections::HashMap;

pub struct Compiled {
    /// Read-Only memory (ROM).
    pub data: Vec<u8>,
    /// All the compiled routines (including interrupts and main).
    pub routines: Vec<Routine>,
    /// Interrupt handler routine indices.
    pub interrupt_handlers: InterruptHandlers,
    /// Index of the entry point routine.
    pub main: usize,
}

pub struct InterruptHandlers {
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

pub type IrBlock = Vec<Ir>;

pub struct Routine {
    block: Vec<IrBlock>,
}

pub enum Register<T> {
    /// Register at the given address (pointer).
    Address(u16),
    /// Address relative to stack pointer.
    Stack(i8),
    /// Cpu register index.
    Cpu(usize),
    /// Immediate value.
    Immediate(T),
}

pub enum Target {
    /// Jump to the given block.
    Block(usize),
    /// Relative jump.
    Relative(i8),
}

// # Example
//
// fn mul(a:u8 b:u8):u8 {
//     (&= a 0xf)
//     (&= b 0xf)
//     let tmp:u8 = 0
//     for _i:u8 in 0..b {
//         (+= tmp a)
//     }
//     return tmp
// }
//
// ## stack on entry
//
// (sp-2): a
// (sp-1): b
//
// ## stack on exit
//
// (sp-1): tmp
//
// ## routine
//
// mul: Add  { destination: Stack(-2), left: Stack(-2), right: Immediate(0xf) }
//      Add  { destination: Stack(-1), left: Stack(-1), right: Immediate(0xf) }
//
//      // .. (evaluate tmp and store in Cpu(0))
//      Push { source: Cpu(0) }
//
//      // .. (evaluate 0.. and store in Cpu(0))
//      Push { source: Cpu(0) }
//
//      // ... (evaluate ..b, store in Cpu(0)
//      Sub  { destination: Cpu(0),    left: Cpu(0),    right: Stack(-1) }
//      Cmp  { target: Relative(4), source: Cpu(0) }
//
//      // loop block contents
//      Add  { destination: Stack(-2), left: Stack(-2), right: Stack(-4) }
//      Inc  { destination: Stack(-1) }
//      Jmp  { target: Relative(-3) }
//
//      // code after loop
//      Ld   { source: Stack(-2), destination: Cpu(0) }
//
//      // remove a, b, tmp, _i from the stack
//      Pop
//      Pop
//      Pop
//      Pop
//      Push { source: Cpu(0) }
//      Ret
#[rustfmt::skip]
pub enum Ir {
    // move
    Ld   { source: Register<u8>,  destination: Register<u8> },
    Ld16 { source: Register<u16>, destination: Register<u16> },

    // arithmetic (unary)
    Inc   { destination: Register<u8> },
    Inc16 { destination: Register<u8> },
    Dec   { destination: Register<u8> },
    Dec16 { destination: Register<u8> },

    // arithmetic (binary)
    Add { left: Register<u8>, right: Register<u8>, destination: Register<u8> },
    Sub { left: Register<u8>, right: Register<u8>, destination: Register<u8> },
    And { left: Register<u8>, right: Register<u8>, destination: Register<u8> },
    Xor { left: Register<u8>, right: Register<u8>, destination: Register<u8> },
    Or  { left: Register<u8>, right: Register<u8>, destination: Register<u8> },

    // stack
    Push   { source: Register<u8> },
    Push16 { source: Register<u16> },
    Pop    { destination: Register<u8> },
    Pop16  { destination: Register<u8> },

    // flow control
    Jmp { target: Target },
    Cmp { target: Target, source: Register<u8> },

    // routines
    Call   { routine: usize },
    Ret,
    CmpRet { source: Register<u8> }
}
