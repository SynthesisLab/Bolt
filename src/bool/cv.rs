//! Characteristic vectors of boolean formulas.
use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    ops::{BitAnd, BitOr, BitXor, Not},
};

use crate::HashType;

use super::{hash::BoolHash, sv::SatVec};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
/// Represents the truth table of a formula over a set of inputs.
pub(crate) struct CharVec {
    pub(super) values: u128,
    pub(super) length: u8,
}

type CvHasher = fxhash::FxHasher64;

impl CharVec {
    pub(crate) fn len(&self) -> usize {
        self.length as usize
    }

    pub fn hashed(&self) -> BoolHash {
        let mut h = CvHasher::default();
        self.hash(&mut h);
        BoolHash(h.finish() as HashType)
    }

    /// Returns the characteristic vector of satisfied inputs
    pub(crate) fn satisfied(&self, target: CharVec) -> SatVec {
        assert_eq!(self.length, target.length);
        let values = (self.values & target.values) | (self.values | target.values).not();

        // Edge case: shifting 1u128 by 128 gives 1 in release mode, and panics in debug mode.
        let values = if self.length < 128 {
            values & ((1u128 << self.length) - 1)
        } else {
            values
        };
        SatVec { values }
    }

    #[inline]
    pub(crate) fn xor_satvec(&self, sv: SatVec) -> SatVec {
        let values = self.values.bitxor(sv.values);
        SatVec { values }
    }
}

impl Not for CharVec {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        let CharVec { values: x, length } = self;
        let values = x.not();
        // Edge case: shifting 1u128 by 128 gives 1 in release mode, and panics in debug mode.
        let values = if self.length < 128 {
            values & ((1u128 << self.length) - 1)
        } else {
            values
        };
        CharVec { values, length }
    }
}

impl BitOr for CharVec {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let CharVec { values: x, length } = self;
        let CharVec {
            values: y,
            length: _l2,
        } = rhs;
        CharVec {
            values: x.bitor(y),
            length,
        }
    }
}

impl BitAnd for CharVec {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let CharVec { values: x, length } = self;
        let CharVec {
            values: y,
            length: _l2,
        } = rhs;
        CharVec {
            values: x.bitand(y),
            length,
        }
    }
}

impl Debug for CharVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for CharVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.values;
        for i in 0..self.len() {
            write!(f, "{}", (x >> i) & 1)?;
        }
        Ok(())
    }
}

impl FromIterator<bool> for CharVec {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut x = 0;
        let mut length = 0;
        iter.into_iter().enumerate().for_each(|(i, b)| {
            if i > 127 {
                panic!("Iterator is too long! (max len {})", u128::BITS);
            }
            if b {
                x |= 1 << i;
            }
            length = (i + 1) as u8;
        });
        CharVec { values: x, length }
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    fn random_vec_with_len(len: usize, rng: &mut impl Rng) -> CharVec {
        let x: u128 = rng.gen();
        let x = if len < 128 {
            x & ((1u128 << len) - 1)
        } else {
            x
        };
        CharVec {
            values: x,
            length: len as u8,
        }
    }

    fn random_pair() -> (CharVec, CharVec) {
        let mut rng = thread_rng();
        let len = rng.gen_range(0..128);
        (
            random_vec_with_len(len, &mut rng),
            random_vec_with_len(len, &mut rng),
        )
    }

    fn random_vec() -> CharVec {
        let mut rng = thread_rng();
        let len = rng.gen_range(0..128);
        random_vec_with_len(len, &mut rng)
    }

    #[test]
    fn phi_and_not_phi_is_zero() {
        for _ in 0..100 {
            let x = random_vec();
            assert_eq!((x & !x).values, 0);
        }
    }

    #[test]
    fn not_is_involutive() {
        for _ in 0..100 {
            let x = random_vec();
            assert_eq!(x, !!x);
        }
    }

    #[test]
    fn and_is_idempotent() {
        for _ in 0..100 {
            let x = random_vec();
            assert_eq!(x & x, x);
        }
    }

    #[test]
    fn or_is_idempotent() {
        for _ in 0..100 {
            let x = random_vec();
            assert_eq!(x | x, x);
        }
    }

    #[test]
    /// Test "DeMorgan" identities for operators
    fn de_morgan_or_and() {
        for _ in 0..100 {
            let (x1, x2) = random_pair();
            assert_eq!(!(x1 | x2), !x1 & !x2);
        }
    }
}
