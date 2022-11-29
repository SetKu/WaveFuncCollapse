pub mod helpers;
pub mod stringtools;
pub mod prelude;

pub use helpers::BorderMode;

use cgmath::Vector2;
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
    pub flags: Vec<Flags>,
    patterns: Vec<Pattern>,
    patterns_total: usize,
    elements: Vec<Element>,
    chunk_size: Vector2<usize>,
    chunk_fill_size: Vector2<usize>,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            flags: vec![],
            patterns: vec![],
            patterns_total: 0,
            elements: vec![],
            chunk_size: Vector2::new(0, 0),
            chunk_fill_size: Vector2::new(0, 0),
        }
    }

    /// Collapses continuously until the wave function either completely collapses or the max number of contradictions (attempts has been reached).
    ///
    /// # Arguments
    /// * callback: An optional function which accepts the number of iterations and failures occured, as well as the current state of the wave function. This can be used for logging or print purposes.
    ///
    /// # Notes
    ///
    /// * The number of iterations resets after a failed attempt. The number of failures is never reset.
    /// * The current representation (`Vec<Vec<Vec<usize>>>`) is provided in the callback as its not available while the function is borrowing the `Wave`.
    /// * No final perfect result is returned from a successful run as to avoid doing extra work in case the caller doesn't need the final representation.
    pub fn collapse_all<F>(
        &mut self,
        max_contradictions: usize,
        callback: Option<F>,
    ) -> Result<(), String>
    where
        F: Fn(usize, usize, Vec<Vec<Vec<usize>>>),
    {
        if self.chunk_fill_size.x == 0 || self.chunk_fill_size.y == 0 {
            return Err("The superpositions are empty or were not filled properly".to_owned());
        }

        let mut failures = 0;
        let mut iterations = 0;

        while !self.completely_collapsed() {
            self.collapse_once();

            if self.contradiction_occurred() {
                failures += 1;

                if failures == max_contradictions {
                    return Err("The max number of contradictions has been reached".to_owned());
                }

                self.fill(self.true_size())?;
                iterations = 0;
            } else {
                iterations += 1;
            }

            if let Some(cb) = &callback {
                cb(iterations, failures, self.current_rep());
            }
        }

        Ok(())
    }

    fn contradiction_occurred(&self) -> bool {
        self.elements.iter().filter(|e| e.values.is_empty()).count() != 0
    }

    fn completely_collapsed(&self) -> bool {
        self.elements.iter().all(|e| e.is_collapsed())
    }

    pub fn perfect_rep(&self) -> Result<Vec<Vec<usize>>, String> {
        if self.elements.is_empty() {
            return Err("There are no superpositions to create a representation from".to_owned());
        }

        if self.contradiction_occurred() {
            return Err(
                "A contradiction occurred preventing the formation of a perfect representation"
                    .to_owned(),
            );
        }

        if !self.completely_collapsed() {
            return Err("The superpositions aren't completely collapsed yet".to_owned());
        }

        let mut pairs: Vec<(usize, Vector2<usize>)> = vec![];

        for element in self.elements.iter() {
            let real_origin = Vector2 {
                x: element.position.x * self.chunk_size.x,
                y: element.position.y * self.chunk_size.y,
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

        let result = arrayify(pairs, &self.true_size());
        Ok(result)
    }

    fn true_size(&self) -> Vector2<usize> {
        Vector2 {
            x: self.chunk_fill_size.x * self.chunk_size.x,
            y: self.chunk_fill_size.y * self.chunk_size.y,
        }
    }

    pub fn current_rep(&self) -> Vec<Vec<Vec<usize>>> {
        if self.elements.is_empty() {
            return vec![];
        }

        let mut pairs: Vec<(Vec<usize>, Vector2<usize>)> = vec![];

        for element in &self.elements {
            let real_origin = Vector2 {
                x: element.position.x * self.chunk_size.x,
                y: element.position.y * self.chunk_size.y,
            };

            for cx in 0..self.chunk_size.x {
                for cy in 0..self.chunk_size.y {
                    let mut new_pair = (
                        vec![],
                        Vector2 {
                            x: real_origin.x + cx,
                            y: real_origin.y + cy,
                        },
                    );

                    for value in &element.values {
                        new_pair.0.push(value.contents[cx][cy]);
                    }

                    // deduplication is required because when working with the overlapping tiled
                    // model the patterns are set up such that they can at times have duplicated
                    // contents, which is a little bit disorienting and doesn't make sense to the
                    // caller of this function.
                    new_pair.0.sort();
                    new_pair.0.dedup();

                    pairs.push(new_pair);
                }
            }
        }

        let result = arrayify(pairs, &self.true_size());
        result
    }

    /// # Notes
    ///
    /// * The wave doesn't stop propogating until its completely iterated over the entire superposition grid. It does this as, although on the first run it doesn't make much sense, on future runs it will propagate out changes between the sites of different collapses.
    pub fn collapse_once(&mut self) {
        if self.elements.is_empty() {
            return;
        }

        let mut selected_elements = vec![0usize];
        let mut greatest_entropy = 0.;

        for (i, element) in self.elements.iter().enumerate() {
            if element.is_collapsed() {
                continue;
            }

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
        let center = &self.elements[center_element];
        let center_pos = center.position.clone();
        std::mem::drop(center);

        let mut current_locs = noneg_neighbours(&center_pos);

        let first_reference = (
            center_pos.clone(),
            center.values.iter().map(|v| v.rules.clone()).collect(),
        );

        let mut references: Vec<(Vector2<usize>, Vec<Vec<Rule>>)> = vec![first_reference];
        let mut banned_locs: Vec<Vec<Vector2<usize>>> = vec![vec![center_pos.clone()]];

        loop {
            let mut indexes = vec![];

            for loc in &current_locs {
                for i in 0..self.elements.len() {
                    if self.elements[i].position == *loc {
                        indexes.push(i);
                    }
                }
            }

            if indexes.is_empty() {
                // propagation finished!
                return;
            }

            let mut new_references = vec![];
            let mut new_locs = vec![];

            // *** Value Pruning ***
            for i in indexes {
                let element = &mut self.elements[i];
                let element_neighbours = noneg_neighbours(&element.position);
                let mut values_to_remove = vec![];

                for (value_idx, value) in element.values.iter().enumerate() {
                    let mut valid = false;

                    // now we search for a valid reason to keep the element!
                    'refloop: for reference in &references {
                        if element_neighbours.contains(&reference.0) {
                            let direction = orthog_direction(&reference.0, &element.position);

                            for rule_set in &reference.1 {
                                for rule in rule_set {
                                    if rule.content == value.contents && rule.direction == direction
                                    {
                                        valid = true;
                                        break 'refloop;
                                    }
                                }
                            }
                        }
                    }

                    if !valid {
                        values_to_remove.push(value_idx);
                    }
                }

                remove_indexes(&mut element.values, values_to_remove);

                let reference = (
                    element.position.clone(),
                    element
                        .values
                        .iter()
                        .map(|v| v.rules.clone())
                        .collect::<Vec<Vec<Rule>>>(),
                );

                new_references.push(reference);

                new_locs.push(noneg_neighbours(&element.position));
            }

            // *** Clean Up and Preparation ***
            references = new_references;
            banned_locs.push(current_locs.clone());

            if banned_locs.len() == 3 {
                banned_locs.remove(0);
            }

            current_locs.clear();

            for neighbour_set in new_locs {
                for loc in neighbour_set {
                    // check if banned
                    if banned_locs.iter().find(|s| s.contains(&loc)).is_some() {
                        continue;
                    }

                    current_locs.push(loc);
                }
            }

            current_locs.dedup();
        }
    }

    pub fn fill(&mut self, size: Vector2<usize>) -> Result<(), String> {
        if size.x % self.chunk_size.x != 0 {
            return Err("The output width must be a factor of the chunk size".to_owned());
        }

        if size.y % self.chunk_size.y != 0 {
            return Err("The output height must be a factor of the chunk size".to_owned());
        }

        self.elements.clear();

        let values_preset: Vec<Rc<Pattern>> = self
            .patterns
            .clone()
            .into_iter()
            .map(|p| Rc::new(p))
            .collect();

        let chunk_fill_size = Vector2 {
            x: size.x / self.chunk_size.x,
            y: size.y / self.chunk_size.y,
        };

        for x in 0..chunk_fill_size.x {
            for y in 0..chunk_fill_size.y {
                let values = values_preset.clone();
                let position = Vector2::new(x, y);
                let element = Element::new(values, position);
                self.elements.push(element);
            }
        }

        self.chunk_fill_size = chunk_fill_size;

        Ok(())
    }

    /// Please note, the flag `Flag::NoTransforms` must be set at this point for it to be registered.
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
            let mut pattern = Pattern::new(id_counter, adjacency.origin_content.to_owned());
            id_counter += 1;

            for (i, neighbour) in adjacency.neighbours_content.into_iter().enumerate() {
                if let Some(unwrapped) = neighbour {
                    let rule = Rule::new(i as u8, unwrapped);
                    pattern.rules.push(rule);
                }
            }

            patterns.push(pattern);
        }

        count_patterns(&mut patterns);
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

                new_patterns.append(&mut (vec![mirrored_x, mirrored_y, combination]));
            }

            patterns.append(&mut new_patterns);
        }

        dedup_patterns(&mut patterns);

        self.patterns = patterns;
        self.patterns_total = initial_count;
        self.chunk_size = chunk_size;
    }
}

/// If counting, always ensure that you count up your patterns before deduplicating them.
///
/// This function will do nothing if the patterns are deduplicated.
fn count_patterns(patterns: &mut Vec<Pattern>) {
    let copy = patterns.to_owned();

    for pattern in patterns.iter_mut() {
        // counting doesn't deal with transforms,
        // as they aren't part of the original
        // patterns
        if pattern.is_transform {
            continue;
        }

        for patcopy in &copy {
            if patcopy.is_transform {
                continue;
            }

            if pattern.contents == patcopy.contents {
                if pattern.id != patcopy.id {
                    pattern.count += 1;
                }
            }
        }
    }
}

fn dedup_patterns(patterns: &mut Vec<Pattern>) {
    let copy = patterns.to_owned();

    for pattern in patterns.iter_mut() {
        for patcopy in &copy {
            if pattern.contents == patcopy.contents {
                if pattern.id != patcopy.id {
                    // check duplicate's rules
                    for rule in &patcopy.rules {
                        // push the new rule
                        if !pattern.rules.contains(&rule) {
                            pattern.rules.push(rule.to_owned());
                        }
                    }
                }
            }
        }
    }

    std::mem::drop(copy);

    for pattern in patterns.iter_mut() {
        pattern.rules.sort();
        pattern.rules.dedup();
    }

    patterns.sort();
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

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents && self.rules == other.rules && self.count == other.count
    }
}

impl Eq for Pattern {}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.contents.cmp(&other.contents))
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.contents.cmp(&other.contents)
    }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Rule {
    /// direction corresponds to the top, right, bottom, left directions
    /// 0: up
    /// 1: right
    /// 2: down
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
