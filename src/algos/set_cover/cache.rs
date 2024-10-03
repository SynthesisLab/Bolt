use std::collections::hash_map::{Entry, Values};

use fxhash::FxHashMap;

use crate::{
    bool::{charac::BoolCharac, BoolFormula},
    cache::FormulaCache,
    traits::Hashed,
};

pub struct ScCache {
    entries: FxHashMap<<BoolCharac as Hashed>::HashType, BoolFormula>,
}

impl ScCache {
    pub(crate) fn new() -> Self {
        Self {
            entries: Default::default(),
        }
    }

    pub(crate) fn push(&mut self, f: BoolFormula) -> bool {
        match self.entries.entry(f.hashed()) {
            Entry::Occupied(mut e) => {
                let sz = e.get().size;
                if sz > f.size {
                    e.insert(f);
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(e) => {
                e.insert(f);
                false
            }
        }
    }
}

impl FormulaCache<BoolCharac> for ScCache {
    fn len(&self) -> usize {
        self.entries.len()
    }

    fn get(&self, hash: &<BoolCharac as Hashed>::HashType) -> Option<&BoolFormula> {
        self.entries.get(hash)
    }
}

impl<'a> IntoIterator for &'a ScCache {
    type Item = &'a BoolFormula;

    type IntoIter = Values<'a, <BoolCharac as Hashed>::HashType, BoolFormula>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.values()
    }
}
