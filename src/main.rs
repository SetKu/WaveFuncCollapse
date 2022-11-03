use std::cmp::Ordering;

// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation
use wfc::{Collapser, Sample, Parser};

fn main() {
    let input: String = include_str!("sample.txt").to_string().replace(", ", "");
    let sample = Sample::<char>::from_str(input);
    let mut collapser = Collapser::new();
    collapser.analyze(sample);
    let result = collapser.collapse_all((5, 5)).expect("Collapsing failed.");
    let parsed = Parser::parse(result);
}
