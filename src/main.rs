
// declare dependencies
extern crate clap;
extern crate wfc;

use clap::{arg, crate_version, value_parser, Command, Arg};
use std::path::PathBuf;
use std::fs;
use wfc::Wave;

fn main() {
    let matches = Command::new("Wave Function Collapse")
        .version(crate_version!())
        .arg(Arg::new("width")
            .required(true)
            .value_parser(value_parser!(u32)))
        .arg(Arg::new("height")
            .required(true)
            .value_parser(value_parser!(u32)))
        .arg(arg!( -s --sample <file> "Use a custom sample file instead of the default sea, land, coast example." )
            .value_parser(value_parser!(PathBuf)))
        .arg(arg!( -m --tilesize <number> "Specify the tile size used in analysis and the result. The sample provided and output size must be a factor of the tile size. The default for this value is 2." )
            .value_parser(value_parser!(u16)))
        .arg(arg!( -c --contradictions <number> "The maximum number of contradictions (attempts) that can be reached before the program panics.")
            .value_parser(value_parser!(u32)))
        .arg(arg!( -p --noprint "(Fast Mode 🚀) Disables incrementally printing the function's progress. This also removes artificially induced delays for human readability."))
        .arg(arg!( -q --quiet "Makes the output suitable for analysis by other programs, removing the commas and label text. This option also invokes the 'p' flag."))
        .arg(arg!( -w --noweights "Disables using weights in the randomization process."))
        .arg(arg!( -t --notransforms "Disables using transforms in rule analysis."))
        .arg(arg!( -l --usewhitespace "Takes into account whitespace in the sample."))
        .get_matches();

    let width = *matches.get_one::<u32>("width").unwrap();
    let height = *matches.get_one::<u32>("height").unwrap();
    let tilesize = matches.get_one::<u16>("tilesize");
    let pathbuf = matches.get_one::<PathBuf>("sample");
    let mut print = !matches.get_flag("noprint");
    let use_weights = !matches.get_flag("noweights");
    let use_transforms = !matches.get_flag("notransforms");
    let simple_output = matches.get_flag("quiet");
    let max_contradictions = matches.get_one::<u32>("contradictions");
    let usewhitespace = matches.get_flag("usewhitespace");

    if simple_output {
        print = false;
    }

    let input: String = if let Some(buf) = pathbuf {
        fs::read_to_string(buf)
        .expect("The sample provided cannot be read or is invalid")
        .replace(", ", "")
        .replace(",", "")
    } else {
        include_str!("sample.txt").to_string()
        .replace(", ", "")
        .replace(",", "")
    };

    if input.is_empty() {
        panic!("The input sample cannot be empty")
    }

    // convert string input into a usable bitset-based sample
    let mut sample: Vec<Vec<u16>> = vec![]; 
    sample.reserve(input.lines().count());
    let mut source_map: Vec<(u16, char)> = vec![];
    let mut id_counter = 0u16;

    for (row, line) in input.lines().enumerate() {
        for (_, ch) in line.chars().enumerate() {
            if !usewhitespace {
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

    debug_assert_eq!(sample.len(), input.lines().count());

    let mut wave = Wave::new();
    wave.analyze(sample.clone(), if tilesize.is_some() { *tilesize.unwrap() } else { 2 });

    // let sample = Sample::<Vec<(char, Location)>>::chunkstr(input, 2);
    // let mut collapser = Collapser::new();
    // collapser.analyze(sample);

    // collapser.use_weights = use_weights;
    // collapser.use_transforms = use_transforms;

    // if let Some(max) = max_contras {
        // collapser.max_contradictions = *max;
    // }
    
    // let interval = std::time::Duration::from_secs_f32(0.05);
    // let mut output = Collapser::chunkstr_pipeline(&mut collapser, (width, height), print, interval)
        // .expect("There was an error during execution");
    
    // if simple_output {
        // println!("{}", output.0);
    // } else {
        // Parser::insert_commas(&mut output.0);
        // let rate = 1.0 / output.1 as f32 * 100.0;
        // println!("\x1b[1mFinal Output:\n{}\n\nSuccess Rate: {:.2}%", output.0, rate);
    // }
}
