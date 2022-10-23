use rand::{prelude::SliceRandom, thread_rng};
use std::clone::Clone;
use std::rc::Rc;

mod errors;
use errors::*;

#[derive(Debug, PartialEq, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new_i32(x: i32, y: i32) -> Self {
        Location { x, y }
    }

    fn new_u32(x: u32, y: u32) -> Self {
        Location {
            x: x as i32,
            y: y as i32,
        }
    }

    fn new_usize(x: usize, y: usize) -> Self {
        Location {
            x: x as i32,
            y: y as i32,
        }
    }

    // Returns the top, right, down, left neighbours to a specified location.
    fn orthogonal_neighbours(&self) -> [(Location, Direction); 4] {
        use Direction::*;
        [
            (Self::new_i32(self.x, self.y - 1), Up),
            (Self::new_i32(self.x + 1, self.y), Right),
            (Self::new_i32(self.x, self.y + 1), Down),
            (Self::new_i32(self.x - 1, self.y), Left),
        ]
    }

    fn diagonal_neighbours(&self) -> [(Location, Direction); 4] {
        use Direction::*;
        [
            (Self::new_i32(self.x - 1, self.y - 1), UpLeft),
            (Self::new_i32(self.x + 1, self.y - 1), UpRight),
            (Self::new_i32(self.x + 1, self.y + 1), DownRight),
            (Self::new_i32(self.x - 1, self.y + 1), DownLeft),
        ]
    }

    fn all_neighbours(&self) -> Vec<(Location, Direction)> {
        self.orthogonal_neighbours()
            .into_iter()
            .chain(self.diagonal_neighbours().into_iter())
            .collect::<Vec<(Location, Direction)>>()
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Loc({}, {})", self.x, self.y)
    }
}

#[derive(Debug, std::clone::Clone, PartialEq)]
enum Direction {
    UpLeft = 0,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity<T>
where
    T: PartialEq + Clone,
{
    item: T,
    rules: Vec<(T, Direction)>,
    weight: i32,
}

// Generic syntax for implementations: https://is.gd/gYBL5c
impl<T> Entity<T>
where
    T: PartialEq + Clone,
{
    fn new(i: T) -> Self {
        Entity {
            item: i,
            rules: vec![],
            weight: 1,
        }
    }

    // Adds a rule only if it doesn't already existing in validations.
    fn add_rule(&mut self, v: (T, Direction)) {
        let mut iter = self.rules.iter();
        let existing_val = iter.find(|&e| e.0 == v.0 && e.1 == v.1);

        if None == existing_val {
            self.rules.push(v);
        }
    }
}

#[derive(Debug, Clone)]
struct Superposition<T>
where
    T: PartialEq + Clone,
{
    location: Location,
    candidates: Vec<Rc<Entity<Vec<T>>>>,
}

impl<T> Superposition<T>
where
    T: PartialEq + Clone,
{
    fn new(l: Location) -> Self {
        Superposition {
            location: l,
            candidates: vec![],
        }
    }

    fn is_collapsed(&self) -> bool {
        self.candidates.len() == 1
    }

    fn entropy(&self) -> usize {
        self.candidates.len()
    }
}

pub struct Coordinator<T>
where
    T: PartialEq + Clone,
{
    superpositions: Vec<Superposition<T>>,
    entities: Vec<Entity<Vec<T>>>,
    pub use_weights: bool,
    pub use_transforms: bool,
}

impl<T> Coordinator<T>
where
    T: PartialEq + Clone,
{
    fn new() -> Self {
        Coordinator {
            superpositions: vec![],
            entities: Vec::new(),
            use_weights: true,
            use_transforms: true,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn collapse_all(&mut self) -> Result<Vec<Entity<T>>, WaveError> {
        while !self.superpositions.iter().all(|i| i.is_collapsed()) {}

        Ok(vec![])
    }

    pub fn collapse(&mut self) -> Result<(), WaveError> {
        let mut lowest_entropies: Vec<&mut Superposition<T>> = Vec::new();

        for superposition in &mut self.superpositions {
            if superposition.is_collapsed() {
                continue;
            }

            if lowest_entropies.is_empty() {
                lowest_entropies.push(superposition);
                continue;
            }

            let old = lowest_entropies.first().unwrap().entropy();
            let new = superposition.entropy();

            if old > new {
                lowest_entropies.clear();
            }

            if old >= new {
                lowest_entropies.push(superposition);
            }
        }

        if lowest_entropies.is_empty() {
            return Err(WaveError::Contradiction);
        }

        let mut generator = thread_rng();
        let chosen_superposition = lowest_entropies.choose_mut(&mut generator).unwrap();
        chosen_superposition.candidates.clear();

        if self.use_weights {
            let chosen_candidate = chosen_superposition
                .candidates
                .choose_weighted(&mut generator, |c| c.weight)
                .unwrap()
                .clone();
            chosen_superposition.candidates.push(chosen_candidate);
        } else {
            let chosen_candidate = chosen_superposition
                .candidates
                .choose(&mut generator)
                .unwrap()
                .clone();
            chosen_superposition.candidates.push(chosen_candidate);
        }

        debug_assert!(chosen_superposition.is_collapsed());

        let entity = chosen_superposition.candidates.first().unwrap().clone();
        let neighbours_locations = chosen_superposition.location.all_neighbours();

        // Ripple Propogation
        for neighbour_location in neighbours_locations {
            let found_superposition_opt = self
                .superpositions
                .iter_mut()
                .find(|s| s.location == neighbour_location.0);

            if let Some(found_superposition) = found_superposition_opt {
                let mut removed = 0;

                for (i, candidate) in found_superposition.candidates.clone().iter().enumerate() {
                    let mut is_valid = false;

                    for rule in entity.rules.iter() {
                        if rule.0 == candidate.item && rule.1 == neighbour_location.1 {
                            is_valid = true;
                            break;
                        }
                    }

                    if !is_valid {
                        found_superposition.candidates.remove(i - removed);
                        removed += 1;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn populate_superpositions(&mut self, width: u32, height: u32) {
        self.superpositions.clear();

        let entities_lock: Vec<Rc<Entity<Vec<T>>>> =
            self.entities.iter().map(|e| Rc::new(e.clone())).collect();

        for y in 0..height {
            for x in 0..width {
                let location = Location::new_u32(x, y);
                let mut new_superposition: Superposition<T> = Superposition::new(location);
                new_superposition.candidates = entities_lock.clone();
                self.superpositions.push(new_superposition);
            }
        }
    }

    pub fn create_rules(
        &mut self,
        sample: Vec<Vec<T>>,
        item_size: usize,
    ) -> Result<&Vec<Entity<Vec<T>>>, WaveError> {
        let sample_y_len = sample.len();

        if sample_y_len % item_size != 0 {
            return Err(WaveError::InvalidSample);
        }

        if let Some(first_row) = sample.first() {
            if first_row.len() % item_size != 0 {
                return Err(WaveError::InvalidSample);
            }
        }

        let total_vertical_items = sample_y_len / item_size;
        let total_horizontal_items = sample.first().unwrap().len() / item_size;

        let mut all_items: Vec<Vec<Vec<&T>>> = vec![];

        for _ in 0..total_vertical_items {
            let mut new_item: Vec<Vec<&T>> = vec![];

            for _ in 0..total_horizontal_items {
                new_item.push(vec![]);
            }

            all_items.push(new_item);
        }

        for (y, row) in sample.iter().enumerate() {
            for x in 0..row.len() {
                let item_size_float = item_size as f32;
                let item_y = (y as f32 / item_size_float).floor() as usize;
                let item_x = (x as f32 / item_size_float).floor() as usize;
                all_items[item_y][item_x].push(&sample[y][x]);
            }
        }

        self.entities.clear();

        for y in 0..total_vertical_items {
            for x in 0..total_horizontal_items {
                let item = &all_items[y][x];
                let loc = Location::new_usize(x, y);
                let neighbours = loc.all_neighbours();

                for neighbour in neighbours {
                    let neighbour_y_pos = neighbour.0.y as usize;
                    let y_row_opt = all_items
                        .iter()
                        .enumerate()
                        .find(|&e| e.0 == neighbour_y_pos);

                    if let Some(y_row) = y_row_opt {
                        let neighbour_x_pos = neighbour.0.x as usize;
                        let neighbour_item_opt =
                            y_row.1.iter().enumerate().find(|&e| e.0 == neighbour_x_pos);

                        if let Some(neighbour_item) = neighbour_item_opt {
                            let item_copy = item.clone().into_iter().cloned().collect::<Vec<T>>();
                            let neighbour_item_copy = neighbour_item
                                .1
                                .clone()
                                .into_iter()
                                .cloned()
                                .collect::<Vec<T>>();
                            let rule = (neighbour_item_copy, neighbour.1);

                            if let Some(existing_entity) = self.existing_entity(&item_copy) {
                                existing_entity.add_rule(rule);
                            } else {
                                let mut new_entity = Entity::new(item_copy);
                                new_entity.add_rule(rule);
                                self.entities.push(new_entity);
                            }
                        }
                    }
                }
            }
        }

        Ok(&self.entities)
    }

    fn existing_entity(&mut self, item: &Vec<T>) -> Option<&mut Entity<Vec<T>>> {
        self.entities.iter_mut().find(|ent| ent.item == *item)
    }
}
