use super::DEFAULT_MAX_CONTRADICTIONS;
use crate::shared::SharedArgs;
use clap::ArgMatches;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use wavefc::prelude::*;

pub fn handler(matches: &ArgMatches) -> Result<(), String> {
    let pathbuf = matches.get_one::<PathBuf>("sample");
    let print = !matches.get_flag("noprint");
    let use_whitespace = matches.get_flag("whitespace");
    let disablecommas = matches.get_flag("disablecom");

    let shared_args = SharedArgs::from(matches);

    let preparation_start = Instant::now();

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

    let preparation_duration = preparation_start.elapsed();

    let dimensions = dimensions_of(&sample);

    if dimensions.x == 0 && dimensions.y == 0 {
        println!("Warning: The sample provided has no items.");
    }

    let chunk_size = if shared_args.tilesize.is_some() {
        let mut size = Vector2::new(
            *shared_args.tilesize.unwrap(),
            *shared_args.tilesize.unwrap(),
        );

        if let Some(width) = shared_args.tilewidth {
            size.x = *width;
        }

        if let Some(height) = shared_args.tileheight {
            size.y = *height;
        }

        size
    } else {
        Vector2::new(1, 1)
    };

    let mut wave = Wave::new();

    if !shared_args.use_transforms {
        wave.flags.push(Flags::NoTransforms);
    }

    if !shared_args.use_weights {
        wave.flags.push(Flags::NoWeights);
    }

    let a_start = Instant::now();
    wave.analyze(sample, chunk_size, BorderMode::Clamp);
    let a_dur = a_start.elapsed();
    wave.fill(Vector2::new(shared_args.width, shared_args.height))?;

    let real_contradictions = if let Some(max) = shared_args.max_contradictions {
        *max
    } else {
        DEFAULT_MAX_CONTRADICTIONS
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
    let string = reconstruct_string(result, &source_map, true, print);
    println!("{}", string);

    if print {
        println!(
            "\nAnalysis and Prep. Time: {:?}",
            preparation_duration + a_dur
        );
        println!("Collapse Time: {:?}", c_dur);
    }

    Ok(())
}
