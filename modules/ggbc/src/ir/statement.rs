#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

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
}

impl Pointer {
    pub fn offset(self, offset: Address) -> Self {
        use Pointer::{Absolute, Const, Stack, Static};
        match self {
            Absolute(a) => Absolute(a + offset),
            Static(a) => Static(a + offset),
            Const(a) => Const(a + offset),
            Stack(a) => Stack(a + offset),
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
        offset: Option<Box<Source<Address>>>,
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
        offset: Option<Box<Source<Address>>>,
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

/// The statements, or instruction set of the IR.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Statement {
    /// Placeholder instructions.
    Nop(usize),
    /// Statement to stop execution (end program)
    Stop,

    // move/load instructions
    Ld {
        source: Source<u8>,
        destination: Destination,
    },
    LdW {
        source: Source<u16>,
        destination: Destination,
    },
    // move/load of memory addresses
    LdAddr {
        source: Source<Address>,
        destination: Destination,
    },

    // arithmetic (unary)
    Inc {
        source: Source<u8>,
        destination: Destination,
    },
    Dec {
        source: Source<u8>,
        destination: Destination,
    },
    IncW {
        source: Source<u16>,
        destination: Destination,
    },
    DecW {
        source: Source<u16>,
        destination: Destination,
    },

    // arithmetic (binary)
    Add {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    Sub {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    And {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    Xor {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    Or {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    LeftShift {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    RightShift {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    AddW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },
    SubW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },
    AndW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },
    XorW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },
    OrW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },
    LeftShiftW {
        left: Source<u16>,
        right: Source<u8>,
        destination: Destination,
    },
    RightShiftW {
        left: Source<u16>,
        right: Source<u8>,
        destination: Destination,
    },

    // The following instructions may not be available in target architectures through a native
    // instruction. In those cases, codegen might decompose to the corresponding equivalent
    // bitwise operations.
    Mul {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    }, // multiply
    Div {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    MulW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    }, // multiply
    DivW {
        left: Source<u16>,
        right: Source<u16>,
        destination: Destination,
    },
    Rem {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    // Compare
    // ==, !=
    Eq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    NotEq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    // >, >=
    Greater {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    GreaterEq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    // <, <=
    Less {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },
    LessEq {
        left: Source<u8>,
        right: Source<u8>,
        destination: Destination,
    },

    // flow control
    Jmp {
        /// Jump location.
        location: Location,
    },
    /// jump to target if source == 0
    JmpCmp {
        /// Jump location
        location: Location,
        source: Source<u8>,
    },
    /// jump to target if source != 0
    JmpCmpNot {
        /// Jump location.
        location: Location,
        source: Source<u8>,
    },

    // routines
    Call {
        /// Routine index in `Ir::routines`
        routine: usize,
        /// Address of the beginning of the function stack frame.
        address: Address,
    },
    // TODO return value
    Ret,
}

impl Statement {
    /// Return displayable mnemonic.
    pub fn display(&self) -> MnemonicDisplay<'_> {
        MnemonicDisplay { statement: self }
    }
}

/// Displayable statement mnemonic.
#[derive(Debug)]
pub struct MnemonicDisplay<'a> {
    statement: &'a Statement,
}

impl fmt::Display for MnemonicDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Statement::{
            Add, AddW, And, AndW, Call, Dec, DecW, Div, DivW, Eq, Greater, GreaterEq, Inc, IncW,
            Jmp, JmpCmp, JmpCmpNot, Ld, LdAddr, LdW, LeftShift, LeftShiftW, Less, LessEq, Mul,
            MulW, Nop, NotEq, Or, OrW, Rem, Ret, RightShift, RightShiftW, Stop, Sub, SubW, Xor,
            XorW,
        };

        match self.statement {
            Nop(i) => write!(f, "NOP {}", i),
            Stop => write!(f, "STOP"),
            Ld { source,
                 destination, } => write_unary(f, "LD", source, destination),
            LdW { source,
                  destination, } => write_unary(f, "LDW", source, destination),
            LdAddr { .. } => unimplemented!(),
            Inc { source,
                  destination, } => write_unary(f, "INC", source, destination),
            Dec { source,
                  destination, } => write_unary(f, "DEC", source, destination),
            IncW { source,
                   destination, } => write_unary(f, "INCW", source, destination),
            DecW { source,
                   destination, } => write_unary(f, "DECW", source, destination),
            Add { left,
                  right,
                  destination, } => write_binary(f, "ADD", left, right, destination),
            Sub { left,
                  right,
                  destination, } => write_binary(f, "SUB", left, right, destination),
            And { left,
                  right,
                  destination, } => write_binary(f, "ADD", left, right, destination),
            Xor { left,
                  right,
                  destination, } => write_binary(f, "XOR", left, right, destination),
            Or { left,
                 right,
                 destination, } => write_binary(f, "OR", left, right, destination),
            LeftShift { left,
                        right,
                        destination, } => write_binary(f, "LEFTSHIFT", left, right, destination),
            RightShift { left,
                         right,
                         destination, } => write_binary(f, "RIGHTSHIFT", left, right, destination),
            AddW { left,
                   right,
                   destination, } => write_binary(f, "ADDW", left, right, destination),
            SubW { left,
                   right,
                   destination, } => write_binary(f, "SUBW", left, right, destination),
            AndW { left,
                   right,
                   destination, } => write_binary(f, "ANDW", left, right, destination),
            XorW { left,
                   right,
                   destination, } => write_binary(f, "XORW", left, right, destination),
            OrW { left,
                  right,
                  destination, } => write_binary(f, "ORW", left, right, destination),
            LeftShiftW { left,
                         right,
                         destination, } => write_binary(f, "LEFTSHIFTW", left, right, destination),
            RightShiftW { left,
                          right,
                          destination, } => {
                write_binary(f, "RIGHTSHIFTW", left, right, destination)
            }
            Mul { left,
                  right,
                  destination, } => write_binary(f, "MUL", left, right, destination),
            Div { left,
                  right,
                  destination, } => write_binary(f, "DIV", left, right, destination),
            MulW { left,
                   right,
                   destination, } => write_binary(f, "MULW", left, right, destination),
            DivW { left,
                   right,
                   destination, } => write_binary(f, "DIVW", left, right, destination),
            Rem { left,
                  right,
                  destination, } => write_binary(f, "REM", left, right, destination),
            Eq { left,
                 right,
                 destination, } => write_binary(f, "EQ", left, right, destination),
            NotEq { left,
                    right,
                    destination, } => write_binary(f, "NOTEQ", left, right, destination),
            Greater { left,
                      right,
                      destination, } => write_binary(f, "GREATER", left, right, destination),
            GreaterEq { left,
                        right,
                        destination, } => write_binary(f, "GREATEREQ", left, right, destination),
            Less { left,
                   right,
                   destination, } => write_binary(f, "LESS", left, right, destination),
            LessEq { left,
                     right,
                     destination, } => write_binary(f, "LESSEQ", left, right, destination),
            Jmp { location } => {
                write!(f, "JMP ")?;
                write_location(f, location)
            }
            JmpCmp { location, source } => {
                write!(f, "JMPCMP ")?;
                write_location(f, location)?;
                write_source(f, source)
            }
            JmpCmpNot { location, source } => {
                write!(f, "JMPCMPNOT ")?;
                write_location(f, location)?;
                write!(f, " ")?;
                write_source(f, source)
            }
            Call { .. } => write!(f, "TODO"),
            Ret => write!(f, "RET"),
        }
    }
}

fn display_pointer(f: &mut fmt::Formatter<'_>, pointer: &Pointer) -> fmt::Result {
    use Pointer::{Absolute, Const, Stack, Static};
    match pointer {
        Absolute(ptr) => write!(f, "abs@{:04x}", ptr),
        Static(ptr) => write!(f, "static@{:04x}", ptr),
        Const(ptr) => write!(f, "const@{:04x}", ptr),
        Stack(ptr) => write!(f, "stack@{:04x}", ptr),
    }
}

fn write_base_offset(f: &mut fmt::Formatter<'_>,
                     base: &Pointer,
                     offset: &Option<Box<Source<u16>>>)
                     -> fmt::Result {
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

fn write_source<T: fmt::Display>(f: &mut fmt::Formatter<'_>, source: &Source<T>) -> fmt::Result {
    use Source::{Literal, Pointer, Register};
    match source {
        Pointer { base, offset } => write_base_offset(f, base, offset),
        Register(reg) => write!(f, "%{}", reg),
        Literal(lit) => write!(f, "#{}", lit),
    }
}

fn write_destination(f: &mut fmt::Formatter<'_>, destination: &Destination) -> fmt::Result {
    use Destination::{Pointer, Register};
    match destination {
        Pointer { base, offset } => write_base_offset(f, base, offset),
        Register(reg) => write!(f, "%{}", reg),
    }
}

fn write_location(f: &mut fmt::Formatter<'_>, location: &Location) -> fmt::Result {
    use Location::Relative;
    match location {
        Relative(rel) => write!(f, "{}", rel),
    }
}

fn write_binary<T: fmt::Display, S: fmt::Display>(f: &mut fmt::Formatter<'_>,
                                                  mnemonic: &str,
                                                  left: &Source<T>,
                                                  right: &Source<S>,
                                                  destination: &Destination)
                                                  -> fmt::Result {
    write!(f, "{} ", mnemonic)?;
    write_destination(f, destination)?;
    write!(f, " ← ")?;
    write_source(f, left)?;
    write!(f, " ")?;
    write_source(f, right)?;
    Ok(())
}

fn write_unary<T: fmt::Display>(f: &mut fmt::Formatter<'_>,
                                mnemonic: &str,
                                source: &Source<T>,
                                destination: &Destination)
                                -> fmt::Result {
    write!(f, "{} ", mnemonic)?;
    write_destination(f, destination)?;
    write!(f, " ← ")?;
    write_source(f, source)?;
    Ok(())
}
