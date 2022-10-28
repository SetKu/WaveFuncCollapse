// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation
use wave_func_collapse::Collapser;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sample: String = include_str!("sample.txt").to_string().replace(", ", "");
}
