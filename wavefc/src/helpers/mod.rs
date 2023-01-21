#[cfg(test)]
mod tests;

extern crate cgmath;
use cgmath::Vector2;

/// This function finds the width and height of the given 2D array.
pub fn dimensions_of<T>(input: &Vec<Vec<T>>) -> Vector2<usize> {
    Vector2::new(
        input.len(),
        if input.len() > 0 { input[0].len() } else { 0 },
    )
}

/// Shifts the given grid by the number of times specified along the different axis.
///
/// # Arguments
///
/// * `x_axis`: bool - whether it shifts along the x-axis (rows)
/// * `y_axis`: bool - whether it shifts along the y-axis (cols)
pub fn roll<T>(input: &mut Vec<Vec<T>>, shifts: usize, x_axis: bool, y_axis: bool)
where
    T: Clone,
{
    let input_size = dimensions_of(&input);

    for _ in 0..shifts {
        let input_copy = input.to_owned();

        for (i_r, row) in input.iter_mut().enumerate() {
            for (i_c, element) in row.iter_mut().enumerate() {
                let s_r: usize;
                let s_c: usize;

                // row
                if y_axis {
                    s_r = if i_r == 0 { input_size.y - 1 } else { i_r - 1 };
                } else {
                    s_r = i_r;
                }

                // column
                if x_axis {
                    s_c = if i_c == 0 { input_size.x - 1 } else { i_c - 1 };
                } else {
                    s_c = i_c;
                }

                let swap_element = input_copy[s_r][s_c].clone();
                *element = swap_element;
            }
        }
    }
}

/// Returns all the possible shifted possibilities of the given 2D array.
// pub fn all_possible_shifts<T>(input: Vec<Vec<T>>) -> Vec<Vec<Vec<T>>>
// where
// T: Clone,
// {
// if input.len() == 0 || input[0].len() == 0 {
// return vec![];
// }

// let mut sets: Vec<Vec<Vec<T>>> = vec![];
// let input_size = dimensions_of(&input);
// let shifts = std::cmp::max(input_size.x, input_size.y);

// for (x_axis, y_axis) in [(true, false), (false, true), (true, true)] {
// for shift_count in 0..shifts {
// let mut new_set = input.to_owned();

// for _ in 0..(shift_count + 1) {
// roll(&mut new_set, shifts.into(), x_axis, y_axis);
// }

// sets.push(new_set);
// }
// }

// sets
// }

/// Converts the given flat vector of `T` and `Vector2` pairs into a 2-dimensional vector of just `T`.
///
/// Debug-wise, this function will also assert that its input doesn't contain any duplicate `Vector2`s. This check only occurs in debug mode.
pub fn arrayify<T>(input: Vec<(T, Vector2<usize>)>, size: &Vector2<usize>) -> Vec<Vec<T>> {
    let mut formatted = vec![];

    for i in 0..size.x {
        formatted.push(vec![]);
        formatted[i as usize].reserve(size.y.into());
    }

    debug_assert_eq!(formatted.len() as usize, size.x);

    for element in input {
        let loc = element.1;

        // check for duplicate locations in input
        debug_assert_eq!(
            {
                formatted
                    .get(loc.x as usize)
                    .map(|row| row.get(loc.y as usize))
                    .unwrap_or(None)
                    .map(|_t| false)
            },
            None
        );

        formatted[loc.x as usize].push(element.0);
    }

    formatted
}

/// Various modes for analyzing adjacencies at the border of a 2d array.
#[derive(PartialEq)]
pub enum BorderMode {
    /// Don't include border chunks.
    Exclude,
    /// Include border chunks and their neighbours.
    Clamp,
    // /// Include border chunks and all their neighbours wrapping across the input.
    // Wrap,
}

/// Adjacency information and data about a given chunk.
#[derive(Debug)]
pub struct Adjacency<T> {
    pub origin_content: Vec<Vec<T>>,
    // array holds values for top, right, bottom, and left.
    pub neighbours_content: [Option<Vec<Vec<T>>>; 4],
}

impl<T> Adjacency<T> {
    pub fn new(origin: Vec<Vec<T>>) -> Self {
        Self {
            origin_content: origin,
            neighbours_content: [None, None, None, None],
        }
    }
}

// pub fn adjacencies<T>(
// input: Vec<Vec<T>>,
// chunk_size: Vector2<usize>,
// border_mode: BorderMode,
// ) -> Vec<Adjacency<T>>
// where
// T: Clone,
// {
// let size = dimensions_of(&input);

// for x in 0..size.x {
// for y in 0..size.y {
// let point = Vector2::new(x, y);
// let cast = point.cast::<isize>().unwrap();
// let mut adjacency = Adjacency::new(vec![vec![input[point.x][point.y].to_owned()]]);
// let neighbours = [
// cast - Vector2::new(0, 1),
// cast + Vector2::new(1, 0),
// cast + Vector2::new(0, 1),
// cast - Vector2::new(1, 0),
// ];

// for nb in neighbours {
// if nb.x > 0 && nb.y > 0 {}
// }
// }
// }

// vec![]
// }

/// Converts the 2-dimensional array into chunked, square patterns of the specified size.
///
/// # Arguments
///
/// If `allow_slims` is false, the function will panic if the input's size is not a factor of `n_size`.
// fn chunkify<T>(
// input: Vec<Vec<T>>,
// chunk_size: Vector2<usize>,
// allow_slims: bool,
// ) -> Vec<(Vec<Vec<T>>, Vector2<usize>)>
// where
// T: Clone,
// {
// if !allow_slims {
// assert!(input.len() >= chunk_size.x);
// assert!(input.len() % chunk_size.x == 0);
// assert!(input[0].len() >= chunk_size.y);
// assert!(input[0].len() % chunk_size.y == 0);
// }

// let mut chunks: Vec<(Vec<(T, Vector2<usize>)>, Vector2<usize>)> = vec![];

// for (i_x, row) in input.iter().enumerate() {
// for (i_y, element) in row.iter().enumerate() {
// let c_x = (i_x as f32 / chunk_size.x as f32).floor() as usize;
// let c_y = (i_y as f32 / chunk_size.y as f32).floor() as usize;
// let chunk = Vector2::new(c_x, c_y);

// let r_x = i_x % chunk_size.x;
// let r_y = i_y % chunk_size.y;
// let rel = Vector2::new(r_x, r_y);

// let content = (element.to_owned(), rel);

// if let Some(chunk) = chunks.iter_mut().find(|c| c.1 == chunk) {
// // i_c naturally increments up and thus rel.y doesn't need to be checked
// chunk.0.push(content);
// } else {
// let mut pattern = vec![];
// pattern.push(content);
// let new = (pattern, chunk);
// chunks.push(new);
// }
// }
// }

// let converted = chunks
// .into_iter()
// .map(|i| (arrayify(i.0, &chunk_size), i.1))
// .collect();

// converted
// }

/// This function finds all possible rectangles of the specified `chunk_size` in the input, and then finds all its adjacencies rectangles (if they exist).
///
/// # Optionals
///
/// The neighbours in the `Adjacency` data provided might be `None`, because a full-sized rectangle (chunk) doesn't exist next to it in a particular direction.
pub fn overlapping_adjacencies<T>(
    input: Vec<Vec<T>>,
    chunk_size: Vector2<usize>,
    border_mode: BorderMode,
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
                    if border_mode == BorderMode::Exclude {
                        // don't include this chunk at all
                        scrap_chunk = true;
                        break;
                    }

                    // if border_mode == BorderMode::Wrap {
                    // let mut content: Vec<(T, Vector2<usize>)> = vec![];

                    // for iy in 0..chunk_size.y {
                    // for ix in 0..chunk_size.x {
                    // let sum = origin + Vector2::new(ix as isize, iy as isize);
                    // let index: Vector2<usize>;

                    // if sum.x < 0 || sum.y < 0 {
                    // index = Vector2::new(
                    // size_indexed_i.x - sum.x,
                    // size_indexed_i.y - sum.y,
                    // )
                    // .cast::<usize>()
                    // .unwrap();
                    // } else if sum.x > size_indexed_i.x || sum.y > size_indexed_i.y {
                    // index = Vector2::new(
                    // sum.x - size_indexed_i.x,
                    // sum.y - size_indexed_i.y,
                    // )
                    // .cast::<usize>()
                    // .unwrap();
                    // } else {
                    // index = Vector2::new(ix, iy).cast::<usize>().unwrap();
                    // }

                    // content.push((input[index.x][index.y].to_owned(), index));
                    // }
                    // }

                    // debug_assert_eq!(content.len(), chunk_size.x * chunk_size.y);

                    // let formatted = arrayify(content, &chunk_size);
                    // adjacency.neighbours[i] = Some(formatted);

                    // continue;
                    // }
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
