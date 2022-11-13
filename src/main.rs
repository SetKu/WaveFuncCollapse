// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation
use wfc::{Collapser, Sample, collapse_all_str, Parser};
use clap::{arg, crate_version, value_parser, Command, Arg};
use std::path::PathBuf;
use std::fs;

fn main() {
    let matches = Command::new("Wave Function Collapse")
        .version(crate_version!())
        .arg(
            Arg::new("width")
            .required(true)
            .value_parser(value_parser!(u32))
        )
        .arg(
            Arg::new("height")
            .required(true)
            .value_parser(value_parser!(u32))
        )
        .arg(
            Arg::new("sample")
            .short('s')
            .long("sample")
            .value_name("file")
            .long_help("Use a custom sample file instead of the default sea, land, coast example.")
            .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(
                -c --contradictions <number> "The maximum number of contradictions (attempts) that can be reached before the program panics."
            )
            .value_parser(value_parser!(u32))
        )
        .arg(
            arg!(
                -p --noprint "(Fast Mode 🚀) Disables incrementally printing the function's progress. This also removes artificially induced delays for human readability."
            )
        )
        .arg(
            arg!(
                -q --quiet "Makes the output suitable for analysis by other programs, removing the commas and label text. This option also invokes the 'p' flag."
            )
        )
        .arg(
            arg!(
                -w --noweights "Disables using weights in the randomization process."
            )
        )
        .arg(
            arg!(
                -t --notransforms "Disables using transforms in rule analysis."
            )
        )
        .get_matches();

    let width = *matches.get_one::<u32>("width").unwrap();
    let height = *matches.get_one::<u32>("height").unwrap();
    let pathbuf = matches.get_one::<PathBuf>("sample");
    let mut print = !matches.get_flag("noprint");
    let use_weights = !matches.get_flag("noweights");
    let use_transforms = !matches.get_flag("notransforms");
    let simple_output = matches.get_flag("quiet");
    let max_contras = matches.get_one::<u32>("contradictions");

    if simple_output {
        print = false;
    }

    let input: String = if let Some(buf) = pathbuf {
        fs::read_to_string(buf)
        .expect("The sample provided cannot be read or is invalid")
        .replace(", ", "")
    } else {
        include_str!("sample.txt").to_string().replace(", ", "")
    };

    if input.is_empty() {
        panic!("The input sample cannot be empty!")
    }

    let sample = Sample::<char>::new_str(input);
    let mut collapser = Collapser::new();
    collapser.analyze(sample);

    collapser.use_weights = use_weights;
    collapser.use_transforms = use_transforms;

    if let Some(max) = max_contras {
        collapser.max_contradictions = *max;
    }
    
    let interval = std::time::Duration::from_secs_f32(0.05);
    let mut output = collapse_all_str(&mut collapser, (width, height), print, interval)
        .expect("There was an error during execution");
    
    if simple_output {
        println!("{}", output.0);
    } else {
        Parser::insert_commas(&mut output.0);
        let rate = 1.0 / output.1 as f32 * 100.0;
        println!("\x1b[1mFinal Output:\n{}\n\nSuccess Rate: {:.2}%", output.0, rate);
    }
}
