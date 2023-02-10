#![warn(clippy::pedantic, clippy::nursery)]

#[cfg(test)]
mod tests;

use cgmath::{vec2, Vector2, Zero};
use std::collections::HashMap;

/// This function finds the width and height of the given 2D array.
///
/// Assumes constant uniform internal vector length.
pub fn dimensions_of<T>(input: &[Vec<T>]) -> Vector2<usize> {
    Vector2::new(
        input.len(),
        if input.is_empty() { 0 } else { input[0].len() },
    )
}

#[derive(Debug)]
pub struct Adjacency {
    pub root_data: Vec<Vec<u32>>,
    // Array holds values for top (0), right (1), bottom (2), and left (3).
    pub neighbours_data: [Option<Vec<Vec<u32>>>; 4],
}

// Overlapping
pub fn adjacencies(
    input: &[Vec<u32>],
    chunk_size: Vector2<u32>,
    up_is_greater: bool,
) -> Vec<Adjacency> {
    if chunk_size.x.is_zero() || chunk_size.y.is_zero() {
        return vec![];
    }

    let size = dimensions_of(input);
    let mut list: Vec<Adjacency> = vec![];

    // Avoid calculating for a root that doesn't exist.
    for x in 0..=(size.x - chunk_size.x as usize) {
        for y in 0..=(size.y - chunk_size.y as usize) {
            let mut root = vec![]; // [x][y]

            for ix in 0..(chunk_size.x as usize) {
                root.push(vec![]);

                for iy in 0..(chunk_size.y as usize) {
                    root[ix].push(input[x + ix][y + iy]);
                }
            }

            let new_origins = [
                (0_i32, if up_is_greater { 1 } else { -1 }),
                (1, 0),
                (0, if up_is_greater { -1 } else { 1 }),
                (-1, 0),
            ]
            .into_iter()
            .map(|pair| {
                chunk_size.cast::<i32>().map(|cast| {
                    // Wrapping and truncation aren't a problem here as both are less than a u32::MAX - 1.
                    // In the event either occur, they will be flipped negative and not checked when
                    // indexing, regardless.
                    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                    vec2(pair.0 * cast.x + x as i32, pair.1 * cast.y + y as i32)
                })
            });

            let mut neighbours = [None, None, None, None];

            for (i, origin) in new_origins.enumerate() {
                if origin.is_none() {
                    continue;
                }

                let origin = origin.unwrap();

                // Check if the origin or its endpoint is out of bounds.
                if origin.x < 0 || origin.y < 0 {
                    continue;
                }

                // If the chunk size wraps, there won't be a problem as
                // we should continue anyways if so.
                #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
                if origin.x + chunk_size.x as i32 > size.x as i32
                    || origin.y + chunk_size.y as i32 > size.y as i32
                {
                    continue;
                }

                let mut data = vec![];

                for ix in 0..(chunk_size.x as usize) {
                    data.push(vec![]);

                    for iy in 0..(chunk_size.y as usize) {
                        // Values are guaranteed not to be negative.
                        #[allow(clippy::cast_sign_loss)]
                        data[ix].push(input[origin.x as usize + ix][origin.y as usize + iy]);
                    }
                }

                neighbours[i] = Some(data);
            }

            let formed = Adjacency {
                root_data: root,
                neighbours_data: neighbours,
            };

            list.push(formed);
        }
    }

    list
}

/// Returns in the order top (0), right, bottom, left. Assumes top has an incremented y-value.
pub const fn orthogonal(o: Vector2<i32>) -> [Vector2<i32>; 4] {
    [
        Vector2::new(o.x, o.y + 1),
        Vector2::new(o.x + 1, o.y),
        Vector2::new(o.x, o.y - 1),
        Vector2::new(o.x - 1, o.y),
    ]
}

pub fn remove_indexes<T>(vec: &mut Vec<T>, indexes: &[usize]) {
    for (removed, i) in indexes.iter().enumerate() {
        vec.remove(i - removed);
    }
}

#[derive(Debug)]
pub struct Sample<T> {
    grid: Vec<Vec<u32>>,
    map: HashMap<u32, T>,
}

pub fn deconstruct_string(input: &str, use_whitespace: bool) -> Sample<char> {
    let mut sample = Sample {
        grid: Vec::with_capacity(input.lines().count()),
        map: HashMap::new(),
    };

    let mut id_counter = 0;

    for (y, text) in input.lines().enumerate() {
        let chars = text.chars();

        for (x, ch) in if use_whitespace {
            chars.enumerate().collect::<Vec<(usize, char)>>()
        } else {
            chars.filter(|v| !v.is_whitespace()).enumerate().collect()
        } {
            dbg!(x);

            if sample.grid.len() < x + 1 {
                // Make space for at least the current y-value.
                // Saves time.
                sample.grid.push(Vec::with_capacity(y));
                println!("added new col");
            }

            let translation: u32 = sample
                .map
                .iter()
                .find_map(|(k, v)| if *v == ch { Some(*k) } else { None })
                .unwrap_or_else(|| {
                    let new_id = id_counter;
                    sample.map.insert(new_id, ch);
                    id_counter += 1;
                    new_id
                });

            sample.grid[x].push(translation);
        }
    }

    sample
}
