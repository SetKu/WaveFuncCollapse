use clap::{arg, value_parser, Arg, Command, ArgMatches};
use wavefc::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

pub fn string_mode(matches: &ArgMatches) -> Result<(), String> {
    let max_contradictions_default = 20;

    let width = *matches.get_one::<usize>("width").unwrap();
    let height = *matches.get_one::<usize>("height").unwrap();
    let tilesize = matches.get_one::<usize>("tilesize");
    let tilewidth = matches.get_one::<usize>("tilewidth");
    let tileheight = matches.get_one::<usize>("tileheight");
    let pathbuf = matches.get_one::<PathBuf>("sample");
    let print = !matches.get_flag("noprint");
    let use_weights = !matches.get_flag("noweights");
    let use_transforms = !matches.get_flag("notransforms");
    let max_contradictions = matches.get_one::<usize>("attempts");
    let use_whitespace = matches.get_flag("whitespace");
    let disablecommas = matches.get_flag("disablecom");

    let input: String = if let Some(buf) = pathbuf {
        let content =
            fs::read_to_string(buf).expect("The sample provided cannot be read or is invalid");

        if !disablecommas {
            content.replace(", ", "").replace(",", "")
        } else {
            content
        }
    } else {
        let content = include_str!("sample.txt").to_string();

        if !disablecommas {
            content.replace(", ", "").replace(",", "")
        } else {
            content
        }
    };

    if input.is_empty() {
        panic!("The input sample cannot be empty")
    }

    let (sample, source_map) = deconstruct_string(&input, use_whitespace);
    let dimensions = dimensions_of(&sample);

    if dimensions.x == 0 && dimensions.y == 0 {
        println!("Warning: The sample provided has no items.");
    }

    let chunk_size = if tilesize.is_some() {
        let mut size = Vector2::new(*tilesize.unwrap(), *tilesize.unwrap());

        if let Some(width) = tilewidth {
            size.x = *width;
        }

        if let Some(height) = tileheight {
            size.y = *height;
        }

        size
    } else {
        Vector2::new(1, 1)
    };

    let mut wave = Wave::new();

    if !use_transforms {
        wave.flags.push(Flags::NoTransforms);
    }

    if !use_weights {
        wave.flags.push(Flags::NoWeights);
    }

    let a_start = Instant::now();
    wave.analyze(sample, chunk_size, BorderMode::Clamp);
    let a_dur = a_start.elapsed();
    wave.fill(Vector2::new(width, height))?;

    let real_contradictions = if let Some(max) = max_contradictions {
        *max
    } else {
        max_contradictions_default
    };

    let midway_print = Some(
        |iterations: usize, failures: usize, current_rep: Vec<Vec<Vec<usize>>>| {
            let string = construct_wip_string(current_rep, &source_map);
            println!(
                "Iteration: {}, Attempt: {}\n{}\n",
                iterations + 1,
                failures + 1,
                string
            );
        },
    );

    let c_start = Instant::now();
    wave.collapse_all(real_contradictions, if print { midway_print } else { None })?;
    let c_dur = c_start.elapsed();

    let result = wave.perfect_rep()?;
    let string = reconstruct_string(result, &source_map, print, print);
    println!("{}", string);

    if print {
        println!("\nAnalysis Time: {:?}", a_dur);
        println!("Collapse Time: {:?}", c_dur);
    }

    Ok(())
}
