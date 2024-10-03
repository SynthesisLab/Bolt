//! Beam Search algorithm for Boolean Synthesis.
//!
//! Bottom-up enumeration with fixed width.
//! Implemented using a fixed-width cache ([`BeamSearchCache`])
//! and the enumeration algorithm.
use cache::BeamSearchCache;
use clap::Args;

use crate::{
    algos::enumeration::aux::enum_aux,
    bool::{charac::BoolCharac, cv::CharVec, BoolFormula},
    cache::{EnumFormulaCache, EnumFormulaCacheLine},
    formula::{rebuild_formula, tree::FormulaTree},
    ltl::trace::Operators,
};

pub mod cache;

use super::{meta::cache::InitialBoolCache, BoolAlgoParams};

#[derive(Args, Clone, Copy)]
pub struct BeamSearchParams {
    /// Number of formulas to keep at each level.
    beam_width: usize,
    /// Maximum enumeration size
    max_size_bool: usize,
}

impl BoolAlgoParams for BeamSearchParams {
    type Data = ();

    fn run(
        &self,
        cache: InitialBoolCache,
        operators: Operators,
        target: &[bool],
    ) -> (Option<FormulaTree>, Self::Data) {
        let bool_target = target.iter().copied().collect();
        let bool_operators = operators.filter_bool();
        let mut cache = convert_cache_beam_search(cache, self.beam_width, bool_target);
        let f = enum_aux(
            &mut cache,
            &bool_operators,
            &bool_target,
            self.max_size_bool,
        );

        let f_str = f.map(|f| rebuild_formula(&f, &cache));
        (f_str, ())
    }

    fn name() -> &'static str {
        "beam_search"
    }
}

fn convert_cache_beam_search(
    cache: InitialBoolCache,
    max_line_size: usize,
    target: CharVec,
) -> BeamSearchCache {
    let mut bs_cache = BeamSearchCache::new(max_line_size);

    for (size, cache) in cache.iter_lines().into_iter().enumerate() {
        let mut new_line = bs_cache.new_line(size);

        for (cv, t, size) in cache {
            let cv = cv.into_iter().collect();
            let f = BoolFormula::new_base(BoolCharac::from_cv(cv, target), size, t);
            new_line.push(f);
        }
    }

    bs_cache
}
