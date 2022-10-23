// Modules in Rust: https://is.gd/gRhcqA
// Modules Cheat Sheet: https://is.gd/WusVq8
use wave_func_collapse::Coordinator;

// Cross Compilation Possibility: https://kerkour.com/rust-cross-compilation

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let sample: String = include_str!("sample.txt").to_string().replace(", ", "");
    let chars = sample.chars();
    let mut chars_dimensional: Vec<Vec<char>> = vec![vec![]];

    let mut row = 0_u32;
    for c in chars {
        if c.is_whitespace() {
            chars_dimensional.push(vec![]);
            row += 1;
            continue;
        }

        chars_dimensional[row as usize].push(c);
    }

    let mut coord = Coordinator::default();
    coord
        .create_rules(chars_dimensional, 2)
        .expect("Creating the rules for the sample failed.");
    coord.populate_superpositions(6, 6);

    // let mut w = 5_u32;
    // let mut h = 5_u32;
    // let mut t = false;
    // let mut d = true;
    // let mut wt = true;

    // // S = Sea, C = Coast, L = Land
    // let mut sample: String = include_str!("sample.txt").to_string();

    // // Flags:
    // for arg in args {
    //     if arg.starts_with("-w") {
    //         let res: u32 = arg.strip_prefix("-w").unwrap().parse().expect("The width provided was not a unsigned integer.");
    //         w = res;
    //     }

    //     if arg.starts_with("-h") {
    //         let res: u32 = arg.strip_prefix("-h").unwrap().parse().expect("The height provided was not a unsigned integer.");
    //         h = res;
    //     }

    //     if arg.starts_with("--transform") || arg.starts_with("-t") {
    //         t = true;
    //     }

    //     if arg.starts_with("-s:") {
    //         let path = arg.strip_prefix("-s:").expect("No file path was provided for the sample map.").to_string();
    //         sample = std::fs::read_to_string(path).expect("The file provided could not be read from.");
    //     }

    //     if arg.starts_with("-d") || arg.starts_with("--no-diagonals") {
    //         d = false;
    //     }

    //     if arg.starts_with("-t") || arg.starts_with("--no-weights") {
    //         wt = false;
    //     }
    // }

    // let mut coord = Coordinator::new();
    // coord.set_diagonals(d);
    // coord.set_use_weights(wt);
    // coord.process_sample(sample.clone(), t);
    // coord.set_dimensions(w, h);
    // coord.populate_superpositions();

    // let interval = std::time::Duration::new(0, 10_u32 * 10_u32.pow(7));
    // match coord.collapse_all(true, interval) {
    //     Err(e) => println!("Found Error: {}", e),
    //     Ok(_) => {
    //         println!("Input:\n\n{}", sample.replace(", ", " "));
    //         println!("Output:\n\n{}", coord.get_rep(true));
    //     },
    // }
}
