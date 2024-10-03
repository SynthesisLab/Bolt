use fxhash::FxHashSet;
use log::debug;

use crate::{bool::BoolFormula, formula::apply_binary, ops::binary::LtlBinaryOp};

use super::cache::ScCache;

pub(super) fn aux_set_cover<F>(
    cache: &mut ScCache,
    formulas: Vec<BoolFormula>,
    sat_fn: F,
    target_sat: usize,
    op: LtlBinaryOp,
    max_nb_formulas: usize,
) -> Vec<BoolFormula>
where
    F: Fn(&BoolFormula) -> usize + Copy,
{
    let mut res = vec![];
    let mut formulas: FxHashSet<BoolFormula> = formulas.into_iter().collect();

    'run: while !formulas.is_empty() && res.len() < max_nb_formulas {
        let mut best = take_max_by_key(&mut formulas, |f| sat_fn(f)).unwrap();

        while sat_fn(&best) < target_sat {
            if formulas.is_empty() {
                break 'run;
            }

            let (new_best, f) = formulas
                .iter()
                .map(|f| (apply_binary(op, &best, f), f))
                .max_by_key(|(new, _f)| sat_fn(new))
                .unwrap();
            formulas.remove(&f.clone());
            // If no progress has been made, abort.
            if sat_fn(&new_best) == sat_fn(&best) {
                break 'run;
            }
            cache.push(best);
            best = new_best;
        }

        assert_eq!(sat_fn(&best), target_sat);
        cache.push(best.clone());
        res.push(best);
    }

    debug!("Found {} formulas with aux_set_cover", res.len());

    res
}

fn take_max_by_key<T>(
    formulas: &mut FxHashSet<BoolFormula>,
    weight_fn: impl Fn(&BoolFormula) -> T,
) -> Option<BoolFormula>
where
    T: Ord,
{
    let target = formulas
        .iter()
        .max_by_key(|&f| weight_fn(f))
        .unwrap()
        .clone();
    formulas.take(&target)
}
