use std::ops::{Deref, DerefMut};

pub type Stack<T> = Vec<T>;

#[derive(Debug)]
pub struct StackFrame {
    data: Vec<u8>,
}

impl StackFrame {
    pub(crate) fn new() -> Self {
        Self { data: vec![0; 0x100] }
    }
}

impl Deref for StackFrame {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for StackFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
