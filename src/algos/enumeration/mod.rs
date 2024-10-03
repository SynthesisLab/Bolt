//! Semanting Enumration algorithm for LTL and Boolean Synthesis.

pub(crate) mod aux;

use aux::enum_aux;
use clap::Args;

use crate::{
    bool::{cache::BoolCache, charac::BoolCharac, cv::CharVec, BoolFormula},
    cache::{EnumFormulaCache, EnumFormulaCacheLine},
    formula::{rebuild_formula, tree::FormulaTree},
    ltl::trace::Operators,
};

use super::{meta::cache::InitialBoolCache, BoolAlgoParams};

#[derive(Args, Clone, Copy)]
pub struct EnumParams {
    max_size_bool: usize,
    domin_nb: usize,
}

impl BoolAlgoParams for EnumParams {
    type Data = Vec<usize>;

    fn run(
        &self,
        cache: InitialBoolCache,
        operators: Operators,
        target: &[bool],
    ) -> (Option<FormulaTree>, Self::Data) {
        let bool_target: CharVec = target.iter().copied().collect();
        let mut bool_cache = convert_cache_enum(cache, bool_target, self.domin_nb);
        let bool_operators = operators.filter_bool();
        let f = enum_aux(
            &mut bool_cache,
            &bool_operators,
            &bool_target,
            self.max_size_bool,
        );

        let f_str = f.map(|f| rebuild_formula(&f, &bool_cache));
        (
            f_str,
            bool_cache
                .iter_lines()
                .into_iter()
                .map(|l| l.len())
                .collect(),
        )
    }

    fn name() -> &'static str {
        "bool_enum"
    }
}

fn convert_cache_enum(cache: InitialBoolCache, target: CharVec, k: usize) -> BoolCache {
    let mut bs_cache = BoolCache::new(k);

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
