use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap},
};

use fxhash::FxHashMap;
use itertools::Itertools;

use crate::{
    cache::{EnumFormulaCache, EnumFormulaCacheLine, FormulaCache},
    traits::Hashed,
};

use super::{charac::BoolCharac, sv::SatVec, BoolFormula};

#[derive(Debug)]
pub struct BoolCache {
    hash_to_line: FxHashMap<<BoolCharac as Hashed>::HashType, (usize, usize)>,
    lines: Vec<Vec<BoolFormula>>,
    best_sv: Vec<BinaryHeap<SvHash>>,
    k: usize,
}

impl BoolCache {
    pub(crate) fn new(k: usize) -> Self {
        Self {
            hash_to_line: Default::default(),
            lines: vec![],
            best_sv: vec![],
            k,
        }
    }

    pub(crate) fn iter_lines(self) -> impl IntoIterator<Item = Vec<BoolFormula>> {
        self.lines
    }
}

impl FormulaCache<BoolCharac> for BoolCache {
    fn len(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum()
    }

    fn get(&self, hash: &<BoolCharac as Hashed>::HashType) -> Option<&BoolFormula> {
        let &(line_id, index) = self.hash_to_line.get(hash)?;
        self.lines[line_id].get(index)
    }
}

impl EnumFormulaCache<BoolCharac> for BoolCache {
    type CacheLine<'a> = BoolCacheLine<'a>;

    fn iter_size<'a>(&'a self, size: usize) -> impl Iterator<Item = &'a BoolFormula>
    where
        BoolCharac: 'a,
    {
        self.lines[size].iter()
    }

    fn new_line_and_iter_size<'a>(
        &'a mut self,
        size: usize,
    ) -> (
        impl Iterator<Item = &'a crate::formula::Formula<BoolCharac>>,
        impl Iterator<
            Item = (
                &'a crate::formula::Formula<BoolCharac>,
                &'a crate::formula::Formula<BoolCharac>,
            ),
        >,
        Self::CacheLine<'a>,
    )
    where
        BoolCharac: 'a,
    {
        self.lines.push(vec![]);
        let (old_lines, new) = self.lines.split_at_mut(size);
        self.best_sv.push(BinaryHeap::new());
        let (old_heaps, new_heap) = self.best_sv.split_at_mut(size);

        let new_line = BoolCacheLine {
            size_index: size,
            entries: &mut new[0],
            hashes: &mut self.hash_to_line,
            best_sv: &mut new_heap[0],
            other_heaps: old_heaps,
            k: self.k,
        };

        let iter_size = size - 1;
        let iter_formulas = old_lines[iter_size].iter();

        let iter_pairs_size = (size + 1) / 2;
        let iter_pairs = old_lines
            .iter()
            .zip(old_lines.iter().rev())
            .take(iter_pairs_size)
            .flat_map(|(i1, i2)| i1.iter().cartesian_product(i2));
        (iter_formulas, iter_pairs, new_line)
    }

    fn new_line<'a>(&'a mut self, size: usize) -> Self::CacheLine<'a>
    where
        BoolCharac: 'a,
    {
        self.lines.push(vec![]);
        self.best_sv.push(BinaryHeap::new());
        let (old_heaps, new_heap) = self.best_sv.split_at_mut(size);

        BoolCacheLine {
            size_index: size,
            entries: &mut self.lines[size],
            hashes: &mut self.hash_to_line,
            best_sv: &mut new_heap[0],
            other_heaps: old_heaps,
            k: self.k,
        }
    }

    fn nb_lines(&self) -> usize {
        self.lines.len()
    }
}

impl IntoIterator for BoolCache {
    type Item = BoolFormula;

    type IntoIter = Box<dyn Iterator<Item = BoolFormula>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.lines.into_iter().flatten())
    }
}

pub(crate) struct BoolCacheLine<'a> {
    entries: &'a mut Vec<BoolFormula>,
    best_sv: &'a mut BinaryHeap<SvHash>,
    hashes: &'a mut FxHashMap<<BoolCharac as Hashed>::HashType, (usize, usize)>,
    other_heaps: &'a [BinaryHeap<SvHash>],
    k: usize,
    size_index: usize,
}

impl<'a> BoolCacheLine<'a> {
    fn dominates(&self, f: &BoolFormula) -> Option<<BoolCharac as Hashed>::HashType> {
        // Iterate in reverse order to start with the densest formulas,
        // which are more likely to dominate.
        if let Some(r) =
            self.best_sv
                .iter()
                .rev()
                .find_map(|sv| if sv.dominates(f) { Some(sv.hash) } else { None })
        {
            Some(r)
        } else {
            self.other_heaps.iter().find_map(|h| {
                h.iter()
                    .rev()
                    .find_map(|sv| if sv.dominates(f) { Some(sv.hash) } else { None })
            })
        }
    }
}

impl<'a> EnumFormulaCacheLine<BoolCharac> for BoolCacheLine<'a> {
    fn push(&mut self, f: BoolFormula) -> bool {
        assert_eq!(f.size, self.size_index);
        if self.dominates(&f).is_some() {
            return false;
        }

        let hash = f.hashed();
        match self.hashes.entry(hash) {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) => {
                let index = self.entries.len();
                e.insert((self.size_index, index));
                self.best_sv.push(SvHash {
                    sv: f.charac.sv,
                    hash,
                });
                if self.best_sv.len() > self.k {
                    self.best_sv.pop();
                }
                self.entries.push(f);
                true
            }
        }
    }
}

/// Stores a SatVec together with the hash of the corresponding Boolean formula.
/// Used when removing dominated formulas in a single  to canonicalize the entries at the end of the push round.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SvHash {
    sv: SatVec,
    hash: <BoolCharac as Hashed>::HashType,
}

impl PartialOrd for SvHash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Order by max popcount.
impl Ord for SvHash {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sv.popcount().cmp(&self.sv.popcount())
    }
}

impl SvHash {
    pub(crate) fn dominates(&self, f: &BoolFormula) -> bool {
        self.sv.dominates(f.charac.sv)
    }
}

#[cfg(test)]
mod test {
    use crate::bool::hash::BoolHash;

    use super::*;

    #[test]
    fn sv_hash_ordering_in_heap() {
        let mut h = BinaryHeap::new();
        h.push(SvHash {
            sv: SatVec { values: 42 },
            hash: BoolHash(0),
        });

        h.push(SvHash {
            sv: SatVec { values: 1 },
            hash: BoolHash(0),
        });

        h.push(SvHash {
            sv: SatVec { values: 7 },
            hash: BoolHash(0),
        });

        // Ensure that we get min popcount first
        assert_eq!(h.pop().unwrap().sv.values, 1);
    }
}
