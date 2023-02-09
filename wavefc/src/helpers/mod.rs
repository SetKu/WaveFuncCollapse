#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo,
)]

#[cfg(test)]
mod tests;

use cgmath::{vec2, Vector2, Zero};

/// This function finds the width and height of the given 2D array.
///
/// Assumes constant uniform internal vector length.
pub(crate) fn dimensions_of<T>(input: &[Vec<T>]) -> Vector2<usize> {
    Vector2::new(
        input.len(),
        if input.is_empty() { 0 } else { input[0].len() },
    )
}

#[derive(Debug)]
pub(crate) struct Adjacency {
    pub(crate) root_data: Vec<Vec<u32>>,
    // Array holds values for top (0), right (1), bottom (2), and left (3).
    pub(crate) neighbours_data: [Option<Vec<Vec<u32>>>; 4],
}

// Overlapping
pub(crate) fn adjacencies(
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
    for x in 0..(size.x - chunk_size.x as usize + 1) {
        for y in 0..(size.y - chunk_size.y as usize + 1) {
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

pub fn noneg_neighbours(origin: &Vector2<usize>) -> Vec<Vector2<usize>> {
    let cast = origin.cast::<isize>().unwrap();
    let val = vec![
        Vector2::new(cast.x, cast.y - 1),
        Vector2::new(cast.x + 1, cast.y),
        Vector2::new(cast.x, cast.y + 1),
        Vector2::new(cast.x - 1, cast.y),
    ];

    val.into_iter()
        .filter(|v| v.x >= 0 && v.y >= 0)
        .map(|v| v.cast::<usize>().unwrap())
        .collect()
}

pub fn remove_indexes<T>(vec: &mut Vec<T>, indexes: Vec<usize>) {
    let mut removed = 0usize;

    for i in indexes {
        vec.remove(i - removed);
        removed += 1;
    }
}

pub fn orthog_direction(origin: &Vector2<usize>, point: &Vector2<usize>) -> u8 {
    let diff = point.cast::<isize>().unwrap() - origin.cast::<isize>().unwrap();

    if diff.x < 0 {
        3
    } else if diff.x > 0 {
        1
    } else if diff.y < 0 {
        2
    } else if diff.y > 0 {
        0
    } else {
        0
    }
}
