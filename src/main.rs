// declare dependencies
extern crate cgmath;
extern crate clap;
extern crate wfc;

use cgmath::Vector2;
use clap::{arg, crate_version, value_parser, Arg, Command};
use std::fs;
use std::path::PathBuf;
use wfc::{helpers::xy_swap, BorderMode, Flags, Wave};

fn main() -> Result<(), String> {
    let matches = Command::new("Wave Function Collapse")
        .version(crate_version!())
        .arg(Arg::new("width")
            .required(true)
            .value_parser(value_parser!(usize)))
        .arg(Arg::new("height")
            .required(true)
            .value_parser(value_parser!(usize)))
        .arg(arg!( -s --sample <file> "Use a custom sample file instead of the default sea, land, coast example." )
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!( -m --tilesize <number> "Specify the tile size used in the analysis and result. By default this value is 1." )
            .value_parser(value_parser!(usize)))
        .arg(arg!( -j --tilesizewidth <number> "Specify the tile size width (precedent over --tilesize)." )
            .value_parser(value_parser!(usize)))
        .arg(arg!( -k --tilesizeheight <number> "Specify the tile size height (precedent over --tilesize)." )
            .value_parser(value_parser!(usize)))
        .arg(arg!( -c --contradictions <number> "The maximum number of contradictions (attempts) that can be reached before the program panics.")
            .value_parser(value_parser!(usize)))
        .arg(arg!( -p --noprint "(Fast Mode 🚀) Disables incrementally printing the function's progress. This also removes artificially induced delays for human readability."))
        .arg(arg!( -q --quiet "Makes the output suitable for analysis by other programs, removing the commas and label text. This option also invokes the 'p' flag."))
        .arg(arg!( -w --noweights "Disables using weights in the randomization process."))
        .arg(arg!( -t --notransforms "Disables using transforms in rule analysis."))
        .arg(arg!( -l --usewhitespace "Takes into account whitespace in the sample."))
        .get_matches();

    let width = *matches.get_one::<usize>("width").unwrap();
    let height = *matches.get_one::<usize>("height").unwrap();
    let tilesize = matches.get_one::<usize>("tilesize");
    let tilesize_width = matches.get_one::<usize>("tilesizewidth");
    let tilesize_height = matches.get_one::<usize>("tilesizeheight");
    let pathbuf = matches.get_one::<PathBuf>("sample");
    let mut print = !matches.get_flag("noprint");
    let use_weights = !matches.get_flag("noweights");
    let use_transforms = !matches.get_flag("notransforms");
    let simple_output = matches.get_flag("quiet");
    let max_contradictions = matches.get_one::<usize>("contradictions");
    let use_whitespace = matches.get_flag("usewhitespace");

    if simple_output {
        print = false;
    }

    let input: String = if let Some(buf) = pathbuf {
        fs::read_to_string(buf)
            .expect("The sample provided cannot be read or is invalid")
            .replace(", ", "")
            .replace(",", "")
    } else {
        include_str!("sample.txt")
            .to_string()
            .replace(", ", "")
            .replace(",", "")
    };

    if input.is_empty() {
        panic!("The input sample cannot be empty")
    }

    let (sample, source_map) = deconstruct_string(&input, use_whitespace);

    debug_assert_eq!(sample.len(), input.lines().count());

    let chunk_size = if tilesize.is_some() {
        let mut size = Vector2::new(*tilesize.unwrap(), *tilesize.unwrap());

        if let Some(width) = tilesize_width {
            size.x = *width;
        }

        if let Some(height) = tilesize_height {
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

    wave.analyze(sample, chunk_size, BorderMode::Clamp);
    wave.fill(Vector2::new(width, height))?;

    let result = wave.current_rep();
    let string = construct_wip_string(result, &source_map);
    println!("{}", string);

    wave.collapse_once();

    let result = wave.current_rep();
    let string = construct_wip_string(result, &source_map);
    println!("{}", string);

    Ok(())
}

fn deconstruct_string(
    input: &String,
    use_whitespace: bool,
) -> (Vec<Vec<usize>>, Vec<(usize, char)>) {
    // convert string input into a usable bitset-based sample
    let mut sample: Vec<Vec<usize>> = vec![];
    sample.reserve(input.lines().count());
    let mut source_map: Vec<(usize, char)> = vec![];
    let mut id_counter = 0usize;

    for (row, line) in input.lines().enumerate() {
        for (_, ch) in line.chars().enumerate() {
            if !use_whitespace {
                if ch.is_whitespace() {
                    continue;
                }
            }

            if sample.len() < row + 1 {
                sample.push(vec![]);
            }

            if let Some(translation) = source_map.iter().find(|t| t.1 == ch) {
                sample[row].push(translation.0);
            } else {
                source_map.push((id_counter, ch));
                sample[row].push(id_counter);
                id_counter += 1;
            }
        }
    }

    (xy_swap(sample), source_map)
}

fn construct_wip_string(input: Vec<Vec<Vec<usize>>>, source_map: &Vec<(usize, char)>) -> String {
    let swapped = xy_swap(input);
    let mut output = "".to_string();
    let mut lines_added = 0;

    for (r, row) in swapped.iter().enumerate() {
        if lines_added < r + 1 {
            output.push('\n');
            lines_added += 1;
        }

        for vals in row {
            let mut mapped: Vec<char> = vals
                .iter()
                .map(|v| source_map.iter().find(|s| s.0 == *v).unwrap().1)
                .collect();
            mapped.sort();

            let mut string = "(".to_string();

            for ch in mapped {
                string.push_str(&format!("{}", ch));
            }

            string.push(')');
            output.push_str(&string);
        }
    }

    output
}

fn reconstruct_string(input: Vec<Vec<usize>>, source_map: &Vec<(usize, char)>) -> String {
    let swapped = xy_swap(input);
    let mut output = "".to_string();

    for (r, row) in swapped.iter().enumerate() {
        if output.lines().count() < r + 1 {
            output.push('\n');
        }

        for id in row {
            let real_val = source_map.iter().find(|s| s.0 == *id).unwrap().1;
            output.push_str(&format!("{}, ", real_val));
        }
    }

    output
}
