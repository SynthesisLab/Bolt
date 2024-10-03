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

use crate::bool::{charac::BoolCharac, BoolFormula};

/// Keeps a hashmap for observational equivalence,
/// and only the `max_line_size` formulas with highest density of each size.
/// Computes domination over the line size.
#[derive(Debug)]
pub struct BeamSearchCache {
    entries: FxHashMap<<BoolCharac as Hashed>::HashType, BoolFormula>,
    lines: Vec<BinaryHeap<PcoBoolFormula>>,
    max_line_size: usize,
}

impl BeamSearchCache {
    pub(crate) fn new(max_line_size: usize) -> Self {
        Self {
            entries: Default::default(),
            lines: vec![],
            max_line_size,
        }
    }
}

impl FormulaCache<BoolCharac> for BeamSearchCache {
    fn len(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum()
    }

    fn get(&self, hash: &<BoolCharac as Hashed>::HashType) -> Option<&BoolFormula> {
        self.entries.get(hash)
    }
}

impl EnumFormulaCache<BoolCharac> for BeamSearchCache {
    type CacheLine<'a> = BeamSearchBoolCacheLine<'a>;

    fn iter_size<'a>(&'a self, size: usize) -> impl Iterator<Item = &'a BoolFormula>
    where
        BoolCharac: 'a,
    {
        self.lines[size].iter().map(|pf| &pf.f)
    }

    fn new_line_and_iter_size<'a>(
        &'a mut self,
        size: usize,
    ) -> (
        impl Iterator<Item = &'a BoolFormula>,
        impl Iterator<Item = (&'a BoolFormula, &'a BoolFormula)>,
        Self::CacheLine<'a>,
    )
    where
        BoolCharac: 'a,
    {
        self.lines.push(BinaryHeap::new());
        let (old_lines, new) = self.lines.split_at_mut(size);

        let new_line = BeamSearchBoolCacheLine {
            line: &mut new[0],
            hashes: &mut self.entries,
            max_line_size: self.max_line_size,
        };

        let iter_size = size - 1;
        let iter_formulas = old_lines[iter_size].iter().map(|pf| &pf.f);

        let iter_pairs_size = (size + 1) / 2;
        let iter_pairs = old_lines
            .iter()
            .zip(old_lines.iter().rev())
            .take(iter_pairs_size)
            .flat_map(|(i1, i2)| i1.iter().cartesian_product(i2))
            .map(|(pf1, pf2)| (&pf1.f, &pf2.f));
        (iter_formulas, iter_pairs, new_line)
    }

    fn new_line<'a>(&'a mut self, size: usize) -> Self::CacheLine<'a>
    where
        BoolCharac: 'a,
    {
        self.lines.push(BinaryHeap::new());

        BeamSearchBoolCacheLine {
            line: &mut self.lines[size],
            hashes: &mut self.entries,
            max_line_size: self.max_line_size,
        }
    }

    fn nb_lines(&self) -> usize {
        self.lines.len()
    }
}

impl IntoIterator for BeamSearchCache {
    type Item = BoolFormula;

    type IntoIter = Box<dyn Iterator<Item = BoolFormula>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.lines.into_iter().flatten().map(|pf| pf.f))
    }
}

pub(crate) struct BeamSearchBoolCacheLine<'a> {
    line: &'a mut BinaryHeap<PcoBoolFormula>,
    hashes: &'a mut FxHashMap<<BoolCharac as Hashed>::HashType, BoolFormula>,
    max_line_size: usize,
}

impl<'a> BeamSearchBoolCacheLine<'a> {
    fn dominates(&self, f: &BoolFormula) -> Option<<BoolCharac as Hashed>::HashType> {
        self.line.iter().find_map(|sv| {
            if sv.dominates(f) {
                Some(sv.f.hashed())
            } else {
                None
            }
        })
    }
}

impl<'a> EnumFormulaCacheLine<BoolCharac> for BeamSearchBoolCacheLine<'a> {
    fn push(&mut self, f: BoolFormula) -> bool {
        if self.dominates(&f).is_some() {
            return false;
        }

        let hash = f.hashed();
        let removed = match self.hashes.entry(hash) {
            Entry::Occupied(_) => return false,
            Entry::Vacant(e) => {
                e.insert(f.clone());
                self.line.push(PcoBoolFormula { f });
                if self.line.len() > self.max_line_size {
                    self.line.pop()
                } else {
                    return true;
                }
            }
        };

        self.hashes.remove(&removed.unwrap().f.hashed());

        true
    }
}

// Stores a SatVec together with the hash of the corresponding Boolean formula.
// Used when removing dominated formulas in a single  to canonicalize the entries at the end of the push round.
#[derive(Debug, PartialEq, Eq)]
struct PcoBoolFormula {
    pub(crate) f: BoolFormula,
}

impl PartialOrd for PcoBoolFormula {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Order by max popcount.
impl Ord for PcoBoolFormula {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f
            .charac
            .sv
            .popcount()
            .cmp(&self.f.charac.sv.popcount())
    }
}

impl PcoBoolFormula {
    pub(crate) fn dominates(&self, f: &BoolFormula) -> bool {
        self.f.charac.sv.dominates(f.charac.sv)
    }
}
