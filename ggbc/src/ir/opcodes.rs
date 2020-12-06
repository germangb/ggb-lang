#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::ops::{Range, RangeFrom};

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
        #[serde(serialize_with = "ser_range_from")]
        #[serde(deserialize_with = "de_range_from")]
        range: RangeFrom<u16>,
    },

    /// Return from routine.
    Ret,
}

fn ser_range_from<S: Serializer>(range_from: &RangeFrom<u16>, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u16(range_from.start)
}

fn de_range_from<'de, D: Deserializer<'de>>(de: D) -> Result<RangeFrom<u16>, D::Error> {
    todo!()
}
