const MAX_REGISTERS: usize = 100;

/// Register storage.
pub struct Registers<T> {
    store: Vec<Option<T>>,
}

impl<T: Copy + Default> Registers<T> {
    pub(crate) fn new() -> Self {
        Self { store: vec![None; MAX_REGISTERS] }
    }

    /// Set register.
    pub fn set(&mut self, register: usize, value: T) {
        self.store[register] = Some(value);
    }

    /// Get register.
    pub fn get(&self, register: usize) -> T {
        self.store[register].unwrap()
    }

    /// Iterator over the used registers.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, T)> + 'a {
        self.store
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(idx, item)| item.map(|value| (idx, value)))
    }
}
