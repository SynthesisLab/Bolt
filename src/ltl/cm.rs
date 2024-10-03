//! Characteristic Matrix of LTL formulas.
use std::{
    hash::{Hash, Hasher},
    ops::{BitAnd, BitOr},
};

use crate::HashType;

use super::{cs::CharSeq, hash::LtlHash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharMatrix {
    pub(crate) seqs: Box<[CharSeq]>,
}

type CmHasher = fxhash::FxHasher64;

impl CharMatrix {
    pub(crate) fn hashed(&self) -> LtlHash {
        let mut h = CmHasher::default();
        self.hash(&mut h);
        LtlHash(h.finish() as HashType)
    }

    pub(crate) fn is_equivalent(&self, target: &[bool]) -> bool {
        self.seqs
            .iter()
            .zip(target)
            .all(|(cm, b)| cm.accepts() == *b)
    }
}

impl FromIterator<CharSeq> for CharMatrix {
    fn from_iter<T: IntoIterator<Item = CharSeq>>(iter: T) -> Self {
        Self {
            seqs: iter.into_iter().collect(),
        }
    }
}

/// Helper macro to implement LTL unary operators for CharMatrix
macro_rules! op_for_cm {
    ($( $f:ident ),*) => {
        $(
            pub(crate) fn $f(&self) -> Self {
                let seqs = self.seqs.iter().map(|c| c.$f()).collect();
                CharMatrix { seqs }
            }
        )*
    };
}

/// Helper macro to implement LTL binary operators for CharMatrix
macro_rules! binop_for_cm {
    ($f:ident) => {
        binop_for_cm!($f as $f);
    };
    ($f:ident as $g:ident) => {
        pub(crate) fn $g(&self, rhs: &Self) -> Self {
            let seqs = self
                .seqs
                .iter()
                .zip(rhs.seqs.iter())
                .map(|(c1, c2)| c1.$f(*c2))
                .collect();
            CharMatrix { seqs }
        }
    };
    () => {};
}

impl CharMatrix {
    pub fn accepted_vec(&self) -> Vec<bool> {
        self.seqs.iter().map(|x| x.accepts()).collect()
    }

    op_for_cm!(next, globally, finally);
    binop_for_cm!(bitor as or);
    binop_for_cm!(bitand as and);
    binop_for_cm!(until);
}
