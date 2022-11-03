use std::cmp::Ordering;

// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation
use wfc::{Collapser, Sample};

fn main() {
    let input: String = include_str!("sample.txt").to_string().replace(", ", "");
    let sample = Sample::<char>::from_str(input);
    let mut collapser = Collapser::new();
    collapser.analyze(sample);
    let result = collapser.collapse_all((5, 5)).expect("Collapsing failed.");

    let mut organized = result.clone();
    organized.sort_by_key(|i| i.1.clone());
    
    let mut output = String::new();
    let mut line: i64 = 0;

    println!("{:#?}", organized);

    for (ch, loc) in organized {
        if line < loc.y as i64 {
            output.push('\n');
            line = loc.y as i64;
        }
        
        output.push(*ch);
    }

    println!("{}", output);
}
