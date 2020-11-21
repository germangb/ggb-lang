/// Register storage.
#[derive(Debug, Clone)]
pub struct Registers<T> {
    store: Vec<Option<T>>,
}

impl<T: Copy + Default> Registers<T> {
    pub(crate) fn with_capacity(cap: usize) -> Self {
        Self { store: vec![None; cap] }
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
