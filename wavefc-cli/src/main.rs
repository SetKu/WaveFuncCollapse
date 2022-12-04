// declare dependencies
extern crate clap;
extern crate wavefc;

use clap::{arg, crate_version, value_parser, Arg, Command};
use std::path::PathBuf;

mod string_mode;
use string_mode::string_mode;

fn main() -> Result<(), String> {
    let matches = Command::new("Wave Function Collapse")
        .version(crate_version!())
        .subcommand(Command::new("string")
            .about("Creates a new string output from a given character map. By default, it uses a template sample.")
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
            .arg(arg!( -j --tilewidth <number> "Specify the tile size width (precedent over --tilesize)." )
                .value_parser(value_parser!(usize)))
            .arg(arg!( -k --tileheight <number> "Specify the tile size height (precedent over --tilesize)." )
                .value_parser(value_parser!(usize)))
            .arg(arg!( -a --attempts <number> "The maximum number of contradictions (attempts) that can be reached before the program quits.")
                .value_parser(value_parser!(usize)))
            .arg(arg!( -p --noprint "Disables incrementally printing the function's progress."))
            .arg(arg!( -w --noweights "Disables using weights in when calculating superposition entropy."))
            .arg(arg!( -t --notransforms "Disables using transforms in rule analysis."))
            .arg(arg!( -l --whitespace "Takes into account whitespace in the sample."))
            .arg(arg!( -d --disablecom "Disables stripping commas from the input sample."))
        )
        .get_matches();

    match matches.subcommand().expect("No command was provided.") {
        ("string", sub_matches) => string_mode(sub_matches)?,
        _ => println!("Unknown command."),
    };

    Ok(())
}
