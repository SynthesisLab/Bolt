use log::{debug, info};

use crate::ltl::trace::Operators;

use crate::{
    cache::{EnumFormulaCache, EnumFormulaCacheLine},
    formula::{apply_binary, apply_unary, Formula},
    ops::traits::{BinaryOp, Commutativity, UnaryOp},
    traits::{EqTarget, Hashed},
};

use std::fmt::Debug;

pub(crate) fn enum_aux<Cache, Char>(
    cache: &mut Cache,
    operators: &Operators,
    target: &Char::TargetType,
    max_size: usize,
) -> Option<Formula<Char>>
where
    Char: UnaryOp + BinaryOp + Debug + Eq + EqTarget + Hashed + Clone + Debug,
    Char::TargetType: Debug,
    Cache: EnumFormulaCache<Char> + Debug,
{
    let start_size = cache.nb_lines();

    debug!(
        "Starting boolean search, start size {}, max size {}",
        start_size, max_size
    );
    debug!("operators: {operators:?}");
    debug!("target: {target:?}");

    for size in start_size..=max_size {
        debug!(
            "Iteration for size {}, cache has size {}",
            size,
            cache.len(),
        );
        let (iter, pair_iter, mut new_line) = cache.new_line_and_iter_size(size);

        debug!("  Unary:");
        let res = aux_search_unary::<Cache, Char>(iter, &mut new_line, operators, target);
        match res {
            Ok(f) => {
                info!("Found formula");
                return Some(f);
            }
            Err(hits) => debug!("    Got {hits} hits"),
        }

        debug!("  Binary:");
        let res = aux_search_binary::<Cache, Char>(pair_iter, &mut new_line, operators, target);

        match res {
            Ok(f) => {
                info!("Found formula");
                return Some(f);
            }

            Err(hits) => debug!("    Got {hits} hits"),
        }
    }

    info!("Not found, exiting");
    None
}

fn aux_search_unary<'a, Cache, Char>(
    formula_iter: impl Iterator<Item = &'a Formula<Char>>,
    new_cache: &mut Cache::CacheLine<'a>,
    operators: &Operators,
    target: &Char::TargetType,
) -> Result<Formula<Char>, usize>
where
    Char: UnaryOp + EqTarget + Hashed + 'a + Debug,
    Cache: EnumFormulaCache<Char>,
{
    let mut hits = 0;

    let ops = &operators.unary;
    if ops.is_empty() {
        return Err(0);
    }

    for f in formula_iter {
        for &op in ops {
            let g = apply_unary(op, f);
            if g.eq_target(target) {
                return Ok(g);
            }

            if !new_cache.push(g) {
                hits += 1;
            }
        }
    }
    Err(hits)
}

fn aux_search_binary<'a, Cache, Char>(
    pair_iter: impl Iterator<Item = (&'a Formula<Char>, &'a Formula<Char>)>,
    new_cache: &mut Cache::CacheLine<'a>,
    operators: &Operators,
    target: &Char::TargetType,
) -> Result<Formula<Char>, usize>
where
    Char: BinaryOp + EqTarget + Hashed + 'a + Clone + Debug,
    Cache: EnumFormulaCache<Char>,
{
    let mut hits = 0;

    let ops = &operators.binary;
    if ops.is_empty() {
        return Err(0);
    }

    for (f_l, f_r) in pair_iter {
        for &op in ops {
            let g = apply_binary(op, f_l, f_r);
            if g.eq_target(target) {
                return Ok(g);
            }

            if !new_cache.push(g) {
                hits += 1;
            }

            if op.commutes() {
                continue;
            }

            // For non-commutative operations
            let g = apply_binary(op, f_r, f_l);
            if g.eq_target(target) {
                return Ok(g);
            }

            if !new_cache.push(g) {
                hits += 1;
            }
        }
    }
    Err(hits)
}
