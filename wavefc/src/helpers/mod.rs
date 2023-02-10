#![warn(clippy::pedantic, clippy::nursery)]
#![allow(unused)]

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

/// Returns all the possible overlapping adjacent tiles in the input grid. The chunk size determines the dimension of each tile.
///
/// To determine whether higher y values are considered up or down, use the `up_is_greater` flag.
///
/// If you want to eliminate tiles that are on the border of the input, simply filter out the tiles which are missing a neighbour.
///
/// Note: The behaviour of the function is undefined for a grid that's columns have varying length. In most events, this will cause a panic.
pub fn adjacencies(
    input: &[Vec<u32>],
    chunk_size: Vector2<u32>,
    up_is_greater: bool,
) -> Vec<Adjacency> {
    // Avoid panics with zeroed chunk sizes.
    if chunk_size.x.is_zero() || chunk_size.y.is_zero() {
        return vec![];
    }

    let size = dimensions_of(input);
    let mut list: Vec<Adjacency> = vec![];

    // Avoid calculating for a root that doesn't exist by excluding the edge for the chunk size.
    for x in 0..=(size.x - chunk_size.x as usize) {
        for y in 0..=(size.y - chunk_size.y as usize) {
            let mut root = vec![]; // [x][y]

            // Iterate through root data indexes, collecting their data for the adjacency.
            for ix in 0..(chunk_size.x as usize) {
                root.push(vec![]);

                for iy in 0..(chunk_size.y as usize) {
                    root[ix].push(input[x + ix][y + iy]);
                }
            }

            // Compile a list of possible origins for neighbour tiles from the root.
            let new_origins = [
                (0_i32, if up_is_greater { 1 } else { -1 }),
                (1, 0),
                (0, if up_is_greater { -1 } else { 1 }),
                (-1, 0),
            ]
            .into_iter()
            .map(|pair| {
                // Mapping here accounts for if the chunk size cast fails, for whatever reason.
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
                // Skip invalidated origins.
                if origin.is_none() {
                    continue;
                }

                let origin = origin.unwrap();

                // Check if the origin or its endpoint is out of bounds, making the tile invalid.
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

                // Collect data for adjacent tile like we did the root.
                for ix in 0..(chunk_size.x as usize) {
                    data.push(vec![]);

                    for iy in 0..(chunk_size.y as usize) {
                        // Values are guaranteed not to be negative.
                        #[allow(clippy::cast_sign_loss)]
                        data[ix].push(input[origin.x as usize + ix][origin.y as usize + iy]);
                    }
                }

                // Update the neighbour data in the index we're keeping.
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

/// Converts a given grid-like string into an xy-oriented ([x][y]) vector of bits along with a map relating these bits to their original characters.
pub fn deconstruct_string(input: &str, use_whitespace: bool) -> Sample<char> {
    let mut sample = Sample {
        grid: Vec::with_capacity(input.lines().count()),
        map: HashMap::new(),
    };

    let mut id_counter = 0;

    for (y, text) in input.lines().enumerate() {
        let chars = text.chars();

        // Since enumeration is used below, simply skipping whitespace in the loop would screw up the count.
        // To avoid this, we filter the list of whitespace, if requested, before iterating.
        let iterator = if use_whitespace {
            chars.enumerate().collect::<Vec<(usize, char)>>()
        } else {
            chars.filter(|v| !v.is_whitespace()).enumerate().collect()
        };

        for (x, ch) in iterator {
            if sample.grid.len() < x + 1 {
                // Make space for at least the current y-value.
                // Saves time.
                sample.grid.push(Vec::with_capacity(y));
            }

            // Translate the character into a bit, whether one exists for it or not.
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
