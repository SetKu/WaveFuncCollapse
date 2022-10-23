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
    
    // pub fn process_sample(&mut self, s: String) {
        //     let lines = s.lines().enumerate();
        //     let lines_copy = lines.clone();
        
        //     for (iy, line) in lines_copy {
            //         // Ampersand in paramter simply indicates an immediate derefence.
            //         // https://is.gd/iXvDC0
            //         for (ix, c) in line.chars().enumerate() {
                //             let tmp_loc = Location::new(ix as i32, iy as i32);
                //             let mut neighbours = tmp_loc.pos_orthogonal_neighbours().to_vec();
                
                //             if self.use_diagonals {
                    //                 neighbours.append(&mut tmp_loc.diagonal_neighbours().to_vec());
                    //             }
                    
                    //             let unwrapped_neighbours = neighbours.into_iter().filter_map(|nb| nb);
                    
                    //             for (loc, dir) in unwrapped_neighbours {
                        //                 let mut tmp_copy = lines.clone();
                        
                        //                 // Find the line.
                        //                 if let Some((_, found_line)) = tmp_copy.find(|&e| e.0 == loc.y) {
                            //                     // Find the character.
                            //                     if let Some(found_ch) = found_line.chars().nth(loc.x) {
                                //                         let existing_entity = self.existing_entity(&c);
                                //                         let validation = (c, found_ch, dir);
                                
                                //                         // Use the existing entity.
                                //                         if let Some(found_ent) = existing_entity {
                                    //                             found_ent.add_unknown_validation(validation);
                                    //                             found_ent.increment_weight(&1.0);
                                    //                             continue;
                                    //                         }
                                    
                                    //                         // Create a new entity.
                                    //                         let mut new_ent = Entity::new(c);
                                    //                         new_ent.rules.push(validation);
                                    //                         self.entities.push(new_ent); // Consumption occurs!
                                    //                     }
                                    //                 }
                                    //             }
                                    //         }
                                    //     }
                                    
                                    // Rotate and reflect the rules in all 
                                    // validations if transforms are enabled!
                                    //     if self.use_transforms {
                                        //         for ent in &mut self.entities {
                                            //             let vals_copy = ent.rules.clone();
                                            //             for val in vals_copy {
                                                //                 // Mirror/reflect the validation.
                                                //                 let new1 = (val.1, val.0, val.2.clone());
                                                //                 ent.add_rule(new1);
                                                
                                                //                 // Get the rotated validations.
                                                //                 for i in 0..4 {
                                                    //                     let mut dir = val.2.clone();
                                                    
                                                    //                     for _ in 0..i {
                                                        //                         dir = dir.rotate_90();
                                                        //                     }
                                                        
                                                        //                     let new2 = (val.0, val.1, dir);
                                                        //                     ent.add_rule(new2);
                                                        //                 }
                                                        //             }
                                                        //         }
                                                        //     } 
                                                        // }
                                                        
                                                        // fn populate_superpositions(&mut self) {
                                                            //     self.superpositions.clear();
                                                            
                                                            //     // Reference counters in Rust: https://doc.rust-lang.org/book/ch15-04-rc.html
                                                            //     let candidate_refs: Vec<Rc<Entity<T>>> = self.entities.clone().into_iter().map(|c| Rc::new(c)).collect();
                                                            
                                                            //     for x in 0..self.width {
                                                                //         for y in 0..self.height {
                                                                    //             let loc = Location::new(x as usize, y as usize);
                                                                    //             let mut superpos = Superposition::new(loc);
                                                                    //             superpos.candidates = candidate_refs.clone();
                                                                    //             self.superpositions.push(superpos);
                                                                    //         }
                                                                    //     }
                                                                    
                                                                    //     self.entities_lock = Some(candidate_refs);
                                                                    // }
                                                                    
                                                                    // pub fn collapse_once(&mut self) -> Result<(), WaveError> {
                                                                        //     let mut lowests: Vec<&mut Superposition> = Vec::new();
                                                                        
                                                                        //     for superpos in &mut self.superpositions {
                                                                            //         if superpos.is_collapsed() {
                                                                                //             continue;
                                                                                //         }
                                                                                
                                                                                //         // Find superpos with lowest entropy and collapse it.
                                                                                //         // Otherwise choose among those with the same entropy using weights.
                                                                                //         if lowests.len() == 0 {
                                                                                    //             lowests.push(superpos);
                                                                                    //             continue;
                                                                                    //         }
                                                                                    
                                                                                    //         let old = lowests.first().unwrap().entropy();
                                                                                    //         let new = superpos.entropy();
                                                                                    
                                                                                    //         if old > new {
                                                                                        //             lowests.clear();
                                                                                        //         }
                                                                                        
                                                                                        //         if old >= new {
                                                                                            //             lowests.push(superpos);
                                                                                            //         }
                                                                                            //     }
                                                                                            
                                                                                            //     if lowests.len() == 0 {
                                                                                                //         return Err(WaveError::no_uncollapsed_superpositions());
                                                                                                //     }
                                                                                                
                                                                                                //     let mut rng = thread_rng();
                                                                                                //     // Choose a random superposition among those with equal low entropy.
                                                                                                //     let chosen_sp = lowests.choose_mut(&mut rng).unwrap();
                                                                                                
                                                                                                //     if chosen_sp.candidates.is_empty() {
                                                                                                    //         // This is a contradiction!
                                                                                                    //         return Err(WaveError::contradiction());
                                                                                                    //     }
                                                                                                    
                                                                                                    //     // Choose a weighted random entity from the possible candidates for the superposition.
                                                                                                    //     let entity = {
                                                                                                        //         if self.use_weights {
                                                                                                            //             chosen_sp.candidates.choose_weighted(&mut rng, |c| c.weight).unwrap().clone()
                                                                                                            //         } else {
                                                                                                                //             chosen_sp.candidates.choose(&mut rng).unwrap().clone()
                                                                                                                //         }
                                                                                                                //     };
                                                                                                                
                                                                                                                //     // Clear the superposition's entities and then add the chosen entity as the only one.
                                                                                                                //     chosen_sp.candidates.clear();
                                                                                                                //     chosen_sp.candidates.push(entity);
                                                                                                                
                                                                                                                //     let mut neighbours = chosen_sp.location.orthogonal_neighbours().to_vec();
                                                                                                                
                                                                                                                //     if self.diagonals {
                                                                                                                    //         neighbours.append(&mut chosen_sp.location.diagonal_neighbours().to_vec());
                                                                                                                    //     }
                                                                                                                    
                                                                                                                    //     let validations = chosen_sp.candidates.first().unwrap().validations.clone();
                                                                                                                    
                                                                                                                    //     // Start propogating ripples!
                                                                                                                    //     for neighbour in &neighbours {
                                                                                                                        //         if let Some((pos, dir)) = neighbour {
                                                                                                                            //             if let Some(found_sp) = self.superposition_for(&pos) {
                                                                                                                                //                 if found_sp.is_collapsed() {
                                                                                                                                    //                     // No need to reduce this superpositions entropy.
                                                                                                                                    //                     continue;
                                                                                                                                    //                 }
                                                                                                                                    
                                                                                                                                    //                 let mut indexes_removed = 0;
                                                                                                                                    
                                                                                                                                    //                 for i in 0..found_sp.candidates.len() {
                                                                                                                                        //                     let mut found_valid = false;
                                                                                                                                        
                                                                                                                                        //                     // Try to match the candidate to a valid rule.
                                                                                                                                        //                     for validation in &validations {
                                                                                                                                            //                         let candidate = &found_sp.candidates[i - indexes_removed];
                                                                                                                                            
                                                                                                                                            //                         if candidate.identifier == validation.1 {
                                                                                                                                                //                             if validation.2 == *dir {
                                                                                                                                                    //                                 // This is a valid candidate.
                                                                                                                                                    //                                 found_valid = true;
                                                                                                                                                    
                                                                                                                                                    //                                 break;
                                                                                                                                                    //                             }
                                                                                                                                                    //                         }
                                                                                                                                                    //                     }
                                                                                                                                                    
                                                                                                                                                    //                     if !found_valid {
                                                                                                                                                        //                         // The candidate is invalid at this point.
                                                                                                                                                        //                         found_sp.candidates.remove(i - indexes_removed);
                                                                                                                                                        //                         indexes_removed += 1;
                                                                                                                                                        //                     }
                                                                                                                                                        //                 }
                                                                                                                                                        //             }
                                                                                                                                                        //         }
                                                                                                                                                        //     }
                                                                                                                                                        
                                                                                                                                                        //     return Ok(());
                                                                                                                                                        // }
                                                                                                                                                        
                                                                                                                                                        // fn superposition_for(&mut self, pos: &Location) -> Option<&mut Superposition<T>> {
                                                                                                                                                            //     for sp in self.superpositions.iter_mut() {
                                                                                                                                                                //         if sp.location == *pos {
                                                                                                                                                                    //             return Some(sp);
                                                                                                                                                                    //         }
                                                                                                                                                                    //     }
                                                                                                                                                                    
                                                                                                                                                                    //     None
                                                                                                                                                                    // }
                                                                                                                                                                    
                                                                                                                                                                    // fn all_collapsed(&self) -> bool {
                                                                                                                                                                        //     for sp in &self.superpositions {
                                                                                                                                                                            //         if !sp.is_collapsed() {
                                                                                                                                                                                //             return false;
                                                                                                                                                                                //         }
                                                                                                                                                                                //     }
                                                                                                                                                                                
                                                                                                                                                                                //     return true;
                                                                                                                                                                                // }
                                                                                                                                                                                
                                                                                                                                                                                // pub fn collapse_all(&mut self, show_output: bool, interval: std::time::Duration) -> Result<(), WaveError> {
                                                                                                                                                                                    //     let threshold = (self.width * self.height).pow(2);
                                                                                                                                                                                    
                                                                                                                                                                                    //     if show_output {
                                                                                                                                                                                        //         println!("Set attempt threshold of {}.", threshold);
                                                                                                                                                                                        //     }
                                                                                                                                                                                        
                                                                                                                                                                                        //     let mut failures = 0;
                                                                                                                                                                                        //     let mut iteration = 0;
                                                                                                                                                                                        //     while !self.all_collapsed() {
                                                                                                                                                                                            //         let res = self.collapse_once();
                                                                                                                                                                                            
                                                                                                                                                                                            //         if let Err(_) = res {
                                                                                                                                                                                                //             failures += 1;
                                                                                                                                                                                                
                                                                                                                                                                                                //             if failures > threshold - 1 {
                                                                                                                                                                                                    //                 return Err(WaveError::threshhold(threshold));
                                                                                                                                                                                                    //             }
                                                                                                                                                                                                    
                                                                                                                                                                                                    //             // The algorithm needs to be restarted (keeping 
                                                                                                                                                                                                        //             // the same rules) to try for a different outcome.
                                                                                                                                                                                                        //             self.populate_superpositions();
                                                                                                                                                                                                        
                                                                                                                                                                                                        //             iteration = 0;
                                                                                                                                                                                                        //         }
                                                                                                                                                                                                        
                                                                                                                                                                                                        //         if show_output {
                                                                                                                                                                                                            //             // ANSI Escape Codes: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
                                                                                                                                                                                                            //             let rep = self.get_rep(false);
                                                                                                                                                                                                            //             println!("\x1B7Attempt {} Iteration {}:\n{}", failures + 1, iteration + 1, &rep);
                                                                                                                                                                                                            
                                                                                                                                                                                                            //             // for i in 0..(rep.lines().count() + 2) {
                                                                                                                                                                                                                //             //     print!("\x1B[1A\x1B[0G\x1B[J");
                                                                                                                                                                                                                //             // }
                                                                                                                                                                                                                
                                                                                                                                                                                                                //             // print!("\x1B7");
                                                                                                                                                                                                                
                                                                                                                                                                                                                //             std::thread::sleep(interval);
                                                                                                                                                                                                                //         }
                                                                                                                                                                                                                
                                                                                                                                                                                                                //         iteration += 1;
                                                                                                                                                                                                                //     }
                                                                                                                                                                                                                
                                                                                                                                                                                                                //     Ok(())
                                                                                                                                                                                                                // }
                                                                                                                                                                                                                
                                                                                                                                                                                                                // fn existing_entity(&mut self, id: &char) -> Option<&mut Entity<T>> {
                                                                                                                                                                                                                    //     self.entities.iter_mut().find(|ent| ent.identifier == *id)
                                                                                                                                                                                                                    // }
                                                                                                                                                                                                                    
                                                                                                                                                                                                                    // pub fn entities_found(&self) -> usize {
                                                                                                                                                                                                                        //     self.entities.len()
                                                                                                                                                                                                                        // }
                                                                                                                                                                                                                        
                                                                                                                                                                                                                        // pub fn superpositions_count(&self) -> usize {
                                                                                                                                                                                                                            //     self.superpositions.len()
                                                                                                                                                                                                                            // }
                                                                                                                                                                                                                            
                                                                                                                                                                                                                            // pub fn get_rep(&self, clean: bool) -> String {
                                                                                                                                                                                                                                //     let mut rep = String::new();
                                                                                                                                                                                                                                //     let mut sps_copy = self.superpositions.clone();
                                                                                                                                                                                                                                //     sps_copy.sort_by_key(|sp| sp.location.x);
                                                                                                                                                                                                                                //     let mut sps_organized: Vec<Vec<Superposition>> = Vec::new();
                                                                                                                                                                                                                                
                                                                                                                                                                                                                                //     for _ in 0..self.height as usize {
                                                                                                                                                                                                                                    //         sps_organized.push(Vec::new());
                                                                                                                                                                                                                                    //     }
                                                                                                                                                                                                                                    
                                                                                                                                                                                                                                    //     for sp in sps_copy {
                                                                                                                                                                                                                                        //         sps_organized[sp.location.y].push(sp);
                                                                                                                                                                                                                                        //     }
                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                        //     let entity_lock_count = self.entities_lock.clone().unwrap_or(Vec::new()).len();
                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                        //     for line in sps_organized {
                                                                                                                                                                                                                                            //         for sp in line {
                                                                                                                                                                                                                                                //             let chars_iter = sp.candidates.into_iter().map(|c| c.identifier.to_string());
                                                                                                                                                                                                                                                //             let mut cand_strs = chars_iter.collect::<Vec<String>>();
                                                                                                                                                                                                                                                
                                                                                                                                                                                                                                                //             if !clean {
                                                                                                                                                                                                                                                    //                 for _ in cand_strs.len()..entity_lock_count {
                                                                                                                                                                                                                                                        //                     cand_strs.push(" ".to_string());
                                                                                                                                                                                                                                                        //                 }
                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                        //                 let str = format!("({}) ", cand_strs.concat());
                                                                                                                                                                                                                                                        //                 rep.push_str(&str);
                                                                                                                                                                                                                                                        //                 continue;
                                                                                                                                                                                                                                                        //             }
                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                        //             // Clean branch.
                                                                                                                                                                                                                                                        //             let str = format!("{} ", cand_strs.concat());
                                                                                                                                                                                                                                                        //             rep.push_str(&str);
                                                                                                                                                                                                                                                        //         }
                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                        //         rep.push_str("\n");
                                                                                                                                                                                                                                                        //     }
                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                        //     return rep;
                                                                                                                                                                                                                                                        // }
                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                        // pub fn set_dimensions(&mut self, w: u32, h: u32) {
                                                                                                                                                                                                                                                            //     self.width = w;
                                                                                                                                                                                                                                                            //     self.height = h;
                                                                                                                                                                                                                                                            // }
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        
                                                                                                                                                                                                                                                        