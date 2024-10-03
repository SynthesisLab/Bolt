//! Boolean cache used after LTL search
//! given as input to the boolean algorithms.
use std::{
    collections::BinaryHeap,
    hash::{Hash, Hasher},
    iter::Flatten,
    rc::Rc,
};

use fxhash::{FxHashMap, FxHasher};
use itertools::Itertools;
use log::debug;

use crate::{
    cache::EnumFormulaCache,
    formula::{tree::FormulaTree, FormulaNode},
    ltl::{cache::LtlCache, hash::LtlHash, LtlFormula},
    traits::Hashed,
};

type LsvHash = u64;
/// Contains a Characteristic vector in [`Vec`] form,
/// a pointer to the corresponding [`FormulaTree`]
/// and the size of the formula.
type BoolInfo = (Vec<bool>, Rc<FormulaTree>, usize);

/// Cache for boolean formulas with equivalence and domination test.
#[derive(Debug, Clone)]
pub struct InitialBoolCache {
    /// Hashmap of all the hashes of the formulas contained in the cache,
    /// mapping to the corresponding [`FormulaTree`].
    hash_cache: FxHashMap<LsvHash, Rc<FormulaTree>>,
    lines: Vec<Vec<BoolInfo>>,
    /// Set of formulas kept for domination tests.
    ///
    /// `best_sv[i]` is [`BinaryHeap`] that contains the `self.k` densest
    /// formulas of size `i`.
    ///
    /// By default, a [`BinaryHeap`] is a max-heap.
    /// As the ordering over [`LongSv`] is implemented in reverse,
    /// this gives a min-heap.
    /// Therefore, popping from the heap yields the formula with the lowest count of ones first,
    /// and we keep the `k` densest.
    best_sv: Vec<BinaryHeap<LongSv>>,
    /// Number of formulas of each size to keep for domination tests.
    ///
    /// For each size, the data structure keeps the `k` formulas with the highest number of satisfied inputs
    /// and only tests domination against these.
    k: usize,
}

impl InitialBoolCache {
    pub fn len(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum()
    }

    pub fn iter_all(&self) -> impl IntoIterator<Item = &BoolInfo> {
        self.lines.iter().flat_map(|l| l.iter())
    }

    pub fn iter_lines(self) -> impl IntoIterator<Item = Vec<BoolInfo>> {
        self.lines
    }

    /// Retrieve the formula with the given characteristic vector, if it is present in the cache.
    pub fn get_from_cv(&self, cv: &[bool], target: &[bool]) -> Option<FormulaTree> {
        let lsv = LongSv::from_cv_target(cv, target, 0);
        self.hash_cache
            .get(&lsv.hash)
            .map(|rc| Rc::unwrap_or_clone(rc.clone()))
    }

    /// Test whether the cache contains a formula equivalent to or
    /// dominating the input formula.
    fn is_redundant(&self, lsv: &LongSv) -> bool {
        // Equivalence test
        if self.hash_cache.contains_key(&lsv.hash) {
            return true;
        }

        // Domination test
        self.best_sv[..lsv.size]
            .iter()
            .any(|h| h.iter().rev().any(|lsv2| lsv2.dominates(lsv)))
    }

    /// Add a formula to the cache.
    ///
    /// The parameter `cv` is the characteristic vector of the formula
    /// given in `f_tree`, and `size` is its size.
    pub fn push(
        &mut self,
        cv: Vec<bool>,
        target: &[bool],
        f_tree: Rc<FormulaTree>,
        size: usize,
    ) -> bool {
        let lsv = LongSv::from_cv_target(&cv, target, size);

        if self.is_redundant(&lsv) {
            return false;
        }

        self.hash_cache.insert(lsv.hash, f_tree.clone());
        self.lines[size].push((cv, f_tree, size));
        self.best_sv[size].push(lsv);
        if self.best_sv[size].len() > self.k {
            self.best_sv[size].pop();
        }

        true
    }

    pub(crate) fn from_ltl_cache(k: usize, ltl_cache: LtlCache, target: &[bool]) -> Self {
        let mut rc_cache: FxHashMap<LtlHash, Rc<FormulaTree>> = FxHashMap::default();

        let mut res = Self {
            hash_cache: Default::default(),
            lines: vec![vec![]; ltl_cache.nb_lines()],
            best_sv: vec![Default::default(); ltl_cache.nb_lines()],
            k,
        };

        let mut count = 0;
        let mut hits = 0;
        for l in ltl_cache.lines {
            for f in l {
                let f_tree = rebuild_formula_rc(&f, &rc_cache);
                let cv = f.accepted_vec();
                let hash = f.hashed();
                let size = f.size;
                rc_cache.insert(hash, f_tree.clone());
                if res.push(cv, target, f_tree, size) {
                    count += 1;
                } else {
                    hits += 1;
                }
            }
        }

        // let (lines, best_sv) = (res.lines, res.best_sv);

        // res.lines = lines
        //     .into_iter()
        //     .map(|l| {
        //         l.into_iter()
        //             .filter(|(_, lsv, _)| {
        //                 best_sv[..lsv.size - 1]
        //                     .iter()
        //                     .flatten()
        //                     .any(|lsv2| lsv2.dominates(lsv))
        //             })
        //             .collect_vec()
        //     })
        //     .collect();

        // res.best_sv = best_sv;

        debug!("Creating Initial Cache: {count} formulas, {hits} cache hits");

        res
    }

    /// Convert into a cache of a single search subproblem.
    pub fn reduce(&self, indices: &[usize], target: &[bool]) -> Self {
        let target = indices.iter().map(|&i| target[i]).collect_vec();

        let nb_lines = self.lines.len();
        let mut cache = Self {
            hash_cache: Default::default(),
            lines: vec![vec![]; nb_lines],
            best_sv: vec![Default::default(); nb_lines],
            k: self.k,
        };

        for l in &self.lines {
            for (cv, t, size) in l {
                let cv = indices.iter().map(|&i| cv[i]).collect_vec();
                cache.push(cv, &target, t.clone(), *size);
            }
        }

        cache
    }

    /// Convert into caches for two search subproblems.
    pub fn split(self, left: &[usize], right: &[usize], target: &[bool]) -> (Self, Self) {
        let target_left = left.iter().map(|&i| target[i]).collect_vec();
        let target_right = right.iter().map(|&i| target[i]).collect_vec();

        let nb_lines = self.lines.len();
        let mut left_cache = Self {
            hash_cache: Default::default(),
            lines: vec![vec![]; nb_lines],
            best_sv: vec![Default::default(); nb_lines],
            k: self.k,
        };
        let mut right_cache = Self {
            hash_cache: Default::default(),
            lines: vec![vec![]; nb_lines],
            best_sv: vec![Default::default(); nb_lines],
            k: self.k,
        };

        for l in self.lines {
            for (cv, t, size) in l {
                let cv_left = left.iter().map(|&i| cv[i]).collect_vec();
                left_cache.push(cv_left, &target_left, t.clone(), size);
                let cv_right = right.iter().map(|&i| cv[i]).collect_vec();
                right_cache.push(cv_right, &target_right, t.clone(), size);
            }
        }

        (left_cache, right_cache)
    }
}

impl IntoIterator for InitialBoolCache {
    type Item = BoolInfo;

    type IntoIter = Flatten<std::vec::IntoIter<Vec<(Vec<bool>, Rc<FormulaTree>, usize)>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter().flatten()
    }
}

/// Convert an LTL formula from implicit representation ([`LtlFormula`])
/// to explicit tree representation ([`FormulaTree`]).
fn rebuild_formula_rc(
    f: &LtlFormula,
    rc_cache: &FxHashMap<LtlHash, Rc<FormulaTree>>,
) -> Rc<FormulaTree> {
    match &f.node {
        FormulaNode::Base(t) => t.clone(),
        &FormulaNode::Unary { op, child } => Rc::new(FormulaTree::UnaryNode {
            op,
            child: rc_cache.get(&child).expect("Child not found").clone(),
        }),
        &FormulaNode::Binary { op, left, right } => Rc::new(FormulaTree::BinaryNode {
            op,
            left: rc_cache.get(&left).expect("Left not found").clone(),

            right: rc_cache.get(&right).expect("Right not found").clone(),
        }),
    }
}

/// Satisfiability Vectors without length bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LongSv {
    /// Number of ones in `self.sv`, used for ordering.
    popcount: usize,
    /// Satisfiability vector
    sv: BitVec,
    /// Size of the corresponding formula
    size: usize,
    /// Hash of `sv`
    hash: LsvHash,
}

impl PartialOrd for LongSv {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LongSv {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.popcount.cmp(&self.popcount)
    }
}

impl Hashed for LongSv {
    type HashType = LsvHash;

    fn hashed(&self) -> Self::HashType {
        self.hash
    }
}

impl LongSv {
    pub fn from_cv_target(cv: &[bool], target: &[bool], size: usize) -> Self {
        let sv: BitVec = cv
            .iter()
            .zip(target.iter())
            .map(|(&b, &t)| b == t)
            .collect();
        let popcount = sv.count_ones();

        let mut h = FxHasher::default();
        sv.hash(&mut h);
        let hash = h.finish();

        Self {
            popcount,
            sv,
            size,
            hash,
        }
    }

    pub fn dominates(&self, rhs: &LongSv) -> bool {
        self.sv.dominates(&rhs.sv)
    }
}

/// Arbitrary length bit vector,
/// used to represent satisfiability vectors.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Default)]
struct BitVec {
    inner: Vec<u64>,
}

impl BitVec {
    pub fn count_ones(&self) -> usize {
        self.inner
            .iter()
            .map(|x| x.count_ones() as usize)
            .sum::<usize>()
    }

    /// Test whether `self` dominates `rhs`.
    pub fn dominates(&self, rhs: &Self) -> bool {
        assert_eq!(self.inner.len(), rhs.inner.len());
        self.inner
            .iter()
            .zip(rhs.inner.iter())
            .all(|(&a, &b)| (!a & b) == 0)
    }
}

impl FromIterator<bool> for BitVec {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut inner = vec![];
        let mut len = 0;
        for b in iter {
            if len == 0 {
                inner.push(0);
            }

            if b {
                let x = inner.last_mut().unwrap();
                *x |= 1u64 << len;
            }

            len += 1;
            len &= 63;
        }

        Self { inner }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn long_sv_hash_ordering_in_heap() {
        let mut h = BinaryHeap::new();
        h.push(LongSv {
            popcount: 1,
            sv: BitVec::default(),
            size: 0,
            hash: 0,
        });
        h.push(LongSv {
            popcount: 3,
            sv: BitVec::default(),
            size: 0,
            hash: 0,
        });
        h.push(LongSv {
            popcount: 4,
            sv: BitVec::default(),
            size: 0,
            hash: 0,
        });

        // Ensure that we get min popcount first
        assert_eq!(h.pop().unwrap().popcount, 1);
    }
}
