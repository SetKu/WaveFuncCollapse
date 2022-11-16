pub mod helpers;
use helpers::*;
use cgmath::Vector2;
use std::clone::Clone;

/// Flags for the `Wave`.
pub enum Flags {
    NoWeights = 1,
    NoRotations,
    NoOverlap,
}

/// Various modes for analyzing tiles at the border of the `Wave` sample.
pub enum BorderMode {
    /// Don't include border patterns.
    Exclude = 1,
    /// Include border patterns and their neighbours.
    Clamp,
    /// Include border patterns and all their neighbours wrapping across the input.
    Wrap,
}

/// Encapsulation for the Wave Function Collapse implementation.
pub struct Wave { flags: Vec<Flags>, border_mode: BorderMode,
    patterns: Vec<Pattern>,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            flags: vec![],
            border_mode: BorderMode::Clamp,
            patterns: vec![],
        }
    }

    pub fn analyze(&mut self, input: Vec<Vec<u16>>, n_size: u16) {
        // let overlapped_chunks = overlapping_chunks(input, n_size);

        // let mut chunks = chunkify(input, n_size, false);

        // let update_metas = |chunks: &mut Vec<(Pattern, Vector2<u16>)>| {
            // // # update pattern frequencies and entropies
            // // copy is used to scan for the frequency of certain patterns
            // let chunks_copy = chunks.clone();

            // for (pattern_1, loc_1) in chunks.iter_mut() {
                // for (pattern_2, loc_2) in &chunks_copy {
                    // if loc_1 == loc_2 {
                        // continue;
                    // } 

                    // if pattern_1.contents == pattern_2.contents {
                        // pattern_1.count += 1;
                    // } 
                // }
            // }

            // // drop the copy early instead of waiting for scope exit
            // std::mem::drop(chunks_copy);

            // let total = chunks.len() as f32;

            // for (pattern, _) in chunks.iter_mut() {
                // let frequency = pattern.count as f32 / total;
                // // the entropy of a pattern represents the "surprise" of having it occur
                // // the higher the entropy, the greater the surprise, the more shocking the result.
                // // equation formatted: https://is.gd/VXt4p3
                // pattern.entropy = frequency * (1.0 / frequency).log2();
            // }
        // };

        // update_metas(&mut chunks);

        // // # find rules
        // for (pattern, loc) in &chunks {
             
        // }
    }
}

fn dimensions_of<T>(input: &Vec<Vec<T>>) -> (usize, usize) {
    (input.len(), if input.len() > 0 { input[0].len() } else { 0 })
}

fn roll<T>(input: &mut Vec<Vec<T>>, shifts: usize, x_axis: bool, y_axis: bool) where T: Clone {
    let input_size = dimensions_of(&input);

    for _ in 0..shifts {
        let input_copy = input.to_owned();
        
        for (i_r, row) in input.iter_mut().enumerate() {
            for (i_c, element) in row.iter_mut().enumerate() {
                let s_r: usize;
                let s_c: usize;

                // row
                if y_axis {
                    s_r = if i_r == 0 { input_size.0 - 1 } else { i_r - 1 };
                } else {
                    s_r = i_r;
                }

                // column
                if x_axis {
                    s_c = if i_c == 0 { input_size.1 - 1 } else { i_c - 1 };
                } else {
                    s_c = i_c;
                }

                let swap_element = input_copy[s_r][s_c].clone(); 
                *element = swap_element;
            }
        }
    }
}

/// Converts the 2-dimensional array into chunked, square patterns of the specified size.
///
/// # Arguments
///
/// If `allow_slims` is true, the function will panic if the input's size is not a factor of `n_size`.
fn chunkify(input: Vec<Vec<u16>>, n_size: u16, allow_slims: bool) -> Vec<(Pattern, Vector2<u16>)> {
    if !allow_slims {
        assert!(input.len() >= n_size as usize);
    }

    let mut chunks: Vec<(Pattern, Vector2<u16>)> = vec![];

    for (i_r, row) in input.iter().enumerate() {
        for (i_c, ch) in row.iter().enumerate() {
            let c_x = (i_r as f32 / n_size as f32).floor() as u16;
            let c_y = (i_c as f32 / n_size as f32).floor() as u16;
            let chunk = Vector2::new(c_x, c_y);

            let r_x = i_r as u16 % n_size;
            let r_y = i_c as u16 % n_size;
            let rel = Vector2::new(r_x, r_y);

            if let Some(chunk) = chunks.iter_mut().find(|c| c.1 == chunk) {
                // i_c naturally increments up and thus rel.y doesn't need to be checked
                chunk.0.contents[rel.x as usize].push(*ch);
            } else {
                let mut pattern = Pattern::empty(n_size);
                pattern.contents[rel.x as usize].push(*ch);
                let new = (pattern, chunk);
                chunks.push(new);
            }
        }
    }

    if !allow_slims && chunks.len() > 0 {
        // check the number of chunks is equal to the input's width * height / the number
        // of elements that should be associated with the chunk size.
        assert_eq!(chunks.len(), input.len() * input[0].len() / (n_size * n_size) as usize);
    }

    chunks
}

#[derive(Debug, Clone)]
struct Pattern {
    count: u16,
    // entropy can and should be optimized in the future to refer to a shared value.
    // the reason for this is simply that a f32 is larger than a usize (u8) and 
    // is going to be identical to other identical patterns.
    // this could be achieved using a `Rc` or ARC based reference-counter.
    entropy: f32,
    contents: Vec<Vec<u16>>,
}

impl Pattern {
    fn new(contents: Vec<Vec<u16>>) -> Self {
        Pattern {
            count: 1,
            entropy: 0.0,
            contents,
        }
    }

    fn empty(size: u16) -> Self {
        let mut contents = vec![];

        for _ in 0..size {
            let mut inner = vec![];
            inner.reserve(size.into());
            contents.push(inner);
        }

        Self::new(contents)
    }
}

struct Element {
    values: Vec<Pattern>,
    // shannon entropy: https://youtu.be/-Rb868HKCo8
    entropy: f32,
    position: Vector2<u16>,
}
