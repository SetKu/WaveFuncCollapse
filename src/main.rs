// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation
use wave_func_collapse::location::*;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let sample: String = include_str!("sample.txt").to_string().replace(", ", "");
    let chars = sample.chars();
    let mut chars_dimensional: Vec<Vec<char>> = vec![vec![]];

    // let mut row = 0_u32;
    // for c in chars {
    // if c.is_whitespace() {
    // chars_dimensional.push(vec![]);
    // row += 1;
    // continue;
    // }

    // chars_dimensional[row as usize].push(c);
    // }

    // let mut coord = Coordinator::default();
    // coord
    // .create_rules(chars_dimensional, 2)
    // .expect("Creating the rules for the sample failed.");
    // coord.collapse_all(6, 6);
}
