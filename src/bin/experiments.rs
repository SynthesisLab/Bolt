use std::path::PathBuf;

use clap::{Parser, Subcommand};

use bolt::{
    algos::{
        BoolAlgoParams, beam_search::BeamSearchParams, enumeration::EnumParams,
        meta::divide_conquer, set_cover::SetCoverParams,
    },
    formula::tree::FormulaTree,
    ltl::trace::{Operators, Trace, traces_from_file},
};
use log::info;

fn main() {
    env_logger::init();

    let args = CliArgs::parse();
    let instance = traces_from_file(&args.input_filename);

    match args.command {
        AlgoCommand::BeamSearch(p) => {
            let (time, sol, name) = get_name_time_sol(
                instance.traces,
                instance.atomic_propositions,
                instance.operators,
                instance.target,
                args.max_size_ltl,
                args.domin_nb,
                p,
            );

            println!(
                "rust_{}, {}, {:.5}, {}, {}, {}, {}, {}, {}",
                name,
                args.input_filename.to_string_lossy(),
                time,
                args.max_size_ltl,
                args.domin_nb,
                p.beam_width,
                p.max_size_bool,
                sol.as_ref().map_or(-1, |f| f.size() as isize),
                sol.map_or(String::new(), |f| format!("{f}"))
            )
        }
        _ => panic!("Wrong command used for experiments"),
    };
}

fn get_name_time_sol<P: BoolAlgoParams + Clone>(
    traces: Vec<Trace>,
    atomic_propositions: Vec<String>,
    operators: Operators,
    target: Vec<bool>,
    max_size_ltl: usize,
    domin_nb: usize,
    params: P,
) -> (f64, Option<FormulaTree>, &'static str) {
    let res = divide_conquer(
        &traces,
        atomic_propositions,
        operators,
        target.clone(),
        max_size_ltl,
        domin_nb,
        params,
    );

    if let Some(t) = res.sol() {
        let actual_value = t.eval(&traces).accepted_vec();
        assert_eq!(actual_value, target);
        info!("Correctness check OK!");
    }

    (res.total_time_sec(), res.sol(), P::name())
}

#[derive(Parser)]
// #[command(version, about, long_about = None)]
struct CliArgs {
    /// Name of the .trace file to read.
    input_filename: PathBuf,
    /// Run LTL enumeration until `max_size_ltl`
    /// before switching to boolean algorithm.
    max_size_ltl: usize,
    /// Number of candidates to use for domination checking
    /// in the step that converts LTL formulas to boolean formulas.
    domin_nb: usize,
    #[command(subcommand)]
    command: AlgoCommand,
}

#[derive(Subcommand)]
enum AlgoCommand {
    /// Exhaustive enumeration algorithm
    Enum(EnumParams),
    /// Set cover appoximation algorithm
    SetCover(SetCoverParams),
    /// Bottom-up beam search
    BeamSearch(BeamSearchParams),
}
