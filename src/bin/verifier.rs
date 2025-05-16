use std::path::PathBuf;

use clap::Parser;

use bolt::ltl::trace::traces_from_file;
use log::info;

fn main() {
    env_logger::init();

    let args = CliArgs::parse();
    info!("Reading file '{}'", args.input_filename.display());

    let instance = traces_from_file(&args.input_filename);

    let f = instance
        .formula
        .expect("This instance does not contain a smallest_known_formula");

    assert_eq!(f.eval(&instance.traces).accepted_vec(), instance.target);
    info!("All checks succesful!");
    println!("All checks were succesful");
}

#[derive(Parser)]
struct CliArgs {
    /// Name of the .json file to read.
    input_filename: PathBuf,
}
