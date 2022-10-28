// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation
use wfc::{Collapser, Sample};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let input: String = include_str!("sample.txt").to_string().replace(", ", "");
    let sample = Sample::<char>::from_str(input);
}
