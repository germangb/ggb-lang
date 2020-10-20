/// Virtual register allocator.
#[derive(Default)]
pub struct RegisterAlloc {
    bitset: u64,
}

impl RegisterAlloc {
    /// Returns number of allocated registers.
    pub fn len(&self) -> u32 {
        self.bitset.count_ones()
    }

    /// Allocate register.
    pub fn alloc(&mut self) -> usize {
        let min = self.min();
        self.set(min, true);
        min
    }

    /// Free register being used.
    pub fn free(&mut self, index: usize) {
        assert!(self.get(index));
        self.set(index, false);
    }

    fn min(&self) -> usize {
        (0..64).find(|b| !self.get(*b)).unwrap()
    }

    fn get(&self, index: usize) -> bool {
        let bit = 1 << (index as u64);
        (self.bitset & bit) != 0
    }

    fn set(&mut self, index: usize, value: bool) -> bool {
        let bit = 1 << (index as u64);
        let old = (self.bitset | bit) != 0;
        if value {
            self.bitset |= bit;
        } else {
            self.bitset &= !bit;
        }
        old
    }
}

#[cfg(test)]
mod test {
    use super::RegisterAlloc;

    #[test]
    fn alloc() {
        let mut alloc = RegisterAlloc::default();

        alloc.alloc();
        alloc.alloc();
        alloc.alloc();
        alloc.alloc();
        assert_eq!(0b1111, alloc.bitset);
        alloc.set(1, false);
        assert_eq!(0b1101, alloc.bitset);
        alloc.alloc();
        assert_eq!(0b1111, alloc.bitset);
    }

    #[test]
    fn set() {
        let mut alloc = RegisterAlloc::default();
        alloc.set(0, true);
        alloc.set(2, true);
        alloc.set(4, true);
        alloc.set(6, true);
        assert!(alloc.get(0));
        assert!(!alloc.get(1));
        assert!(alloc.get(2));
        assert!(!alloc.get(3));
        assert!(alloc.get(4));
        assert!(!alloc.get(5));
        assert!(alloc.get(6));
        assert_eq!(0b1010101, alloc.bitset);
        alloc.set(2, false);
        alloc.set(4, false);
        alloc.set(6, false);
        assert_eq!(0b1, alloc.bitset);
        assert_eq!(1, alloc.min());
    }
}
