use std::{collections::hash_map::Entry, vec};

use fxhash::FxHashMap;
use itertools::Itertools;

use crate::{
    cache::{EnumFormulaCache, EnumFormulaCacheLine, FormulaCache},
    traits::Hashed,
};

use super::{charac::LtlCharac, hash::LtlHash, LtlFormula};

#[derive(Debug)]
pub(crate) struct LtlCache {
    hash_to_line: FxHashMap<LtlHash, (usize, usize)>,
    pub(crate) lines: Vec<Vec<LtlFormula>>,
}

impl LtlCache {
    pub(crate) fn new() -> Self {
        Self {
            hash_to_line: Default::default(),
            lines: vec![],
        }
    }
}

impl FormulaCache<LtlCharac> for LtlCache {
    fn len(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum()
    }

    fn get(&self, hash: &LtlHash) -> Option<&LtlFormula> {
        let &(line_id, index) = self.hash_to_line.get(hash)?;
        self.lines[line_id].get(index)
    }
}

impl EnumFormulaCache<LtlCharac> for LtlCache {
    type CacheLine<'a> = LtlCacheLine<'a>;

    fn new_line_and_iter_size<'a>(
        &'a mut self,
        new_size: usize,
    ) -> (
        impl Iterator<Item = &'a LtlFormula>,
        impl Iterator<Item = (&'a LtlFormula, &'a LtlFormula)>,
        Self::CacheLine<'a>,
    )
    where
        LtlCharac: 'a,
    {
        self.lines.push(vec![]);
        let (old_lines, new) = self.lines.split_at_mut(new_size);

        let new_line = LtlCacheLine {
            size_index: new_size,
            entries: &mut new[0],
            hashes: &mut self.hash_to_line,
        };

        let iter_size = new_size - 1;
        let iter_formulas = old_lines[iter_size].iter();

        let iter_pairs_size = (new_size + 1) / 2;
        let iter_pairs = old_lines
            .iter()
            .zip(old_lines.iter().rev())
            .take(iter_pairs_size)
            .flat_map(|(i1, i2)| i1.iter().cartesian_product(i2));
        (iter_formulas, iter_pairs, new_line)
    }

    fn iter_size<'a>(&'a self, size: usize) -> impl Iterator<Item = &'a LtlFormula>
    where
        LtlCharac: 'a,
    {
        self.lines[size].iter()
    }

    fn new_line<'a>(&'a mut self, size: usize) -> Self::CacheLine<'a>
    where
        LtlCharac: 'a,
    {
        self.lines.push(vec![]);
        let entries = &mut self.lines[size];

        LtlCacheLine {
            size_index: size,
            entries,
            hashes: &mut self.hash_to_line,
        }
    }

    fn nb_lines(&self) -> usize {
        self.lines.len()
    }
}

impl IntoIterator for LtlCache {
    type Item = LtlFormula;

    type IntoIter = Box<dyn Iterator<Item = LtlFormula>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.lines.into_iter().flatten())
    }
}

pub(crate) struct LtlCacheLine<'a> {
    size_index: usize,
    entries: &'a mut Vec<LtlFormula>,
    hashes: &'a mut FxHashMap<LtlHash, (usize, usize)>,
}

impl<'a> EnumFormulaCacheLine<LtlCharac> for LtlCacheLine<'a> {
    fn push(&mut self, f: LtlFormula) -> bool {
        assert_eq!(f.size, self.size_index);
        match self.hashes.entry(f.hashed()) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                let index = self.entries.len();
                e.insert((self.size_index, index));
                self.entries.push(f);
                true
            }
        }
    }
}
