use std::clone::Clone;
use std::rc::Rc;
use rand::thread_rng;

mod errors;
use errors::*;

#[derive(Debug, PartialEq, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new_i32(x: i32, y: i32) -> Self {
        Location { x: x, y: y }
    }

    fn new_usize(x: usize, y: usize) -> Self {
        Location { x: x as i32, y: y as i32 }
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
pub struct Entity<T> where T: PartialEq + Clone {
    item: T,
    rules: Vec<(T, Direction)>,
    weight: i32,
}

// Generic syntax for implementations: https://is.gd/gYBL5c
impl <T> Entity<T> where T: PartialEq + Clone {
    fn new(i: T) -> Self {
        Entity { item: i, rules: vec![], weight: 1 }
    }
    
    // Adds a rule only if it doesn't already existing in validations.
    fn add_rule(&mut self, v: (T, Direction)) {
        let mut iter = self.rules.iter();
        let existing_val = iter.find(|&e| e.0 == v.0 && e.1 == v.1);
        
        if None == existing_val {
            self.rules.push(v);
        }
    }
    
    fn increment_weight(&mut self, x: i32) {
        self.weight += x;
    }
}


#[derive(Debug, Clone)]
struct Superposition<T> where T: PartialEq + Clone {
    location: Location,
    candidates: Vec<Rc<Entity<T>>>,
}

impl <T> Superposition<T> where T: PartialEq + Clone {
    fn new(l: Location) -> Self {
        Superposition { location: l, candidates: vec![] }
    }
    
    fn is_collapsed(&self) -> bool {
        self.candidates.len() == 1
    }
    
    fn entropy(&self) -> usize {
        self.candidates.len()
    }
}


pub struct Coordinator<T> where T: PartialEq + Clone {
    superpositions: Vec<Superposition<Vec<T>>>,
    entities: Vec<Entity<Vec<T>>>,
    pub use_diagonals: bool,
    pub use_weights: bool,
    pub use_transforms: bool,
}

// Anonymous lifetimes ('_) are just syntax to indicate that the lifetime is implied.
// https://is.gd/Qt1zJH
impl <T> Coordinator<T> where T: PartialEq + Clone {
    pub fn new() -> Self {
        Coordinator { 
            superpositions: vec![], 
            entities: Vec::new(),
            use_diagonals: true,
            use_weights: true,
            use_transforms: true,
        }
    }
    
    pub fn create_rules(&mut self, sample: Vec<Vec<T>>, item_size: usize) -> Result<&Vec<Entity<Vec<T>>>, WaveError> {
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

        for y in 0..sample.len() {
            for x in 0..sample[y].len() {
                let item_size_float = item_size as f32;
                let item_y = (y as f32 / item_size_float).floor() as usize;
                let item_x = (x as f32 / item_size_float).floor() as usize;
                all_items[item_y][item_x].push(&sample[y][x]);
            }
        }

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
                        let neighbour_item_opt = y_row.1
                            .iter()
                            .enumerate()
                            .find(|&e| e.0 == neighbour_x_pos);

                        if let Some(neighbour_item) = neighbour_item_opt {
                            let neighbour_item_copy = neighbour_item.1.clone().into_iter().map(|i| i.clone()).collect::<Vec<T>>();
                            let rule = (neighbour_item_copy, neighbour.1);

                            if let Some(existing_entity) = self.existing_entity(&rule.0) {
                                existing_entity.add_rule(rule);
                            } else {
                                let item_copy = item.clone().into_iter().map(|i| i.clone()).collect::<Vec<T>>();
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