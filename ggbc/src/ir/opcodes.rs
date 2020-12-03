#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result},
    ops::Range,
};

/// Virtual memory address type.
pub type Address = u16;

/// Virtual register index.
pub type Register = usize;

/// Virtual memory pointers.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Pointer {
    /// Absolute pointer.
    Absolute(Address),

    /// Pointer in virtual static memory.
    Static(Address),

    /// Pointer in virtual const (ROM) memory.
    Const(Address),

    /// Pointer in virtual stack memory.
    Stack(Address),

    /// Function return space.
    Return(Address),
}

impl Pointer {
    pub(crate) fn offset(self, offset: Address) -> Self {
        use Pointer::{Absolute, Const, Return, Stack, Static};
        match self {
            Absolute(a) => Absolute(a + offset),
            Static(a) => Static(a + offset),
            Const(a) => Const(a + offset),
            Stack(a) => Stack(a + offset),
            Return(a) => Return(a + offset),
        }
    }
}

/// Source from where to pull a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Source<T> {
    /// Data at the given address.
    Pointer {
        /// The base pointer itself.
        base: Pointer,

        /// Dynamic applied to the address of the pointer.
        offset: Option<Box<Source<u8>>>,
    },

    /// Data at the given register.
    Register(Register),

    /// Literal data.
    Literal(T),
}

/// Destination where to store a value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Destination {
    /// Store at the given address
    Pointer {
        /// The base pointer itself.
        base: Pointer,

        /// Dynamic applied to the address of the pointer.
        offset: Option<Box<Source<u8>>>,
    },

    /// Store at the given register.
    Register(Register),
}

/// Jump location of `Jmp` and `Cmp` statements.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Location {
    /// Jump relative to the current program pointer.
    Relative(i8),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StopStatus {
    Error,
    Success,
}

/// The statements, or instruction set of the IR.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Statement {
    /// Do nothing, used as placeholder.
    Nop(usize),

    /// Statement to stop execution (end program).
    Stop(StopStatus),

    /// 8bit load.
    Ld {
        source: Source<u8>,
        destination: Destination,
    },

    /// 16bit load.
    LdW {
        source: Source<u16>,
        destination: Destination,
    },

    /// Load address.
    LdAddr {
        source: Source<Address>,
        destination: Destination,
    },

    /// 8bit increment.
    Inc {
        source: Source<u8>,
        destination: Destination,
    },

    /// 8bit decrement.
    Dec {
        source: Source<u8>,
        destination: Destination,
    },

    /// 16bit increment.
    IncW {
        source: Source<u16>,
        destination: Destination,
    },

    /// 16bit decrement.
    DecW {
        source: Source<u16>,
        destination: Destination,
    },

    /// 8bit add.
    Add {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit subtract.
    Sub {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit bitwise AND.
    And {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit bitwise XOR.
    Xor {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit bitwise OR.
    Or {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit left shift.
    LeftShift {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit right shift.
    RightShift {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit multiply.
    Mul {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit divide.
    Div {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit remainder.
    Rem {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 16bit add.
    AddW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 16bit subtract.
    SubW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 16bit bitwise AND.
    AndW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 16bit bitwise XOR.
    XorW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 16bit bitwise OR.
    OrW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 16bit left shift.
    LeftShiftW {
        left: Source<u16>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 16bit right shift.
    RightShiftW {
        left: Source<u16>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 16bit multiply.
    MulW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 16bit divide.
    DivW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 16bit remainder.
    RemW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },

    /// 8bit boolean equals.
    Eq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit boolean not-equals.
    NotEq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit boolean greater-than.
    Greater {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit boolean greater-or-equal-than.
    GreaterEq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit boolean less-than.
    Less {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// 8bit boolean less-or-equal-than.
    LessEq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    /// Jump to location.
    Jmp { location: Location },

    /// Conditional jump to location.
    /// Jumps to the given location if `source` resolves to zero.
    JmpCmp {
        location: Location,
        source: Source<u8>,
    },

    /// Conditional jump
    /// Jumps to the given location if `source` resolves to non-zero.
    JmpCmpNot {
        location: Location,
        source: Source<u8>,
    },

    /// Routine call.
    Call {
        /// Routine index.
        routine: usize,

        /// range of the current stack frame corresponding to the beginning of
        /// the new function's stack frame.
        range: Range<u16>,
    },

    /// Return from routine.
    Ret,
}

impl Statement {
    /// Return displayable mnemonic.
    pub fn display(&self) -> Mnemonic<'_> {
        Mnemonic { statement: self }
    }
}

/// Displayable statement mnemonic.
#[derive(Debug)]
pub struct Mnemonic<'a> {
    statement: &'a Statement,
}

impl Display for Mnemonic<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.statement {
            Statement::Nop(i) => write!(f, "NOP {}", i),
            Statement::Stop(_) => write!(f, "STOP"),
            Statement::Ld {
                source,
                destination,
            } => write_unary(f, "LD", source, destination),
            Statement::LdW {
                source,
                destination,
            } => write_unary(f, "LDW", source, destination),
            Statement::LdAddr { .. } => todo!(),
            Statement::Inc {
                source,
                destination,
            } => write_unary(f, "INC", source, destination),
            Statement::Dec {
                source,
                destination,
            } => write_unary(f, "DEC", source, destination),
            Statement::IncW {
                source,
                destination,
            } => write_unary(f, "INCW", source, destination),
            Statement::DecW {
                source,
                destination,
            } => write_unary(f, "DECW", source, destination),
            Statement::Add {
                left,
                right,
                destination,
            } => write_binary(f, "ADD", left, right, destination),
            Statement::Sub {
                left,
                right,
                destination,
            } => write_binary(f, "SUB", left, right, destination),
            Statement::And {
                left,
                right,
                destination,
            } => write_binary(f, "ADD", left, right, destination),
            Statement::Xor {
                left,
                right,
                destination,
            } => write_binary(f, "XOR", left, right, destination),
            Statement::Or {
                left,
                right,
                destination,
            } => write_binary(f, "OR", left, right, destination),
            Statement::LeftShift {
                left,
                right,
                destination,
            } => write_binary(f, "LEFTSHIFT", left, right, destination),
            Statement::RightShift {
                left,
                right,
                destination,
            } => write_binary(f, "RIGHTSHIFT", left, right, destination),
            Statement::AddW {
                left,
                right,
                destination,
            } => write_binary(f, "ADDW", left, right, destination),
            Statement::SubW {
                left,
                right,
                destination,
            } => write_binary(f, "SUBW", left, right, destination),
            Statement::AndW {
                left,
                right,
                destination,
            } => write_binary(f, "ANDW", left, right, destination),
            Statement::XorW {
                left,
                right,
                destination,
            } => write_binary(f, "XORW", left, right, destination),
            Statement::OrW {
                left,
                right,
                destination,
            } => write_binary(f, "ORW", left, right, destination),
            Statement::LeftShiftW {
                left,
                right,
                destination,
            } => write_binary(f, "LEFTSHIFTW", left, right, destination),
            Statement::RightShiftW {
                left,
                right,
                destination,
            } => write_binary(f, "RIGHTSHIFTW", left, right, destination),
            Statement::Mul {
                left,
                right,
                destination,
            } => write_binary(f, "MUL", left, right, destination),
            Statement::Div {
                left,
                right,
                destination,
            } => write_binary(f, "DIV", left, right, destination),
            Statement::MulW {
                left,
                right,
                destination,
            } => write_binary(f, "MULW", left, right, destination),
            Statement::DivW {
                left,
                right,
                destination,
            } => write_binary(f, "DIVW", left, right, destination),
            Statement::RemW {
                left,
                right,
                destination,
            } => write_binary(f, "REMW", left, right, destination),
            Statement::Rem {
                left,
                right,
                destination,
            } => write_binary(f, "REM", left, right, destination),
            Eq {
                left,
                right,
                destination,
            } => write_binary(f, "EQ", left, right, destination),
            Statement::NotEq {
                left,
                right,
                destination,
            } => write_binary(f, "NOTEQ", left, right, destination),
            Statement::Greater {
                left,
                right,
                destination,
            } => write_binary(f, "GREATER", left, right, destination),
            Statement::GreaterEq {
                left,
                right,
                destination,
            } => write_binary(f, "GREATEREQ", left, right, destination),
            Statement::Less {
                left,
                right,
                destination,
            } => write_binary(f, "LESS", left, right, destination),
            Statement::LessEq {
                left,
                right,
                destination,
            } => write_binary(f, "LESSEQ", left, right, destination),
            Statement::Jmp { location } => {
                write!(f, "JMP ")?;
                write_location(f, location)
            }
            Statement::JmpCmp { location, source } => {
                write!(f, "JMPCMP ")?;
                write_location(f, location)?;
                write!(f, " ")?;
                write_source(f, source)
            }
            Statement::JmpCmpNot { location, source } => {
                write!(f, "JMPCMPNOT ")?;
                write_location(f, location)?;
                write!(f, " ")?;
                write_source(f, source)
            }
            Statement::Call { routine, range } => write!(f, "CALL {} {:04x?}", routine, range),
            Statement::Ret => write!(f, "RET"),
        }
    }
}

fn display_pointer(f: &mut Formatter<'_>, pointer: &Pointer) -> Result {
    use Pointer::{Absolute, Const, Return, Stack, Static};
    match pointer {
        Absolute(ptr) => write!(f, "absolute@{:04x}", ptr),
        Static(ptr) => write!(f, "static@{:04x}", ptr),
        Const(ptr) => write!(f, "const@{:04x}", ptr),
        Stack(ptr) => write!(f, "stack@{:04x}", ptr),
        Return(ptr) => write!(f, "return@{:04x}", ptr),
    }
}

fn write_base_offset(
    f: &mut Formatter<'_>,
    base: &Pointer,
    offset: &Option<Box<Source<u8>>>,
) -> Result {
    match (base, offset) {
        (base, None) => {
            write!(f, "(")?;
            display_pointer(f, base)?;
            write!(f, ")")?;
        }
        (base, Some(offset)) => {
            write!(f, "(")?;
            display_pointer(f, base)?;
            write!(f, "+")?;
            write_source(f, offset)?;
            write!(f, ")")?;
        }
    }
    Ok(())
}

fn write_source<T: Display>(f: &mut Formatter<'_>, source: &Source<T>) -> Result {
    use Source::{Literal, Pointer, Register};
    match source {
        Pointer { base, offset } => write_base_offset(f, base, offset),
        Register(reg) => write!(f, "%{}", reg),
        Literal(lit) => write!(f, "#{}", lit),
    }
}

fn write_destination(f: &mut Formatter<'_>, destination: &Destination) -> Result {
    use Destination::{Pointer, Register};
    match destination {
        Pointer { base, offset } => write_base_offset(f, base, offset),
        Register(reg) => write!(f, "%{}", reg),
    }
}

fn write_location(f: &mut Formatter<'_>, location: &Location) -> Result {
    use Location::Relative;
    match location {
        Relative(rel) => write!(f, "{}", rel),
    }
}

fn write_binary<T: Display, S: Display>(
    f: &mut Formatter<'_>,
    mnemonic: &str,
    left: &Source<T>,
    right: &Source<S>,
    destination: &Destination,
) -> Result {
    write!(f, "{} ", mnemonic)?;
    write_destination(f, destination)?;
    write!(f, " ← ")?;
    write_source(f, left)?;
    write!(f, " ")?;
    write_source(f, right)?;
    Ok(())
}

fn write_unary<T: Display>(
    f: &mut Formatter<'_>,
    mnemonic: &str,
    source: &Source<T>,
    destination: &Destination,
) -> Result {
    write!(f, "{} ", mnemonic)?;
    write_destination(f, destination)?;
    write!(f, " ← ")?;
    write_source(f, source)?;
    Ok(())
}
