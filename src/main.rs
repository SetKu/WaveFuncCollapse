// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation
use wfc::{Collapser, Sample, collapse_all_str};

fn main() {
    let input: String = include_str!("sample.txt").to_string().replace(", ", "");
    let sample = Sample::<char>::from_str(input);
    let mut collapser = Collapser::new();
    collapser.analyze(sample);
    let inerval = std::time::Duration::from_secs_f32(0.1);
    let output = collapse_all_str(&mut collapser, (5, 5), true, inerval)
        .expect("There was an error during execution.");
    println!("Final Output:\n{}", output);
}
