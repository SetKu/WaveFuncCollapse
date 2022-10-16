// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
use wave_func_collapse::Coordinator;

fn main() {
    // S = Sea, C = Coast, L = Land
    let sample = "\
    S, S, S, S, S
    C, S, C, S, S
    L, C, L, C, C
    L, L, L, L, L
    L, L, L, L, L".to_string().replace(", ", "");

    let mut coord = Coordinator::new();
    coord.process_sample(&sample);
}