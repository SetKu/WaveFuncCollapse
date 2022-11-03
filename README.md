# Wave Function Collapse

![Swift](https://img.shields.io/badge/Rust-FA7343?style=for-the-badge&logo=rust&logoColor=white)

## Demo

<div align="center">
  <img alt="Demonstration Gif" src="https://i.postimg.cc/tJmT59Tr/Clean-Shot-2022-11-03-at-08-55-34.gif" width="50%" height="50%" style="border-radius:12px;"/>
</div>

## Overview

A home-grown implementation of the [Wave Function Collapse algorithm](https://github.com/mxgmn/WaveFunctionCollapse) written in Rust. At its core, this function is designed to take in a sample input, of any kind, and create a new output from that. The algorithm has been built upon character maps from text files, but can easily be adapted to any other type a generic `Sample` can be created from. For this string-based interpretation, the greater the detail provided in the sample text file, the more quickly the algorithm will run through attempts 🧪. 

The program requires that a width and height be provided to size the output from. These are simply positive integers. Feel from to play around with these values to create differently shaped outputs. Please note that the larger the size of the output, the longer the function generally takes, as the chances for it to contradict itself increase.

The output created by the function can be varied through the use of several options and flags, which can be brought up using the `-h` or `--help` options. Documentation for those flags in included there. The options provided enable various changes, such as disabling transforms or weights. Please note, that, by default, the program runs in an artificially slow form. This enables human-like snapshots of the output in real-time. This is shown in the demonstration gif above. To disable this feature specifically, pass the `-q` flag.

In terms of dependencies, the only two dependencies for this project is the [rand](https://crates.io/crates/rand) crate and [clap](https://docs.rs/clap/latest/clap/) crate. Both are great projects and I encourage you to check them out.
