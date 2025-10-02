# Bolt: Blazingly Fast $\mathsf{LTL}_f$ (and $\mathsf{LTL}$) Learning

This repository contains the code accompanying the paper [*LTLf Learning Meets Boolean Set Cover*](https://arxiv.org/abs/2509.24616), by Gabriel Bathie, Nathanaël Fijalkow, Théo Matricon, Baptiste Mouillon, and Pierre Vandenhove. It provides instructions for running the code and reproducing our experimental results.

A benchmark suite for $\mathsf{LTL}_f$ is provided in a [separate repository](https://github.com/SynthesisLab/LTLf_Learning_Benchmarks).

## Requirements
To run the code, you need to have [Rust](https://www.rust-lang.org/tools/install) installed. The code can then be compiled using:
```
cargo build --release
```

## Running the code
Three algorithms (`beam-search`, `set-cover`, `enum`) can be run with multiple parameters by changing command line arguments. For instance,
```
cargo run --release -- example_input.json 8 10 beam-search 100 70
```
runs the beam search algorithm on file `example_input.json` with LTL2BS-switch = 8, size-dom = 10, beam-width = 100, and D&C-switch = 70.

To see the command line arguments and options, use `cargo run -r -- -h`

The Rust source code is in directory `src/`.
