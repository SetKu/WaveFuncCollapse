pub mod helpers;
use cgmath::Vector2;
pub use helpers::BorderMode;
use helpers::*;
use rand::prelude::*;
use rand::thread_rng;
use std::clone::Clone;
use std::rc::Rc;

#[cfg(test)]
mod tests;

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
    patterns_total: usize,
    elements: Vec<Element>,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            flags: vec![],
            patterns: vec![],
            patterns_total: 0,
            elements: vec![],
        }
    }

    fn contradiction_occurred(&self) -> bool {
        self.elements.iter().filter(|e| e.values.is_empty()).count() != 0
    }

    fn completely_collapsed(&self) -> bool {
        self.elements.iter().all(|e| e.is_collapsed())
    }

    pub fn finalize(&self) -> Vec<Vec<usize>> {
        if self.elements.is_empty() || self.contradiction_occurred() || !self.completely_collapsed()
        {
            return vec![];
        }

        let mut pairs: Vec<(usize, Vector2<usize>)> = vec![];
        let chunk_size = dimensions_of(
            &self
                .elements
                .first()
                .unwrap()
                .values
                .first()
                .unwrap()
                .contents,
        );

        for element in self.elements.iter() {
            let real_origin = Vector2 {
                x: element.position.x * chunk_size.x,
                y: element.position.y * chunk_size.y,
            };

            let contents = &element.values.first().unwrap().contents;

            for (x, col) in contents.iter().enumerate() {
                for (y, id) in col.iter().enumerate() {
                    let real_pos = Vector2 {
                        x: real_origin.x + x,
                        y: real_origin.y + y,
                    };

                    pairs.push((*id, real_pos));
                }
            }
        }

        let mut max = Vector2::new(0, 0);
        for pair in &pairs {
            if pair.1.x >= max.x && pair.1.y >= max.y {
                max = pair.1.clone();
            }
        }

        debug_assert_ne!(max, Vector2::new(0, 0));

        let result = arrayify(pairs, &max);
        result
    }

    pub fn collapse_once(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        let mut selected_elements = vec![0usize];
        let mut greatest_entropy = 0.;

        for (i, element) in self.elements.iter().enumerate() {
            let ent = element.entropy(self.patterns_total);

            if ent == 0. {
                continue;
            }

            if ent > greatest_entropy {
                greatest_entropy = ent;
                selected_elements = vec![i];
            } else if ent == greatest_entropy {
                selected_elements.push(i);
            }
        }

        let mut rng = thread_rng();
        let selected_element = selected_elements.choose(&mut rng).unwrap();
        let borrow = &mut self.elements[*selected_element];
        let choice = if self.flags.contains(&Flags::NoWeights) {
            borrow.values.choose(&mut rng).unwrap()
        } else {
            borrow
                .values
                .choose_weighted(&mut rng, |v| v.count)
                .unwrap()
        };

        // finish collapse!
        let choice_value = choice.clone();
        std::mem::drop(choice);
        borrow.values.clear();
        borrow.values.push(choice_value);

        // propogate changes
        self.propagate(*selected_element);
    }

    pub fn propagate(&mut self, center_element: usize) {
        let element = &self.elements[center_element];
        let center_pos = element.position.clone();
        // let center_pos_cast = center_pos.clone().cast::<f32>().unwrap();
        let mut next_locations = pos_neighbours(&center_pos);
        let first_reference: (Vector2<usize>, Vec<Vec<Rule>>) = (
            element.position.clone(),
            element.values.iter().map(|v| v.rules.clone()).collect(),
        );
        let mut previous_references: Vec<(Vector2<usize>, Vec<Vec<Rule>>)> = vec![first_reference];
        let mut used_locations: Vec<Vec<Vector2<usize>>> = vec![];

        while !next_locations.is_empty() {
            let mut elements_indexes: Vec<usize> = vec![];

            // get indexes for the next locations
            for loc in &next_locations {
                let len = self.elements.len();

                for i in 0..len {
                    if self.elements[i].position == *loc {
                        elements_indexes.push(i);
                    }
                }
            }

            if elements_indexes.is_empty() {
                // propagation is finished!
                // exit the function.
                return;
            }

            let mut new_references: Vec<(Vector2<usize>, Vec<Vec<Rule>>)> = vec![];

            for index in elements_indexes {
                let iter_element = &mut self.elements[index];
                let mut remove_list = vec![];

                for (i, value) in iter_element.values.iter().enumerate() {
                    // assume all its references say its valid to start
                    let mut surely_valid = true;

                    for reference in &previous_references {
                        let mut is_valid = false;

                        // check if the reference is a neighbour of the current superposition
                        // if it is, we refer to its rules when making deductions
                        if pos_neighbours(&reference.0).contains(&iter_element.position) {
                            // check if there is a rule in the last iteration validating this value
                            for rule_set in &reference.1 {
                                for rule in rule_set {
                                    if rule.content == value.contents {
                                        // this value is valid
                                        is_valid = true;
                                        break;
                                    }
                                }

                                if is_valid {
                                    break;
                                }
                            }
                        }

                        // this code will run if no rule could be found to validate this value
                        if !is_valid {
                            // if this reference has invalidated this value, no
                            // need to check other references.

                            // skip to removal!
                            surely_valid = false;
                            break;
                        }
                    }

                    if !surely_valid {
                        // this value is invalid!
                        remove_list.push(i);
                    }
                }

                debug_assert_eq!(remove_list.len(), {
                    let mut a = remove_list.clone();
                    a.dedup();
                    a.len()
                });

                let mut removed = 0usize;
                for i in remove_list {
                    iter_element.values.remove(i - removed);
                    removed += 1;
                }

                let reference: (Vector2<usize>, Vec<Vec<Rule>>) = (
                    iter_element.position.clone(),
                    iter_element.values.iter().map(|v| v.rules.clone()).collect(),
                );
                new_references.push(reference);
            }

            // prep for next propagation
            previous_references = new_references;
            used_locations.push(next_locations.clone());
            
            if used_locations.len() == 3 {
                // only 2 levels of knowledge are required to prune
                // already used locations
                used_locations.remove(0);
            }

            next_locations.clear();

            for reference in &previous_references {
                let neighbours = pos_neighbours(&reference.0);
                // let mut valid_neighbours: Vec<Vector2<usize>> = vec![];
                // let mut last: Option<Vector2<usize>> = None;
                // let mut lowest_dist = f32::MAX;

                // exclude the closest neighbour
                // for neighbour in neighbours {
                    // let cast = neighbour.cast::<f32>().unwrap();
                    // let diff = cast - center_pos_cast;
                    // // apply pythagorean theorem to calculate distance
                    // let dist = (diff.x * diff.x + diff.y * diff.y).sqrt();

                    // if dist < lowest_dist {
                        // lowest_dist = dist;

                        // if let Some(previous) = last {
                            // valid_neighbours.push(previous)
                        // }

                        // last = Some(neighbour);
                    // } else {
                        // valid_neighbours.push(neighbour);
                    // }
                // }

                for neighbour in neighbours {
                    let mut valid = true;

                    // prevent an infinite loop by not running the same locations again
                    for loc_set in &used_locations {
                        for loc in loc_set {
                            if *loc == neighbour {
                                valid = false;
                                break;
                            }
                        }

                        if !valid {
                            break;
                        }
                    }

                    if !valid {
                        continue;
                    }

                    next_locations.push(neighbour)
                }
            }
        }
    }

    pub fn fill(&mut self, size: Vector2<usize>) -> Result<(), String> {
        if let Some(first) = self.patterns.first() {
            let chunk_size = dimensions_of(&first.contents);

            if size.x % chunk_size.x != 0 {
                return Err("The output width must be a factor of the chunk size".to_owned());
            }

            if size.y % chunk_size.y != 0 {
                return Err("The output height must be a factor of the chunk size".to_owned());
            }
        }

        self.elements.clear();

        let values_preset: Vec<Rc<Pattern>> = self
            .patterns
            .clone()
            .into_iter()
            .map(|p| Rc::new(p))
            .collect();

        for x in 0..size.x {
            for y in 0..size.y {
                let values = values_preset.clone();
                let position = Vector2::new(x, y);
                let element = Element::new(values, position);
                self.elements.push(element);
            }
        }

        Ok(())
    }

    pub fn analyze(
        &mut self,
        input: Vec<Vec<usize>>,
        chunk_size: Vector2<usize>,
        border_mode: BorderMode,
    ) {
        let adjacencies = overlapping_adjacencies(input.to_owned(), chunk_size, border_mode);
        let initial_count = adjacencies.len();

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
                mirrored_x.is_transform = true;
                mirrored_x.id = id_counter;
                id_counter += 1;

                let mut mirrored_y = pattern.to_owned();
                mirrored_y.is_transform = true;
                mirrored_y.id = id_counter;
                id_counter += 1;

                let mut combination = pattern.to_owned();
                combination.is_transform = true;
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
                    if rule.direction == 1 {
                        rule.direction = 3
                    } else if rule.direction == 3 {
                        rule.direction = 1
                    }
                };

                let y_transf = |rule: &mut Rule| {
                    // mirror rule content on y-axis
                    for row in rule.content.iter_mut() {
                        row.reverse();
                    }

                    // swap top and bottom rules
                    if rule.direction == 0 {
                        rule.direction = 2
                    } else if rule.direction == 2 {
                        rule.direction = 0
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
        self.patterns_total = initial_count;
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
    is_transform: bool,
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
            is_transform: false,
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
    /// The valid neighbour for the originating pattern of this rule.
    content: Vec<Vec<usize>>,
}

impl Rule {
    fn new(direction: u8, content: Vec<Vec<usize>>) -> Self {
        Self { direction, content }
    }
}

struct Element {
    values: Vec<Rc<Pattern>>,
    position: Vector2<usize>,
}

impl Element {
    fn new(values: Vec<Rc<Pattern>>, position: Vector2<usize>) -> Self {
        Self { values, position }
    }

    fn entropy(&self, patterns_total: usize) -> f32 {
        if self.values.is_empty() {
            return 0.;
        }

        let mut total = 0f32;

        for pattern in self.values.iter() {
            let prob = pattern.count as f32 / patterns_total as f32;
            let entropy = prob * (1.0 / prob).log2();
            total += entropy;
        }

        total
    }

    fn is_collapsed(&self) -> bool {
        self.values.len() == 1
    }
}
