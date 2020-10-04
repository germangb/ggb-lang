//! Intermediate representation.
pub struct Ir {
    /// Allocated ROM memory block.
    pub rom: Vec<u8>,
    /// Memory block allocated at ggbc-ir time.
    /// This is where all static values lie.
    pub static_: Vec<u8>,
    /// List of statements.
    pub statements: Vec<Statement>,
    /// Index of the instructions's entry point.
    pub entry_point: usize,
}

pub struct Statement {
    /// Statement operation.
    pub instruction: Instruction,
    /// Next instruction.
    pub next: usize,
    /// (For conditional ops).
    /// Next instruction if confition is met.
    pub next_condition: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Register {
    /// Register index.
    Register(usize),
    /// Pointer in stack memory.
    Stack(u16),
    /// Pointer in static memory.
    Static(u16),
    /// Pointer in ROM memory.
    Rom(u16),
    /// Literal value.
    Literal(u8),
}

#[rustfmt::skip]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Instruction {
    // Misc

    /// Placeholder instruction meant to be used mainly to signal that an
    /// instruction should be removed.
    Nop,
    /// Halt execution indefinitely.
    Stop,

    // Register load instructions

    /// Load value from `src` into `dst` register.
    Ld  { src: Register, dst: Register },

    // Arithmetic instructions.
    Add { dst: Register, left: Register, right: Register },
    Sub { dst: Register, left: Register, right: Register },
    And { dst: Register, left: Register, right: Register },
    Xor { dst: Register, left: Register, right: Register },
    Or  { dst: Register, left: Register, right: Register },
    Lsh { dst: Register, left: Register, right: Register },
    Rsh { dst: Register, left: Register, right: Register },

    // Flow control

    /// Jump to label if `src != 0`.
    /// Continue execution otherwise.
    Ifnz { src: Register },

    // Routine instructions

    /// Issue call to routine with the given arguments.
    Call { args: Vec<Register> },
    /// Return from routine.
    /// The value, if any, will be located in the given register.
    Ret { dst: Option<Register> }
}
