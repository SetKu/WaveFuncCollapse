pub mod helpers;
pub use helpers::BorderMode;
use helpers::*;
use cgmath::Vector2;
use std::clone::Clone;
use std::rc::Rc;

/// Flags for the `Wave`.
#[derive(PartialEq)]
pub enum Flags {
    NoWeights = 1,
    NoTransforms,
}

/// Encapsulation for the Wave Function Collapse implementation.
pub struct Wave {
    flags: Vec<Flags>,
    patterns: Vec<Pattern>,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            flags: vec![],
            patterns: vec![],
        }
    }

    pub fn analyze(
        &mut self,
        input: Vec<Vec<usize>>,
        chunk_size: Vector2<usize>,
        border_mode: BorderMode,
    ) {
        let adjacencies = overlapping_adjacencies(input.to_owned(), chunk_size, border_mode);

        let mut patterns = vec![];
        patterns.reserve(adjacencies.len());
        let mut id_counter = 0usize;

        for adjacency in adjacencies {
            let mut pattern = Pattern::new(id_counter, adjacency.origin.to_owned());
            id_counter += 1;

            for (i, neighbour) in adjacency.neighbours.into_iter().enumerate() {
                if let Some(unwrapped) = neighbour {
                    let rule = Rule::new(i as u8, unwrapped);
                    pattern.rules.push(rule);
                }
            }

            patterns.push(pattern);
        }

        dedup_patterns(&mut patterns);

        if !self.flags.contains(&Flags::NoTransforms) {
            let mut new_patterns: Vec<Pattern> = vec![];

            // transform time!
            for pattern in patterns.iter() {
                let mut mirrored_x = pattern.to_owned();
                mirrored_x.id = id_counter;
                id_counter += 1;

                let mut mirrored_y = pattern.to_owned();
                mirrored_y.id = id_counter;
                id_counter += 1;

                let mut combination = pattern.to_owned();
                combination.id = id_counter;
                id_counter += 1;

                mirrored_x.contents.reverse();

                for x_val in mirrored_y.contents.iter_mut() {
                    x_val.reverse();
                }

                combination.contents.reverse();
                for x_val in combination.contents.iter_mut() {
                    x_val.reverse();
                }

                let x_transf = |rule: &mut Rule| {
                    // rule content must also be mirrored: AB CD EF -> FE DC BA
                    rule.content.reverse();

                    // only horizontal rules are reversed in place
                    rule.direction = match rule.direction {
                        1 => 3,
                        3 => 1,
                        _ => panic!(),
                    }
                };

                let y_transf = |rule: &mut Rule| {
                    // mirror rule content on y-axis
                    for row in rule.content.iter_mut() {
                        row.reverse();
                    }

                    // swap top and bottom rules
                    rule.direction = match rule.direction {
                        0 => 2,
                        2 => 0,
                        _ => panic!(),
                    }
                };

                for rule in &mut mirrored_x.rules {
                    x_transf(rule);
                }

                for rule in &mut mirrored_y.rules {
                    y_transf(rule);
                }

                for rule in &mut combination.rules {
                    x_transf(rule);
                    y_transf(rule);
                }

                new_patterns = vec![mirrored_x, mirrored_y, combination];
            }

            patterns.append(&mut new_patterns);
        }

        dedup_patterns(&mut patterns);

        self.patterns = patterns;
    }
}

fn dedup_patterns(patterns: &mut Vec<Pattern>) {
    let copy = patterns.to_owned();

    for pattern in patterns.iter_mut() {
        for patcopy in &copy {
            if pattern.contents == patcopy.contents {
                if pattern.id != patcopy.id {
                    pattern.count += 1;

                    // check duplicate's rules
                    for rule in &patcopy.rules {
                        // push the new rule
                        if !pattern.rules.contains(&rule) {
                            pattern.rules.push(rule.to_owned());
                        }
                    }

                    debug_assert_ne!(pattern, patcopy);
                }
            }
        }
    }

    std::mem::drop(copy);

    for pattern in patterns.iter_mut() {
        pattern.rules.dedup();
    }

    patterns.dedup();
}

#[derive(Debug, Clone)]
struct Pattern {
    id: usize,
    count: usize,
    contents: Vec<Vec<usize>>,
    rules: Vec<Rule>,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents && self.rules == other.rules && self.count == other.count
    }
}

impl Pattern {
    fn new(id: usize, contents: Vec<Vec<usize>>) -> Self {
        Pattern {
            id,
            count: 1,
            contents,
            rules: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rule {
    /// direction corresponds to the top, right, bottom, left directions
    /// 0: up
    /// 1: right
    /// 2: down
    /// 3: left
    direction: u8,
    content: Vec<Vec<usize>>,
}

impl Rule {
    fn new(direction: u8, content: Vec<Vec<usize>>) -> Self {
        Self { direction, content }
    }
}

struct Element {
    values: Vec<Rc<Pattern>>,
    position: Vector2<u16>,
}

impl Element {
    fn entropy(&self) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    #[test]
    fn dedup_patterns_works() {
        let mut patterns = vec![
            Pattern {
                id: 0,
                count: 1,
                contents: vec![vec![0]],
                rules: vec![Rule::new(0, vec![vec![1]])],
            },
            Pattern {
                id: 1,
                count: 1,
                contents: vec![vec![1]],
                rules: vec![Rule::new(2, vec![vec![0]])],
            },
            Pattern {
                id: 2,
                count: 1,
                contents: vec![vec![1]],
                rules: vec![Rule::new(2, vec![vec![0]]), Rule::new(2, vec![vec![0]])],
            },
        ];

        dedup_patterns(&mut patterns);

        assert_eq!(patterns.len(), 2);
        assert!(patterns.iter().all(|p| p.rules.len() == 1));
        assert_eq!(
            patterns.iter().map(|p| p.count).filter(|c| *c == 2).count(),
            1,
            "{:#?}",
            patterns,
        );

        let mut hash_list: Vec<u64> = vec![];

        for pattern in patterns.iter() {
            let mut hasher = DefaultHasher::new();
            hasher.write_usize(pattern.count);

            for row in &pattern.contents {
                for n in row {
                    hasher.write_usize(*n);
                }
            }

            for rule in &pattern.rules {
                hasher.write_u8(rule.direction);

                for row in &rule.content {
                    for n in row {
                        hasher.write_usize(*n);
                    }
                }
            }

            hash_list.push(hasher.finish());
        }

        let copy = hash_list.to_owned();
        hash_list.dedup();

        assert_eq!(copy, hash_list);
    }

    #[test]
    fn wave_analyzer_works() {
        let mut wave = Wave::new();
        let input = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];

        wave.flags.push(Flags::NoTransforms);
        wave.analyze(input, Vector2::new(2, 2), BorderMode::Clamp);

        assert_eq!(wave.patterns.len(), 4);
    }
}
