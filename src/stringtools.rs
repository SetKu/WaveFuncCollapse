use crate::helpers::xy_swap;

pub fn deconstruct_string(
    input: &String,
    use_whitespace: bool,
) -> (Vec<Vec<usize>>, Vec<(usize, char)>) {
    // convert string input into a usable bitset-based sample
    let mut sample: Vec<Vec<usize>> = vec![];
    sample.reserve(input.lines().count());
    let mut source_map: Vec<(usize, char)> = vec![];
    let mut id_counter = 0usize;

    for (row, line) in input.lines().enumerate() {
        if sample.len() < row + 1 {
            sample.push(vec![]);
        }

        for (_, ch) in line.chars().enumerate() {
            if !use_whitespace {
                if ch.is_whitespace() {
                    continue;
                }
            }

            if let Some(translation) = source_map.iter().find(|t| t.1 == ch) {
                sample[row].push(translation.0);
            } else {
                source_map.push((id_counter, ch));
                sample[row].push(id_counter);
                id_counter += 1;
            }
        }
    }

    (xy_swap(sample), source_map)
}

pub fn construct_wip_string(input: Vec<Vec<Vec<usize>>>, source_map: &Vec<(usize, char)>) -> String {
    let space_for_unfounds = true;

    let swapped = xy_swap(input);
    let mut output = "".to_string();
    let mut lines_added = 0;

    let mut max_vals_in_pos = 0;

    for row in &swapped {
        for col in row {
            if col.len() > max_vals_in_pos {
                max_vals_in_pos = col.len();
            }
        }
    }

    for (r, row) in swapped.iter().enumerate() {
        if lines_added < r + 1 {
            output.push('\n');
            lines_added += 1;
        }

        for vals in row {
            let mut mapped: Vec<char> = vals
                .iter()
                .map(|v| source_map.iter().find(|s| s.0 == *v).unwrap().1)
                .collect();
            mapped.sort();

            let mut string = "(".to_string();

            for i in 0..max_vals_in_pos {
                if let Some(ch) = mapped.get(i) {
                    string.push_str(&format!("{}", ch));
                } else if space_for_unfounds {
                    string.push(' ');
                }
            }

            string.push(')');
            output.push_str(&string);
        }
    }

    output
}

pub fn reconstruct_string(
    input: Vec<Vec<usize>>,
    source_map: &Vec<(usize, char)>,
    use_color: bool,
    bold: bool,
) -> String {
    let swapped = xy_swap(input);
    let mut output = "".to_string();

    if bold {
        output.push_str("\x1b[1m");
    }

    let mut lines = 1;

    for (r, row) in swapped.iter().enumerate() {
        if lines < r + 1 {
            output.push('\n');
            lines += 1;
        }

        for id in row {
            let real_val = source_map.iter().find(|s| s.0 == *id).unwrap().1;

            if use_color {
                output.push_str("\x1b[");

                if real_val == 'S' {
                    output.push_str("34m");
                } else if real_val == 'C' {
                    output.push_str("33m");
                } else if real_val == 'L' {
                    output.push_str("32m");
                } else {
                    output.push_str("35m");
                }
            }

            output.push_str(&format!("{}, ", real_val));
        }
    }

    if bold || use_color {
        // reset terminal style
        output.push_str("\x1b[0m");
    }

    output
}
