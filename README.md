# Wave Function Collapse

![Rust](https://img.shields.io/badge/Rust-FA7343?style=for-the-badge&logo=rust&logoColor=white)
![Crates.io](https://img.shields.io/crates/v/wavefc?style=for-the-badge)

## Character Map Demo

<div align="center">
  <img alt="Character Map Demo" src="https://i.postimg.cc/8cMKf69Y/Trimmed-2.gif" width="85%" height="85%" style="border-radius:12px;"/>
</div>

## Overview

A home-grown implementation of the [Wave Function Collapse algorithm](https://github.com/mxgmn/WaveFunctionCollapse) written in Rust.

At its core, this function is designed to take in a sample input, of any kind, and create a new output based on that. The `Wave` engine behind the algorithm has been designed to operate on bits and bitsets. This enables it to run against any two-dimensional, grid-based input: Character maps, *images*, video-game levels, sudoku puzzles, etc. The only requirement is that an adapter be written to support the given format. Currently, the program has only been written to support a character map text input and image input.

This project is subject to the MIT license as described in the `license.txt` file.

## Quick Setup

For those who are unfamiliar with Rust and just want to build the project from source, you're in the right spot. To get started, install the Rust toolchain over at [rustup.rs](https://rustup.rs).

Once you've successfully installed the toolchain, simply navigate into the source directory. Presumably you have already cloned this from the repo or downloaded it. Once in the directory, simply run `cargo b --release` to build the project. To actually run the project, use `cargo r --release -- <ARGS>`.

This isn't a tutorial for the Rust language or Cargo. So please check out their excellent resources and documentation over at [rust-lang.org](https://rust-lang.org).

## Discussion

By default, the CLI uses the sea, land, coast example as Robert Heaton describes in his [blog](https://robertheaton.com/2018/12/17/wavefunction-collapse-algorithm/) post on the algorithm. This can be seen in the character map demo above. I think this way of introducing the algorithm makes a lot of sense. It's easy to understand and grasp in a practical way. It's based off the `sample.txt` source file in the `wavefc` crate. It's embedded at compile-time into the program to provide this functionality, though the size is beyond negligible.

The CLI and core logic have been separated into two packages in the source: `wavefc-cli` and `wavefc`. To create your own custom adapters and write your own CLI, I recommend using the `wavefc-cli` source as a template and then building off of the `wavefc` types from there. During development, I personally found it beneficial to build and run in `--release` mode in Cargo. The build time difference between the two is negligible, and the actual collapse is much quicker in this optimized mode.

The program requires that a width and height be provided to size the output from. Feel free to play around with these values to create differently shaped outputs. Please note that the larger the size of the output, the longer the function generally takes, as the chances for it to contradict itself increase (the chances of failure have significantly dropped past commit 1846f0f). Theoretically, if the output size specified is lower than or equal to the sample's size, there exists a valid result. The output size specified must be a product of the tile size. Tile sizes are explored in the next paragraph.

In the version 1 and version 2 of `wavefc`, only the simple-tiled model was implemented for the algorithm. This severely limited its "creative" capabilities, creating rather dull outputs. In the current version of the algorithm, the overlapping-tiled model is used. This produces much better outputs and also more quickly in certain cases. Though, this model generally takes longer than the simple-tiled model. Luckily, the new overlapping logic is simply a more advanced superset of the original approach. This means that by specifying the tile size to be 1, you are essentially using the simple-tiled model.

The way the algorithm chooses which tile (superposition) to collapse next is based on the calculated entropy of a particular location. This is calculated using the probabilities of each of the possible values occurence in the original sample. These are all taken together to form the collective entropy for a given superposition.

<div align="center">
  <img src="https://latex.codecogs.com/png.image?\dpi{110}\bg{white}\sum_{i=0}^{n}-p_i\log_{2}({p_i})"/>
</div>

The `wavefc` library is currently single-threaded, given the sequential nature of the algorithm. However, I do recognize that a lot of performance optimizations could be made by sprinkling in some multi-threading. This is a long-term goal for the project, at the moment. I'm eyeing `rayon` pretty frequently for this specific project.

The CLI has a whole host of flags to tweak the program's settings. There are too many to cover in detail, and doing so would be frivilous regardless. However, by using the `clap` library, the help flag is supported to show a list of all available flags.

## Using this Project in your Code

This project is available in two packages on [crates.io](https://crates.io): `wavefc` and `wavefc-cli`. If you just want to give the program a go, `wavefc-cli` is probably your best bet to install. If you want to use this algorithm in your own code, adding `wavefc` to your `Cargo.toml` should suffice. As an alternative, you can use this project by manually copying its source or including it in a Cargo workspace.

A good place to start getting familiar with the source is `wavefc/src/lib.rs`, which holds the majority of the actual `Wave` code. To include this code in Rust, use the prelude with `use wavefc::prelude::*;`.

Copyright © Zachary Morden 2022
