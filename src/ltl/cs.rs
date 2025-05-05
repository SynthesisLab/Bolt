//! Characteristic sequence of an LTL formula
//! and related operators.
use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitOr, BitXor, Not},
};

/// Characteristic sequence of an LTL formula on a trace.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharSeq {
    pub(crate) values: u64,
    length: usize,
}

impl Not for CharSeq {
    type Output = Self;

    fn not(self) -> Self::Output {
        let CharSeq { values: x, length } = self;
        let values = x.not();
        // Edge case: shifting 1u64 by 64 gives 1 in release mode, and panics in debug mode.
        let values = if self.length < 64 {
            values & ((1u64 << self.length) - 1)
        } else {
            values
        };
        CharSeq { values, length }
    }
}

impl BitOr for CharSeq {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let CharSeq { values: x, length } = self;
        let CharSeq {
            values: y,
            length: l2,
        } = rhs;
        assert_eq!(length, l2);
        CharSeq {
            values: x.bitor(y),
            length,
        }
    }
}

impl BitAnd for CharSeq {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let CharSeq { values: x, length } = self;
        let CharSeq {
            values: y,
            length: l2,
        } = rhs;
        assert_eq!(length, l2);
        CharSeq {
            values: x.bitand(y),
            length,
        }
    }
}

impl CharSeq {
    pub fn len(&self) -> usize {
        self.length
    }

    /// Whether the formula accepts the trace,
    /// i.e. it is true starting from the first position.
    #[inline]
    pub(crate) fn accepts(&self) -> bool {
        (self.values & 1) == 1
    }

    /// Boolean implication operator (->)
    #[inline]
    pub(crate) fn implies(self, rhs: Self) -> Self {
        let CharSeq { values: x, length } = self;
        let CharSeq {
            values: y,
            length: _l2,
        } = rhs;
        // x -> y <=> !x or y
        let values = x.not().bitor(y);
        let values = if self.length < 64 {
            values & ((1u64 << self.length) - 1)
        } else {
            values
        };
        CharSeq { values, length }
    }

    /// Boolean equivalence operator (<->)
    #[inline]
    pub(crate) fn equiv(self, rhs: Self) -> Self {
        let CharSeq { values: x, length } = self;
        let CharSeq {
            values: y,
            length: _l2,
        } = rhs;
        let values = (x.bitxor(y)).not();
        let values = if self.length < 64 {
            values & ((1u64 << self.length) - 1)
        } else {
            values
        };
        CharSeq { values, length }
    }

    /// LTL Next operator (X)
    #[inline]
    pub(crate) fn next(mut self) -> Self {
        self.values >>= 1;
        self
    }

    /// LTL Globally operator (G)
    #[inline]
    pub(crate) fn globally(self) -> Self {
        self.not().finally().not()
    }

    /// LTL Finally operator (F)
    #[inline]
    pub(crate) fn finally(self) -> Self {
        let CharSeq {
            values: mut x,
            length,
        } = self;
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        x |= x >> 32;
        CharSeq { values: x, length }
    }

    /// LTL Until operator (U)
    #[inline]
    pub(crate) fn until(self, rhs: Self) -> Self {
        let CharSeq {
            values: mut x,
            length,
        } = self;
        let CharSeq {
            values: mut y,
            length: _l2,
        } = rhs;
        y |= x & (y >> 1);
        x &= x >> 1;
        y |= x & (y >> 2);
        x &= x >> 2;
        y |= x & (y >> 4);
        x &= x >> 4;
        y |= x & (y >> 8);
        x &= x >> 8;
        y |= x & (y >> 16);
        x &= x >> 16;
        y |= x & (y >> 32);
        CharSeq { values: y, length }
    }
}

impl Debug for CharSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for CharSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.values;
        for i in 0..self.len() {
            write!(f, "{}", (x >> i) & 1)?;
        }
        Ok(())
    }
}

impl FromIterator<bool> for CharSeq {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut x: u64 = 0;
        let mut len = 0;
        iter.into_iter().enumerate().for_each(|(i, b)| {
            if i >= 64 {
                panic!("Trace is too long! (max len 64)");
            }
            if b {
                x |= 1 << i;
            }
            len += 1;
        });
        CharSeq {
            values: x,
            length: len,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, rng};

    use super::*;

    #[allow(non_snake_case)]
    pub(crate) fn X(phi: CharSeq) -> CharSeq {
        phi.next()
    }

    #[allow(non_snake_case)]
    pub(crate) fn G(phi: CharSeq) -> CharSeq {
        phi.globally()
    }

    #[allow(non_snake_case)]
    pub(crate) fn F(phi: CharSeq) -> CharSeq {
        phi.finally()
    }

    #[allow(non_snake_case)]
    pub(crate) fn U(phi: CharSeq, psi: CharSeq) -> CharSeq {
        phi.until(psi)
    }

    fn random_seq_with_len(len: usize, rng: &mut impl Rng) -> CharSeq {
        let x: u64 = rng.random();
        let x = if len < 64 { x & ((1u64 << len) - 1) } else { x };
        CharSeq {
            values: x,
            length: len,
        }
    }

    fn random_pair() -> (CharSeq, CharSeq) {
        let mut rng = rng();
        let len = rng.random_range(0..64);
        (
            random_seq_with_len(len, &mut rng),
            random_seq_with_len(len, &mut rng),
        )
    }

    fn random_seq() -> CharSeq {
        let mut rng = rng();
        let len = rng.random_range(0..64);
        random_seq_with_len(len, &mut rng)
    }

    #[test]
    fn phi_and_not_phi_is_zero() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!((x & !x).values, 0);
        }
    }

    #[test]
    fn phi_or_not_phi_is_true() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!((x | !x).values, (1 << x.length) - 1);
        }
    }

    #[test]
    fn not_is_involutive() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x, !!x);
        }
    }

    #[test]
    fn and_is_idempotent() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x & x, x);
        }
    }

    #[test]
    fn or_is_idempotent() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x | x, x);
        }
    }

    #[test]
    /// Test "DeMorgan" identities for LTL operators
    fn de_morgan_or_and() {
        for _ in 0..100 {
            let (x1, x2) = random_pair();
            assert_eq!(!(x1 | x2), !x1 & !x2);
        }
    }

    #[test]
    /// Test "DeMorgan" identities for LTL operators
    fn de_morgan_f_g() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(!F(x), G(!x));
        }
    }

    #[test]
    fn ff_is_f() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(F(F(x)), F(x));
        }
    }

    #[test]
    fn f_as_phi_or_x_f_phi() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(F(x), x | X(F(x)));
        }
    }

    #[test]
    fn and_distributes_g() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(G(x & y), G(x) & G(y));
        }
    }

    #[test]
    fn or_distributes_f() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(F(x | y), F(x) | F(y));
        }
    }

    #[test]
    fn gg_is_g() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(G(G(x)), G(x));
        }
    }

    #[test]
    fn expand_u() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(U(x, y), y | (x & X(U(x, y))));
        }
    }

    #[test]
    fn x_implies_x() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x.implies(x).values, (1 << x.length) - 1);
        }
    }

    #[test]
    fn implies_def() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(x.implies(y), x.not() | y);
        }
    }

    #[test]
    fn x_equiv_x() {
        for _ in 0..100 {
            let x = random_seq();
            assert_eq!(x.equiv(x).values, (1 << x.length) - 1);
        }
    }

    #[test]
    fn equiv_iff_double_implies() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(x.equiv(y), x.implies(y) & y.implies(x));
        }
    }

    #[test]
    fn equiv_def() {
        for _ in 0..100 {
            let (x, y) = random_pair();
            assert_eq!(x.equiv(y), (x & y) | (x.not() & y.not()));
        }
    }
}
