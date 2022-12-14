pub mod helpers;
pub mod prelude;
pub mod stringtools;

pub use helpers::BorderMode;

use cgmath::Vector2;
use helpers::*;
use rand::prelude::*;
use rand::thread_rng;
use std::clone::Clone;
use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Flags for the `Wave`.
#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Flags {
    NoWeights = 1,
    NoTransforms,
    NoHistory,
    PruneDeadweight,
}

/// Encapsulation for the Wave Function Collapse implementation.
#[derive(Clone)]
pub struct Wave {
    pub flags: Vec<Flags>,
    patterns: Vec<Pattern>,
    patterns_total: usize,
    elements: Vec<Element>,
    chunk_size: Vector2<usize>,
    chunk_fill_size: Vector2<usize>,
    history: Vec<Record>,
    iterations: usize,
    debug: bool,
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
            history: vec![],
            iterations: 0,
            debug: false,
        }
    }

    /// Sets the debug mode for the wave. This will print some extra output helpful for debugging.
    pub fn set_debug(&mut self) {
        self.debug = true;
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
    /// * When an error is returned, the final state at which the error occurred is preserved in the wave.
    pub fn collapse_all<F>(
        &mut self,
        max_contradictions: usize,
        callback: Option<F>,
    ) -> Result<(), String>
    where
        F: Fn(usize, usize, Vec<Vec<Vec<usize>>>),
    {
        if self.patterns.is_empty() {
            return Err("The number of rules identified was zero. The input was flawed or the wave was configured incorrectly.".to_owned());
        }

        if self.chunk_fill_size.x == 0 || self.chunk_fill_size.y == 0 {
            return Err("The superpositions are empty or were not filled properly".to_owned());
        }

        let mut failures = 0;

        while !self.completely_collapsed() {
            self.collapse_once();

            if self.contradiction_occurred() {
                failures += 1;

                if failures == max_contradictions {
                    return Err("The max number of contradictions has been reached".to_owned());
                }

                self.fill(self.true_size())?;
                self.clear_history();
                self.iterations = 0;
            } else {
                self.iterations += 1;
            }

            if let Some(cb) = &callback {
                cb(self.iterations, failures, self.current_rep());
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

    /// Returns the perfect representation of the current internal state of the wave.
    ///
    /// This function will throw an error if the internal wave isn't completely collapsed.
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

    /// Causing the wave to perform one collapse. This will also cause consequent propagation.
    ///
    /// # Notes
    ///
    /// * The wave doesn't stop propogating until its completely iterated over the entire superposition grid. It does this as, although on the first run it doesn't make much sense, on future runs it will propagate out changes between the sites of different collapses.
    /// * After this function is called, it saves what it did to a private history log.
    ///     * This can be disabled using the `NoHistory` flag.
    /// * If the internal superposition grid is empty, this function will do nothing.
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

        debug_assert!(!selected_elements.is_empty());

        let mut rng = thread_rng();
        let selected_element = selected_elements.choose(&mut rng).unwrap();

        if self.debug {
            println!("Chosen element to collapse.");
        }

        let borrow = &mut self.elements[*selected_element];

        debug_assert!(!borrow.values.is_empty());

        let choice = if self.flags.contains(&Flags::NoWeights) {
            borrow.values.choose(&mut rng).unwrap()
        } else {
            borrow
                .values
                .choose_weighted(&mut rng, |v| v.count)
                .unwrap()
        };
        
        if self.debug {
            println!("Chosen element to collapse too.");
        }

        if self.flags.contains(&Flags::NoHistory) {
            if self.debug {
                println!("Creating history record.");
            }

            let previous_pattern_ids = borrow.values.iter().map(|p| p.id).collect();
            let new_record = Record::new(
                borrow.position.clone(),
                choice.id,
                previous_pattern_ids,
                self.iterations,
            );
            self.history.push(new_record);
        }

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

        if self.debug {
            println!("Propagating from {:?}", center_pos);
        }

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
                
                if self.debug {
                    println!("Finished propagating.");
                }

                return;
            }

            let mut new_references = vec![];
            let mut new_locs = vec![];

            if self.debug {
                println!("Propagating output to {:?}", current_locs);
            }

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
        if self.debug {
            println!("Filling superpositions with the following size: {:?}", size);
        }

        if size.x % self.chunk_size.x != 0 {
            return Err("The output width must be a factor of the chunk size".to_owned());
        }

        if size.y % self.chunk_size.y != 0 {
            return Err("The output height must be a factor of the chunk size".to_owned());
        }

        self.elements.clear();

        let values_preset: Vec<Arc<Pattern>> = self
            .patterns
            .clone()
            .into_iter()
            .map(|p| Arc::new(p))
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

        if self.flags.contains(&Flags::PruneDeadweight) {
            self.prune_lone_patterns();
        }
    }

    fn prune_lone_patterns(&mut self) {
        if self.debug {
            println!("Pruning lone patterns.");
        }

        let mut indexes_to_remove = vec![];

        for (i, pattern) in self.patterns.iter().enumerate() {
            if pattern.rules.len() < 5 {
                indexes_to_remove.push(i);
                continue;
            }

            let mut found_directions = [false, false, false, false];

            for rule in &pattern.rules {
                found_directions[rule.direction as usize] = true;

                if found_directions.iter().all(|b| *b) {
                    break;
                }
            }
        }

        let mut removed = 0usize;

        for i in indexes_to_remove {
            self.patterns.remove(i - removed);
            removed += 1;
        }
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

// History Related Functions and Code
impl Wave {
    /// Clears the wave's internal history log.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Undo's the last collapse undertaken by the algorithm.
    ///
    /// # Parameters:
    ///
    /// * `remove_record`: Determines whether the operation being undone should be forgotten in the internal history log or remembered.
    ///
    /// # Notes:
    ///
    /// * If the internal history log for the `Wave` is empty, this function does nothing.
    /// * If `remove_record` is set to true, if an error is returned, **DO NOT** attempt another undo.
    ///     * In this situation, the last undo will have been marked as undone but an error occurred during this process. Undoing again will likely result in an error for a variety of reasons.
    pub fn undo_collapse(&mut self, remove_record: bool) -> Result<(), String> {
        // Don't include undone records in the eventuality `remove_record` was marked false.
        let last_record = self
            .history
            .iter_mut()
            .enumerate()
            .filter(|r| !r.1.undone)
            .last();

        if let Some(record_info) = last_record {
            let index = record_info.0;

            record_info.1.undone = true;
            let clone = record_info.1.clone();
            self.reverse_record(clone)?;

            if remove_record {
                self.history.remove(index);
            }
        }

        Ok(())
    }

    /// Redos the previous undo if one occured. Otherwise, this function does nothing.
    ///
    /// # Notes:
    ///
    /// *
    pub fn redo_collapse(&mut self) -> Result<(), String> {
        let last_undone = self.history.iter_mut().find(|r| r.undone);

        if let Some(undone_record) = last_undone {
            undone_record.undone = false;
            let copy = undone_record.clone();
            self.execute_record(copy)?;
        } else if let Some(last_record) = self.history.last_mut() {
            let copy = last_record.clone();
            self.execute_record(copy)?;
        }

        Ok(())
    }

    /// Undos the previous collapse as described in the record.
    ///
    /// # Notes:
    ///
    /// * This function has the same quirks and behaviours that `execute_record` does due to their similar nature.
    /// * This function decrements the internal iterations count.
    fn reverse_record(&mut self, record: Record) -> Result<(), String> {
        if record.iteration != self.iterations {
            return Err(
                "This record's iteration does not match the internal state of the Wave".to_string(),
            );
        }

        let element = self
            .elements
            .iter_mut()
            .find(|e| e.position == record.location());
        if element.is_none() {
            return Err("Failed to find element at the specified location".to_string());
        };

        let patterns: Vec<&Pattern> = self
            .patterns
            .iter()
            .filter(|p| record.previous_pattern_ids.contains(&p.id))
            .collect();
        if patterns.is_empty() {
            return Err("Failed to find any patterns for the record id. This is possible, but shouldn't happen with the `Wave` history functioning as intended.".to_string());
        };
        let references = patterns.into_iter().map(|p| Arc::new(p.clone())).collect();

        element.unwrap().values = references;
        self.iterations -= 1;

        Ok(())
    }

    /// Executes the given record by attempting to perform the predetermined collapse it contains.
    ///
    /// This function can fail and return an error if the given record has invalid values that don't match with the state of the wave.
    ///
    /// # Notes:
    ///
    /// * This function will fail if the record's iteration does not line up with the current iteration.
    /// * This function has an inefficiency where, when searching for the pattern specified, it has to make a copy of it and create a new `Arc` to that copy.
    ///     * This could be subverted by searching for a prexisting copy of a `Arc` to that specific pattern in the other elements. However, I'm guessing this would be time intensive. For now the memory trade-off is being prioritized over the processing trade-off.
    fn execute_record(&mut self, record: Record) -> Result<(), String> {
        if record.iteration != self.iterations {
            return Err(
                "This record's iteration does not match the internal state of the Wave".to_string(),
            );
        }

        let element = self
            .elements
            .iter_mut()
            .find(|e| e.position == record.location());
        if element.is_none() {
            return Err("Failed to find element at the specified location".to_string());
        };

        let pattern = self
            .patterns
            .iter()
            .find(|p| p.id == record.chosen_pattern_id);
        if pattern.is_none() {
            return Err("Failed to find pattern for the record id".to_string());
        };
        let reference = Arc::new(pattern.unwrap().clone());

        element.unwrap().values = vec![reference];

        Ok(())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[derive(Clone)]
struct Element {
    values: Vec<Arc<Pattern>>,
    position: Vector2<usize>,
}

impl Element {
    fn new(values: Vec<Arc<Pattern>>, position: Vector2<usize>) -> Self {
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

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Record {
    element_location: [usize; 2],
    /// An index in the patterns of the Wave for the selected value
    chosen_pattern_id: usize,
    previous_pattern_ids: Vec<usize>,
    /// The current `Wave` iteration when the record was made.
    iteration: usize,
    undone: bool,
}

impl Record {
    fn new(
        location: Vector2<usize>,
        chosen_pattern_id: usize,
        previous_pattern_ids: Vec<usize>,
        iteration: usize,
    ) -> Self {
        Record {
            element_location: [location.x, location.y],
            chosen_pattern_id,
            previous_pattern_ids,
            iteration,
            undone: false,
        }
    }

    fn location(&self) -> Vector2<usize> {
        Vector2::new(self.element_location[0], self.element_location[1])
    }
}

#[cfg(feature = "serde")]
mod wave_serialization {
    use super::*;

    impl Serialize for Wave {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("Wave", 7)?;
            state.serialize_field("flags", &self.flags)?;
            state.serialize_field("patterns", &self.patterns)?;
            state.serialize_field("patterns_total", &self.patterns_total)?;
            state.serialize_field("chunk_size", &[self.chunk_size.x, self.chunk_size.y])?;
            state.serialize_field(
                "chunk_fill_size",
                &[self.chunk_fill_size.x, self.chunk_fill_size.y],
            )?;
            state.serialize_field("history", &self.history)?;
            state.serialize_field("iterations", &self.iterations)?;
            state.end()
        }
    }

    struct WaveVisitor;

    impl<'de> Visitor<'de> for WaveVisitor {
        type Value = Wave;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "expecting wave object")
        }
    }

    impl Default for WaveVisitor {
        fn default() -> Self {
            Self {}
        }
    }

    impl<'de> Deserialize<'de> for Wave {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_struct(
                "Wave",
                &[
                    "flags",
                    "patterns",
                    "patterns_total",
                    "chunk_size",
                    "chunk_fill_size",
                    "history",
                    "iterations",
                ],
                WaveVisitor::default(),
            )
        }
    }
}
