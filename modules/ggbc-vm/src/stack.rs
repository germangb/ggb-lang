use std::ops::{Deref, DerefMut};

/// Stack data structure.
pub type Stack<T> = Vec<T>;

/// Stack memory space.
#[derive(Debug, Clone)]
pub struct StackMemory {
    /// start of the current stack frame
    base: Stack<usize>,

    /// Stack memory data.
    data: Vec<u8>,
}

impl StackMemory {
    /// Create an empty stack.
    pub(crate) fn with_capacity(cap: usize) -> Self {
        Self { base: vec![0],
               data: vec![0; cap] }
    }

    /// Push stack pointer by a given relative amount.
    pub(crate) fn push(&mut self, rel: usize) {
        let new_base = *self.base.last().unwrap() + rel;
        self.base.push(new_base);
    }

    /// Pop current stack pointer.
    pub(crate) fn pop(&mut self) {
        self.base.pop().unwrap();
    }
}

impl Deref for StackMemory {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let base = *self.base.last().unwrap();
        &self.data[base..]
    }
}

impl DerefMut for StackMemory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let base = *self.base.last().unwrap();
        &mut self.data[base..]
    }
}

#[cfg(test)]
mod tests {}
