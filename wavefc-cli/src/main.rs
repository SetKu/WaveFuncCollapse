// declare dependencies
extern crate clap;
extern crate wavefc;

use clap::{arg, crate_version, value_parser, Arg, Command};
use std::path::PathBuf;

mod shared;
use shared::expand_shared_args;

mod image_process;
mod string_process;
use image_process::handler as image_mode;
use string_process::handler as string_mode;

const DEFAULT_MAX_CONTRADICTIONS: usize = 20;

fn main() -> Result<(), String> {
    let matches = Command::new("Wave Function Collapse")
        .version(crate_version!())
        .subcommand(expand_shared_args!(
                Command::new("string")
                    .about("Creates a new string output from a given character map. By default, it uses a template sample.")
                    .arg(arg!( -s --sample <file> "Use a custom sample file instead of the default sea, land, coast example." )
                        .value_parser(value_parser!(PathBuf)))
                    .arg(arg!( -p --noprint "Disables incrementally printing the function's progress."))
                    .arg(arg!( -l --whitespace "Takes into account whitespace in the sample."))
                    .arg(arg!( -d --disablecom "Disables stripping commas from the input sample."))
                )
        )
        .subcommand(expand_shared_args!(
                Command::new("image")
                    .about("Creates a new image based on the one provided.")
                    .arg(Arg::new("sample")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)))
                    .arg(arg!( -o --output <file> "The path to output the final image to." )
                        .value_parser(value_parser!(PathBuf)))
                    .arg(arg!( -O --open "Opens the output image in the default system application." ))
                )
        )
        .get_matches();

    match matches.subcommand().expect("No command was provided.") {
        ("string", sub_matches) => string_mode(sub_matches)?,
        ("image", sub_matches) => image_mode(sub_matches)?,
        _ => println!("Unknown command."),
    };

    Ok(())
}
