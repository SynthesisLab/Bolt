use std::path::PathBuf;

use clap::{Parser, Subcommand};

use log::info;
use ltl_rs::{
    algos::{
        beam_search::BeamSearchParams, enumeration::EnumParams, meta::divide_conquer,
        set_cover::SetCoverParams, BoolAlgoParams,
    },
    formula::tree::FormulaTree,
    ltl::trace::traces_from_file,
};

fn main() {
    env_logger::init();

    let CliArgs {
        input_filename,
        max_size_ltl,
        domin_nb,
        command,
    } = CliArgs::parse();

    let sol = match command {
        AlgoCommand::Enum(p) => get_name_time_sol(&input_filename, max_size_ltl, domin_nb, p),
        AlgoCommand::SetCover(p) => get_name_time_sol(&input_filename, max_size_ltl, domin_nb, p),
        AlgoCommand::BeamSearch(p) => get_name_time_sol(&input_filename, max_size_ltl, domin_nb, p),
    };

    println!("{}", sol.map_or(String::new(), |f| format!("{f}")))
}

fn get_name_time_sol<P: BoolAlgoParams + Clone>(
    fname: &PathBuf,
    max_size_ltl: usize,
    domin_nb: usize,
    params: P,
) -> Option<FormulaTree> {
    let (traces, alphabet, target, operators) = traces_from_file(fname);

    let res = divide_conquer(
        &traces,
        alphabet,
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

    res.sol()
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
