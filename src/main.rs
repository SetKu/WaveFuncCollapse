// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
use wave_func_collapse::Coordinator;

// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation

fn main() {
    // S = Sea, C = Coast, L = Land
    let sample: String = include_str!("sample.txt").to_string();

    let mut coord = Coordinator::new();
    coord.process_sample(sample, true);
    coord.set_dimensions(5, 5);
    coord.populate_superpositions();
    
    let interval = std::time::Duration::new(0, 10_u32 * 10_u32.pow(7));
    match coord.collapse_all(true, interval) {
        Err(e) => println!("Found Error: {}", e),
        Ok(_) => println!("Final Output:\n\n{}", coord.get_rep(true)),
    }
}