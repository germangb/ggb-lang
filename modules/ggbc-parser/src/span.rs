//! Location within input code.

use std::ops::Deref;

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        self.deref().span()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Span {
    /// Position of the left-most char.
    pub min: [usize; 2],
    /// Position of the right-most char.
    pub max: [usize; 2],
}

pub fn union(l: &Span, r: &Span) -> Span {
    let mut min = l.min;
    let mut max = l.max;
    if r.min[0] < min[0] || r.min[0] == min[0] && r.min[1] < min[1] {
        min = r.min;
    }
    if r.max[0] > max[0] || r.max[0] == max[0] && r.max[1] > max[1] {
        max = r.max;
    }
    Span { min, max }
}

#[cfg(test)]
mod test {
    use crate::span::{union, Span};

    #[test]
    fn same() {
        let l = Span { min: [0, 0],
                       max: [42, 42] };
        let r = l;
        let gt = Span { min: [0, 0],
                        max: [42, 42] };

        assert_eq!(gt, union(&l, &r));
        assert_eq!(gt, union(&r, &l));
    }

    #[test]
    fn disjoint() {
        let l = Span { min: [0, 0],
                       max: [42, 42] };
        let r = Span { min: [43, 0],
                       max: [84, 84] };
        let gt = Span { min: [0, 0],
                        max: [84, 84] };

        assert_eq!(gt, union(&l, &r));
        assert_eq!(gt, union(&r, &l));
    }

    #[test]
    fn inner() {
        let l = Span { min: [0, 0],
                       max: [42, 42] };
        let r = Span { min: [12, 0],
                       max: [24, 24] };
        let gt = Span { min: [0, 0],
                        max: [42, 42] };

        assert_eq!(gt, union(&l, &r));
        assert_eq!(gt, union(&r, &l));
    }

    #[test]
    fn linked() {
        let l = Span { min: [0, 0],
                       max: [24, 42] };
        let r = Span { min: [24, 24],
                       max: [42, 42] };
        let gt = Span { min: [0, 0],
                        max: [42, 42] };

        assert_eq!(gt, union(&l, &r));
        assert_eq!(gt, union(&r, &l));
    }
}
