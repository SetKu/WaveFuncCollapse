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
    input: &Vec<Vec<T>>,
    chunk_size: Vector2<usize>,
) -> Vec<Adjacency<T>>
where
    T: Clone,
{
    let size = dimensions_of(&input);

    let mut list = vec![];

    for y in 0..size.y {
        for x in 0..size.x {
            let point = Vector2::new(x, y);
            let edge_loc = point + chunk_size;
            let size_index = size - Vector2::new(1, 1);

            // check if chunk is available to select from this point
            if edge_loc.x > size_index.x && edge_loc.y > size_index.y {
                // out of bounds
                continue;
            }

            let mut chunk = vec![];

            for cy in point.y..=edge_loc.y {
                for cx in point.x..=edge_loc.x {
                    chunk.push(Vector2::new(cx, cy));
                }
            }

            // check edge calc was correct and didn't pick a slim
            debug_assert_eq!(chunk.len(), chunk_size.x * chunk_size.y);

            let content = chunk
                .into_iter()
                .map(|v| (input[v.x][v.y].to_owned(), v))
                .collect();
            let arr = arrayify(content, &chunk_size);
            let mut adjacency = Adjacency::new(arr);

            // top, right, bottom, left
            let adjac_origins = [
                Vector2 {
                    x: point.x,
                    y: point.y - chunk_size.y,
                },
                Vector2 {
                    x: point.x + chunk_size.x,
                    y: point.y,
                },
                Vector2 {
                    x: point.x,
                    y: point.y - chunk_size.y,
                },
                Vector2 {
                    x: point.x - chunk_size.x,
                    y: point.y,
                },
            ];

            for i in 0..4 {
                let origin = adjac_origins[i];
                let org_edge = origin + chunk_size;
                
                if edge_loc.x > size_index.x || edge_loc.y > size_index.y {
                    let mut content = vec![];

                    for cx in origin.x..=org_edge.x {
                        for cy in origin.y..=org_edge.y {
                            content.push((input[cx][cy].to_owned(), Vector2::new(cx, cy)));
                        }
                    }

                    // check edge calc was correct and didn't pick a slim
                    debug_assert_eq!(content.len(), chunk_size.x * chunk_size.y);

                    let formatted = arrayify(content, &chunk_size);
                    adjacency.neighbours[i] = Some(formatted);
                }
            }

            list.push(adjacency);
        }
    }

    list
}
