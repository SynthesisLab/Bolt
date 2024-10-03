//! Result types for meta-algorithms
use std::time::Duration;

use crate::formula::tree::FormulaTree;

/// Result of a meta-algorithm,
/// with enumeration and running time data.
#[derive(Debug, Clone)]
pub struct MetaResult<D> {
    pub(crate) ltl_time: Duration,
    pub ltl_cache_sizes: Vec<usize>,
    pub(crate) algo_time: Option<Duration>,
    pub algo_data: Option<D>,
    pub(crate) result: MetaRes,
}

impl<D> MetaResult<D> {
    pub fn total_time_sec(&self) -> f64 {
        self.ltl_time.as_secs_f64() + self.algo_time.map_or(0., |d| d.as_secs_f64())
    }

    pub fn sol(&self) -> Option<FormulaTree> {
        self.result.get()
    }
}

/// Whether a result was found, and which part of
/// the algorithm found the result if there is one.
#[derive(Debug, Clone)]
pub enum MetaRes {
    NotFound,
    Atom(FormulaTree),
    FoundByLtl(FormulaTree),
    FoundByBool(FormulaTree),
}

impl MetaRes {
    pub fn get(&self) -> Option<FormulaTree> {
        match self {
            MetaRes::NotFound => None,
            MetaRes::Atom(f) => Some(f.clone()),
            MetaRes::FoundByLtl(f) => Some(f.clone()),
            MetaRes::FoundByBool(f) => Some(f.clone()),
        }
    }
}
