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
pub fn all_possible_shifts<T>(input: Vec<Vec<T>>) -> Vec<Vec<Vec<T>>>
where
    T: Clone,
{
    if input.len() == 0 || input[0].len() == 0 {
        return vec![];
    }

    let mut sets: Vec<Vec<Vec<T>>> = vec![];
    let input_size = dimensions_of(&input);
    let shifts = std::cmp::max(input_size.x, input_size.y);

    for (x_axis, y_axis) in [(true, false), (false, true), (true, true)] {
        for shift_count in 0..shifts {
            let mut new_set = input.to_owned();

            for _ in 0..(shift_count + 1) {
                roll(&mut new_set, shifts.into(), x_axis, y_axis);
            }

            sets.push(new_set);
        }
    }

    sets
}

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
    /// Include border chunks and all their neighbours wrapping across the input.
    Wrap,
}

/// Adjacency information and data about a given chunk.
#[derive(Debug)]
pub struct Adjacency<T> {
    origin: Vec<Vec<T>>,
    // array holds values for top, right, bottom, and left.
    neighbours: [Option<Vec<Vec<T>>>; 4],
}

impl<T> Adjacency<T> {
    pub fn new(origin: Vec<Vec<T>>) -> Self {
        Self {
            origin,
            neighbours: [None, None, None, None],
        }
    }
}

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

    for y in 0..size.y {
        for x in 0..size.x {
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

            // top, right, bottom, left
            let adjac_origins = [
                Vector2 {
                    x: point.x as isize,
                    y: point.y as isize - chunk_size.y as isize,
                },
                Vector2 {
                    x: point.x as isize + chunk_size.x as isize,
                    y: point.y as isize,
                },
                Vector2 {
                    x: point.x as isize,
                    y: point.y as isize + chunk_size.y as isize,
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

                    if border_mode == BorderMode::Wrap {
                        let mut content: Vec<(T, Vector2<usize>)> = vec![];

                        for iy in 0..chunk_size.y {
                            for ix in 0..chunk_size.x {
                                let sum = origin + Vector2::new(ix as isize, iy as isize);
                                let index: Vector2<usize>;

                                if sum.x < 0 || sum.y < 0 {
                                    index = Vector2::new(
                                        size_indexed_i.x - sum.x,
                                        size_indexed_i.y - sum.y,
                                    )
                                    .cast::<usize>()
                                    .unwrap();
                                } else if sum.x > size_indexed_i.x || sum.y > size_indexed_i.y {
                                    index = Vector2::new(
                                        sum.x - size_indexed_i.x,
                                        sum.y - size_indexed_i.y,
                                    )
                                    .cast::<usize>()
                                    .unwrap();
                                } else {
                                    index = Vector2::new(ix, iy).cast::<usize>().unwrap();
                                }

                                content.push((input[index.x][index.y].to_owned(), index));
                            }
                        }

                        debug_assert_eq!(content.len(), chunk_size.x * chunk_size.y);

                        let formatted = arrayify(content, &chunk_size);
                        adjacency.neighbours[i] = Some(formatted);

                        continue;
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
                adjacency.neighbours[i] = Some(formatted);
            }

            if scrap_chunk {
                continue;
            }

            list.push(adjacency);
        }
    }

    list
}

pub fn swap_layers<T>(input: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut new: Vec<Vec<T>> = vec![];

    for ci1 in 0..input[0].len() {
        new.push(vec![]);

        for r in &input {
            for (ci2, c) in r.iter().enumerate() {
                if ci2 == ci1 {
                    new[ci1].push(c.to_owned());
                }
            }
        }
    }

    new
}
