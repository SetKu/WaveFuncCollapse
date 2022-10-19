# Wave Function Collapse

## Demo

![Demonstration Gif](https://i.postimg.cc/DwTBHrFc/Clean-Shot-2022-10-18-at-22-51-45.gif)

## Overview

A home-grown implementation of the [Wave Function Collapse algorithm](https://github.com/mxgmn/WaveFunctionCollapse) written in Rust. It takes in a sample character map, analyzes it, and then attempts to find a valid solution based on it. The greater the detail provided in the sample text file, the more quickly the algorithm will find a solution, in general. 

The output map it creates can be varied in width and height. Additionally, transformations, i.e. rotations and reflections, to the image can be applied, optionally. This option is turned off by default, but can be enabled by the `-t` or `--transform` flag. The reason it's turned off by default is that it often produces results less true to their input. However, it does significantly increase the algorithm's speed. Currently, the algorithm is single threaded, but it could be expanded to be mutlithreaded and run in parallel. Though, this would produce rather minor benefits, and even slow the algorithm down, when producing small dimensions. The algorithm is also capable of choosing to include or exclude diagonal positions. By default, diagonal rule analyses are included. To disable this, pass the flag `-d` or `--no-diagonals`. 

With the default sample implementation, a character map is made based on three identifiers, or entities: 'S' (Sea), 'L' (Land), and 'C' (Coast). The rules with these types is that sea can be next to coast and coast can be next land. The invalid combination here is that sea cannot be next to land, it must be bridged by coast. The algorithm is weighted based on the quantity of the provided type in the sample image. This feature can also be disabled by passing the `-t` or `--no-weights` flag. The entities can be replaced with any other character, and is adaptable. As a matter of syntax, the sample file should have each of its entities separated by a comma and a space. This is demonstrated in the default `sample.txt` file. To provide a custom sample to go off of, provide the path to the file relative to the binary using the flag `-s:<PATH>` (e.g. `-s:custom_sample.txt`).

The width and height of the output can be customized through the `-w<CUSTOM WIDTH>` or `-h<CUSTOM HEIGHT>` flags (e.g. `-w10` or `-h3`). The greater the width or height, the longer it usually takes for a valid result to be found with no contradictions. Contradictions are cases where, during the algorithm's attempt to find a solution, it finds a contradiction where the superposition it's checking cannot be satisfied by any of the possible entities.

In terms of dependencies, the only dependecy for this project is the [rand](https://crates.io/crates/rand) crate, which provides truly random functionality for a variety of inbuilt types.
