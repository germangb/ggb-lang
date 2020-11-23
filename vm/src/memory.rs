use crate::{Opts, Stack};
use std::ops::{Deref, DerefMut};

/// Memory space of static memory.
pub type StaticMemory = Box<[u8]>;

/// Memory space of function return values.
pub type ReturnMemory = Box<[u8]>;

/// Virtual Machine memory.
pub struct Memory {
    /// Stack memory space data.
    pub stack: StackMemory,

    /// Static memory space data.
    pub static_: StaticMemory,

    /// Return memory space data.
    pub return_: ReturnMemory,
}

impl Memory {
    pub(crate) fn new(opts: &Opts) -> Self {
        Self { stack: StackMemory::with_capacity(opts.stack_size),
               static_: vec![0; opts.static_size].into_boxed_slice(),
               return_: vec![0; opts.return_size].into_boxed_slice() }
    }
}

/// Stack memory space.
#[derive(Debug, Clone)]
pub struct StackMemory {
    stack_pointer: Stack<usize>,
    data: Vec<u8>,
}

impl StackMemory {
    /// Create an empty stack with the given capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self { stack_pointer: vec![0],
               data: vec![0; cap] }
    }

    /// Return the current stack pointer.
    pub fn stack_pointer(&self) -> usize {
        *self.stack_pointer.last().unwrap()
    }

    /// Push stack pointer by a given relative amount.
    pub fn push(&mut self, rel: usize) {
        let new_base = *self.stack_pointer.last().unwrap() + rel;
        self.stack_pointer.push(new_base);
    }

    /// Pop current stack pointer.
    pub fn pop(&mut self) {
        self.stack_pointer.pop().unwrap();
    }
}

impl Deref for StackMemory {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let sp = self.stack_pointer();
        &self.data[sp..]
    }
}

impl DerefMut for StackMemory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let sp = self.stack_pointer();
        &mut self.data[sp..]
    }
}
