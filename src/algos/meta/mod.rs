//! Meta algorithms: Divide and Conquer, ...
use std::{rc::Rc, time::Instant};

use cache::InitialBoolCache;
use itertools::Itertools;
use log::{debug, info, trace};
use meta_res::{MetaRes, MetaResult};

use crate::{
    algos::{atoms, create_initial_cache, enumeration::aux::enum_aux},
    cache::FormulaCache,
    formula::{rebuild_formula, tree::FormulaTree},
    ltl::trace::{Operators, Trace},
    ops::binary::LtlBinaryOp,
};

use super::BoolAlgoParams;

pub mod cache;
pub mod meta_res;

/// LTL search followed by Divide and Conquer.
pub fn divide_conquer<P>(
    traces: &[Trace],
    alphabet: Vec<String>,
    operators: Operators,
    target: Vec<bool>,
    max_size_ltl: usize,
    domin_nb: usize,
    params: P,
) -> MetaResult<P::Data>
where
    P: BoolAlgoParams + Clone,
{
    let start = Instant::now();

    let atoms = atoms(traces, alphabet);
    // Add initial formulas
    let (atom, mut ltl_cache) = create_initial_cache(atoms, &target);
    // Check if target is an atom
    if let Some(f) = atom {
        let ltl_time = start.elapsed();
        let f_str = rebuild_formula(&f, &ltl_cache);

        return MetaResult {
            ltl_time,
            ltl_cache_sizes: vec![],
            algo_time: None,
            algo_data: None,
            result: MetaRes::Atom(f_str),
        };
    }

    // Ltl search
    let ltl_res = enum_aux(&mut ltl_cache, &operators, &target, max_size_ltl);

    let ltl_time = start.elapsed();
    let ltl_cache_sizes = ltl_cache.lines.iter().map(|l| l.len()).collect();
    if let Some(f) = ltl_res {
        let f_str = rebuild_formula(&f, &ltl_cache);
        return MetaResult {
            ltl_time,
            ltl_cache_sizes,
            algo_time: None,
            algo_data: None,
            result: MetaRes::FoundByLtl(f_str),
        };
    }
    debug!("Ltl cache has size {}", ltl_cache.len());

    debug!("Running D&C with algo {}", P::name());
    let start = Instant::now();
    let initial_cache = InitialBoolCache::from_ltl_cache(domin_nb, ltl_cache, &target);
    debug!("Initial bool cache len: {}", initial_cache.len());
    let f = solve_or_split(traces, operators, initial_cache, &target, params);
    let algo_time = Some(start.elapsed());

    MetaResult {
        ltl_time,
        ltl_cache_sizes,
        algo_time,
        algo_data: None,
        result: match f {
            Some(f) => MetaRes::FoundByBool(f),
            None => MetaRes::NotFound,
        },
    }
}

/// Solve Boolean Synthesis problem using Divide and Conquer and the algorithm specified in `params`.
///
/// If the number of traces is more than 128 split immediately.
/// Otherwise, try to solve the instance with the algorithm implemented by `params`.
/// If no solution is found, try to find one by splitting recursively.
/// Splitting is handled using [`split_and_solve_non_overlapping`].
fn solve_or_split<P>(
    traces: &[Trace],
    operators: Operators,
    initial_cache: InitialBoolCache,
    target: &[bool],
    params: P,
) -> Option<FormulaTree>
where
    P: BoolAlgoParams + Clone,
{
    let nb_traces = target.len();
    // Check whether the fom
    if let Some(f) = initial_cache.get_from_cv(target, target) {
        debug!("Formula found in cache");
        return Some(f);
    }
    if nb_traces > 128 {
        split_and_solve_non_overlapping(traces, operators, initial_cache, target, params)
    } else {
        let (res, _) = params
            .clone()
            .run(initial_cache.clone(), operators.clone(), target);
        match res {
            Some(f) => Some(f),
            None => {
                split_and_solve_non_overlapping(traces, operators, initial_cache, target, params)
            }
        }
    }
}

fn _split_and_solve<P>(
    traces: &[Trace],
    operators: Operators,
    cache: InitialBoolCache,
    target: &[bool],
    params: P,
) -> Option<FormulaTree>
where
    P: BoolAlgoParams + Clone,
{
    let (op, left, right) = find_split(target)?;
    info!("Splitting on op '{op}'");
    let (left_cache, right_cache) = cache.split(&left, &right, target);
    info!(
        "Cache sizes: {} (left) {} right",
        left_cache.len(),
        right_cache.len()
    );

    let left_target = left.iter().map(|&i| target[i]).collect_vec();
    let left_traces = left.iter().map(|&i| traces[i].clone()).collect_vec();
    let left_res = solve_or_split(
        &left_traces,
        operators.clone(),
        left_cache,
        &left_target,
        params.clone(),
    )?;

    let right_target = right.iter().map(|&i| target[i]).collect_vec();
    let right_traces = right.iter().map(|&i| traces[i].clone()).collect_vec();
    let right_res = solve_or_split(&right_traces, operators, right_cache, &right_target, params)?;

    Some(FormulaTree::BinaryNode {
        op,
        left: Rc::from(left_res),
        right: Rc::from(right_res),
    })
}

/// Divide and conquer subrouting to split into two subproblems with clever merging.
///
/// We use [`find_split`] to get indices for the left subproblem, and solve it recursively.
/// If we get a solution, use the set of unsatisfied indices for the right subproblem,
/// instead of all the other indices.
/// As the left result might solve traces that were not included in the call, this yields much smaller formulas.
fn split_and_solve_non_overlapping<P>(
    traces: &[Trace],
    operators: Operators,
    cache: InitialBoolCache,
    target: &[bool],
    params: P,
) -> Option<FormulaTree>
where
    P: BoolAlgoParams + Clone,
{
    let (op, left, _) = find_split(target)?;
    info!("Splitting on op '{op}'");

    let left_cache = cache.reduce(&left, target);
    info!("Left cache size: {}", left_cache.len());

    let left_target = left.iter().map(|&i| target[i]).collect_vec();
    let left_traces = left.iter().map(|&i| traces[i].clone()).collect_vec();
    let left_res = solve_or_split(
        &left_traces,
        operators.clone(),
        left_cache,
        &left_target,
        params.clone(),
    )?;
    debug!("Found left formula {}", left_res);

    // Compute the indices of the traces that are not satisfied by the left result,
    // and only recurse on these.
    let solved = left_res.eval(traces).accepted_vec();
    let right = solved
        .into_iter()
        .zip(target.iter())
        .enumerate()
        .filter_map(|(i, (b1, &b2))| match op {
            // When splitting on a Or, we need to keep all negatives
            // and unsat positives, i.e. those for which `cv` is false.
            LtlBinaryOp::Or if !b2 | !b1 => Some(i),
            // When splitting on an And, we instead keep all positives
            // and unsat negatives, i.e. those for which `cv` is true.
            LtlBinaryOp::And if b2 | b1 => Some(i),
            _ => None,
        })
        .collect_vec();

    let nb_not_sat = right
        .iter()
        .filter(|&&i| match op {
            LtlBinaryOp::Or => target[i],
            LtlBinaryOp::And => !target[i],
            _ => unreachable!(),
        })
        .count();
    if nb_not_sat == 0 {
        debug!("0 left to satisfy, shortcut return");
        return Some(left_res);
    }

    debug!("Number of unsat after left call: {}", right.len());
    trace!("Unsat after call: {:?}", &right);

    let right_cache = cache.reduce(&right, target);
    let right_target = right.iter().map(|&i| target[i]).collect_vec();
    let right_traces = right.iter().map(|&i| traces[i].clone()).collect_vec();
    let right_res = solve_or_split(&right_traces, operators, right_cache, &right_target, params)?;
    debug!("Found right formula {}", right_res);

    let res = FormulaTree::BinaryNode {
        op,
        left: Rc::from(left_res),
        right: Rc::from(right_res),
    };
    debug!("Found formula {}", res);

    Some(res)
}

/// Split the largest of the negatives or the positive.
///
/// Returns the operation to use when merging, as well as two vectors of indices
/// of traces to keeps in each split.
///  
/// If the split was on the positives, the returned operation is [`LtlBinaryOp::Or`],
/// and otherwise it's [`LtlBinaryOp::And`].
fn find_split(target: &[bool]) -> Option<(LtlBinaryOp, Vec<usize>, Vec<usize>)> {
    let nb_traces = target.len();
    let nb_pos = target.iter().filter(|b| **b).count();
    let nb_neg = nb_traces - nb_pos;

    if nb_pos <= 1 && nb_neg <= 1 {
        return None;
    }

    let mut left = vec![];
    let mut right = vec![];
    let op = if nb_pos > nb_neg {
        LtlBinaryOp::Or
    } else {
        LtlBinaryOp::And
    };
    let mut j = 0;
    for (i, &t) in target.iter().enumerate() {
        if t == (nb_pos > nb_neg) {
            // Split alternatively between left and right.
            if j % 2 == 0 {
                left.push(i);
            } else {
                right.push(i);
            }
            j += 1;
        } else {
            left.push(i);
            right.push(i);
        }
    }

    Some((op, left, right))
}
