# Bolt: Blazingly Fast LTL Learning

This repository contains the code accompanying our paper *LTLf Learning Meets Boolean Set Cover*. It provides instructions for running the code and reproducing our experimental results.

## Requirements
To run the code, you need to have [Rust](https://www.rust-lang.org/tools/install) installed. The code can be compiled using:

```
cargo build --release
```

## Running the code

The three algorithms (`set-cover`, `enum`, `beam-search`) can be run with multiple parameters by changing command line arguments. For instance,
```
cargo run --release -- example_input.json 8 10 beam-search 100 70
```
runs the beam search algorithm on file `example_input.json` with LTL2BS-switch = 8, size-dom = 10, beam-width = 100, and D&C-switch = 70.

To see the command line arguments and options, use `cargo run -r -- -h`

The Rust source code is in directory `src/`.
