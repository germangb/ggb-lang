use std::ops::{Deref, DerefMut};

/// Storage of virtual registers.
#[derive(Debug, Clone)]
pub struct Registers<T> {
    store: Vec<T>,
}

impl<T: Copy + Default> Registers<T> {
    /// Create virtual register storage with the given capacity.
    /// Any register that exceed this capacity, will be considered spilled.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            store: vec![T::default(); cap],
        }
    }
}

impl<T: Copy> Registers<T> {
    /// Set register.
    pub fn set(&mut self, register: usize, value: T) {
        self.store[register] = value;
    }

    /// Get register.
    pub fn get(&self, register: usize) -> T {
        self.store[register]
    }
}

impl<T> Deref for Registers<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.store[..]
    }
}

impl<T> DerefMut for Registers<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store[..]
    }
}
