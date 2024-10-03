//! Set cover algorithm for Boolean Synthesis.
//!
//! Produces Or-of-And or And-of-Or formulas in a greedy fashion.

mod aux;
mod cache;

use aux::aux_set_cover;
use cache::ScCache;
use clap::Args;
use log::info;

use crate::{
    bool::{charac::BoolCharac, cv::CharVec, BoolFormula},
    formula::{rebuild_formula, tree::FormulaTree},
    ltl::trace::Operators,
    ops::binary::LtlBinaryOp,
};

use super::{meta::cache::InitialBoolCache, BoolAlgoParams};

#[derive(Args, Clone, Copy)]
pub struct SetCoverParams {
    /// Maximum number of greedy formulas to generate
    /// before moving to the other operator.
    max_nb_formulas: usize,
    placeholder: usize,
}

impl BoolAlgoParams for SetCoverParams {
    type Data = ();

    fn run(
        &self,
        cache: InitialBoolCache,
        _operators: Operators,
        target: &[bool],
    ) -> (Option<FormulaTree>, Self::Data) {
        let target_cv = target.iter().copied().collect();
        let mut sc_cache = convert_cache_sc(cache, target_cv);
        let f = set_cover_bool(&mut sc_cache, target, self.max_nb_formulas);
        let f_str = f.map(|f| rebuild_formula(&f, &sc_cache));
        (f_str, ())
    }

    fn name() -> &'static str {
        "set_cover"
    }
}

fn set_cover_bool(
    cache: &mut ScCache,
    target: &[bool],
    max_nb_formulas: usize,
) -> Option<BoolFormula> {
    let positive_count = target.iter().filter(|b| **b).count();
    let negative_count = target.len() - positive_count;

    let formulas: Vec<_> = cache.into_iter().cloned().collect();

    info!("Computing C_p");
    let cp = positive_set_cover(cache, formulas.clone(), positive_count, max_nb_formulas);
    info!("Computing C_p,n");
    let cpn = negative_set_cover(cache, cp, negative_count, max_nb_formulas);

    info!("Computing C_n");
    let cn = negative_set_cover(cache, formulas, negative_count, max_nb_formulas);
    info!("Computing C_n,p");
    let cnp = positive_set_cover(cache, cn, positive_count, max_nb_formulas);

    cpn.into_iter().chain(cnp).min_by_key(|f| f.size)
}

fn positive_set_cover(
    cache: &mut ScCache,
    formulas: Vec<BoolFormula>,
    positive_count: usize,
    max_nb_formulas: usize,
) -> Vec<BoolFormula> {
    aux_set_cover(
        cache,
        formulas,
        |f| f.sat_positive_count(),
        positive_count,
        LtlBinaryOp::Or,
        max_nb_formulas,
    )
}

fn negative_set_cover(
    cache: &mut ScCache,
    formulas: Vec<BoolFormula>,
    negative_count: usize,
    max_nb_formulas: usize,
) -> Vec<BoolFormula> {
    aux_set_cover(
        cache,
        formulas,
        |f| f.sat_negative_count(),
        negative_count,
        LtlBinaryOp::And,
        max_nb_formulas,
    )
}

fn convert_cache_sc(bool_cache: InitialBoolCache, target: CharVec) -> ScCache {
    let mut sc_cache = ScCache::new();

    for (cv, t, size) in bool_cache {
        let cv = cv.into_iter().collect();
        let f = BoolFormula::new_base(BoolCharac::from_cv(cv, target), size, t);
        sc_cache.push(f);
    }

    sc_cache
}
