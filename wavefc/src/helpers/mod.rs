#[cfg(test)]
mod tests;

use crate::{Vector2, vec2};

/// This function finds the width and height of the given 2D array.
pub(crate) fn dimensions_of<T>(input: &Vec<Vec<T>>) -> Vector2<usize> {
    Vector2::new(
        input.len(),
        if input.len() > 0 { input[0].len() } else { 0 },
    )
}

pub(crate) struct Adjacency {
    pub(crate) root_data: Vec<Vec<u32>>,
    // Array holds values for top (0), right (1), bottom (2), and left (3).
    pub(crate) neighbours_data: [Vec<Vec<u32>>; 4],
}

// Overlapping
pub(crate) fn adjacencies(
    input: Vec<Vec<u32>>,
    chunk_size: Vector2<u32>,
    include_border_chunks: bool,
) -> Vec<Adjacency> {
    let size = dimensions_of(&input);
    let mut list: Vec<Adjacency> = vec![];

    for x in 0u32..(size.x - chunk_size.x) {
        for y in 0u32..(size.y - chunk_size.y) {
            let mut data = vec![]; // [x][y]

            for ix in 0..chunk_size.x {
                data.push(vec![]);

                for iy in 0..chunk_size.y {
                    data[ix].push(input[x + ix][y + iy]);
                }
            }

            let new_origins = [(-1, -1), (-1, 1), (1, -1), (1, 1)].into_iter().map(|pair| vec2(pair.0 * chunk_size.x + x, pair.1 * chunk_size.y + y));
        }
    }
    
    list
}

/// This function finds all possible rectangles of the specified `chunk_size` in the input, and then finds all its adjacencies rectangles (if they exist).
///
/// # Optionals
///
/// The neighbours in the `Adjacency` data provided might be `None`, because a full-sized rectangle (chunk) doesn't exist next to it in a particular direction.
pub fn overlapping_adjacencies<T>(
    input: Vec<Vec<T>>,
    chunk_size: Vector2<usize>,
    exclude_border_chunks: bool,
) -> Vec<Adjacency<T>>
where
    T: Clone,
{
    if chunk_size.x < 1 || chunk_size.y < 1 {
        return vec![];
    }

    let size = dimensions_of(&input);
    let size_indexed = size - Vector2::new(1, 1);
    let size_indexed_i = size_indexed.cast::<isize>().unwrap();
    let mut list: Vec<Adjacency<T>> = vec![];

    let mut chunk_points = vec![];

    for y in 0..chunk_size.y {
        for x in 0..chunk_size.x {
            chunk_points.push(Vector2::new(x, y));
        }
    }

    debug_assert_eq!(chunk_points.len(), chunk_size.x * chunk_size.y);

    for x in 0..size.x {
        for y in 0..size.y {
            let point = Vector2::new(x, y);
            let edge = point + chunk_size - Vector2::new(1, 1);

            // check if the chunk exists and is in bounds
            if edge.x > size_indexed.x || edge.y > size_indexed.y {
                continue;
            }

            let content = chunk_points
                .to_owned()
                .into_iter()
                .map(|v| (input[v.x + x][v.y + y].to_owned(), v))
                .collect();
            let arr = arrayify(content, &chunk_size);
            let mut adjacency = Adjacency::new(arr);

            // up (adding y), right (adding x), down (subtracting y), left (subtracting x)
            let adjac_origins = [
                Vector2 {
                    x: point.x as isize,
                    y: point.y as isize + chunk_size.y as isize,
                },
                Vector2 {
                    x: point.x as isize + chunk_size.x as isize,
                    y: point.y as isize,
                },
                Vector2 {
                    x: point.x as isize,
                    y: point.y as isize - chunk_size.y as isize,
                },
                Vector2 {
                    x: point.x as isize - chunk_size.x as isize,
                    y: point.y as isize,
                },
            ];

            let mut scrap_chunk = false;

            for i in 0..4 {
                let origin = adjac_origins[i];
                let chunk_size_i = chunk_size.cast::<isize>().unwrap();
                let org_edge = origin + chunk_size_i - Vector2::new(1, 1);

                // corner infringements
                let top_left_e = org_edge.x < chunk_size_i.x || org_edge.y < chunk_size_i.y;
                let bottom_right_o = origin.x > size_indexed_i.x - chunk_size_i.x
                    || origin.y > size_indexed_i.y - chunk_size_i.y;

                if top_left_e || bottom_right_o {
                    if exclude_border_chunks {
                        // don't include this chunk at all
                        scrap_chunk = true;
                        break;
                    }
                }

                let origin_invalid = origin.x < 0
                    || origin.y < 0
                    || origin.x > size_indexed_i.x
                    || origin.y > size_indexed_i.y;
                let edge_invalid = org_edge.x < 0
                    || org_edge.y < 0
                    || org_edge.x > size_indexed_i.x
                    || org_edge.y > size_indexed_i.y;

                if origin_invalid || edge_invalid {
                    continue;
                }

                debug_assert!(org_edge.x >= 0 && org_edge.y >= 0);

                // BorderMode::Clamp
                let content: Vec<(T, Vector2<usize>)> = chunk_points
                    .to_owned()
                    .into_iter()
                    .map(|v| {
                        (
                            input[v.x + origin.x as usize][v.y + origin.y as usize].to_owned(),
                            v,
                        )
                    })
                    .collect();
                let formatted = arrayify(content, &chunk_size);
                adjacency.neighbours_content[i] = Some(formatted);
            }

            if scrap_chunk {
                continue;
            }

            list.push(adjacency);
        }
    }

    list
}

// pub fn rotate_ninety<T>(mut input: Vec<Vec<T>>, count: usize) -> Vec<Vec<T>>
// where
// T: Clone,
// {
// for _ in 0..count {
// for i in 0..input.len() {
// for j in 0..i {
// let tmp = input[i][j].to_owned();
// input[i][j] = input[j][i].to_owned();
// input[j][i] = tmp;
// }
// }

// input.reverse();
// }

// input
// }

/// Note: Will panic if number of columns is zero as it uses unsafe indexing.
pub fn xy_swap<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut list = vec![];

    let rlen = input.len();
    let clen = input[0].len();

    for r in 0..rlen {
        for c in 0..clen {
            list.push((input[r][c].to_owned(), Vector2::new(c, r)));
        }
    }

    std::mem::drop(input);

    arrayify(list, &Vector2::new(clen, rlen))
}

// pub fn mirror<T>(mut input: Vec<T>) -> Vec<T> where T: Clone {
// let width = input.len();
// let half = width / 2;

// for x in 0..half {
// let opp_ind = width - x - 1;
// let tmp = input[opp_ind].to_owned();
// input[opp_ind] = input[x].to_owned();
// input[x] = tmp;
// }

// input
// }

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
