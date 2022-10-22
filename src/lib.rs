use std::clone::Clone;
use std::rc::Rc;
use std::hash::{Hash, Hasher, self};
use rand::thread_rng;

mod errors;
use errors::*;

#[derive(Debug, PartialEq, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
  fn new(x: i32, y: i32) -> Self {
      Location { x: x, y: y }
  }

  // Returns the top, right, down, left neighbours to a specified location.
  fn pos_orthogonal_neighbours(&self) -> [Option<(Location, Direction)>; 4] {
      use Direction::*;
      [
          if self.y > 0 { Some((Self::new(self.x, self.y - 1), Up)) } else { None }, 
          Some((Self::new(self.x + 1, self.y), Right)), 
          Some((Self::new(self.x, self.y + 1), Down)), 
          if self.x > 0 { Some((Self::new(self.x - 1, self.y), Left)) } else { None },
      ]
  }

  fn diagonal_neighbours(&self) -> [Option<(Location, Direction)>; 4] {
      use Direction::*;
      [
          if self.y > 0 && self.x > 0 { Some((Self::new(self.x - 1, self.y - 1), UpLeft)) } else { None }, 
          if self.y > 0 { Some((Self::new(self.x + 1, self.y - 1), UpRight)) } else { None },  
          Some((Self::new(self.x + 1, self.y + 1), DownRight)), 
          if self.x > 0 { Some((Self::new(self.x - 1, self.y + 1), DownLeft)) } else { None },
      ]
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
pub struct Entity {
    hash: u64,
    rules: Vec<(u64, u64, Direction)>,
    weight: i32,
}

// Generic syntax for implementations: https://is.gd/gYBL5c
impl Entity {
    fn new(h: u64) -> Self {
        Entity { hash: h, rules: vec![], weight: 1 }
    }

    // Adds a rule only if it doesn't already existing in validations.
    fn add_rule(&mut self, v: (u64, u64, Direction)) {
        let mut iter = self.rules.iter();
        let existing_val = iter.find(|&e| e.0 == v.0 && e.1 == v.1 && e.2 == v.2);
        
        if None == existing_val {
            self.rules.push(v);
        }
    }

    fn increment_weight(&mut self, x: i32) {
        self.weight += x;
    }
}


#[derive(Debug, Clone)]
struct Superposition {
    location: Location,
    candidates: Vec<Rc<Entity>>,
}

impl Superposition {
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


pub struct Coordinator {
    superpositions: Vec<Superposition>,
    entities: Vec<Entity>,
    pub use_diagonals: bool,
    pub use_weights: bool,
    pub use_transforms: bool,
}

// Anonymous lifetimes ('_) are just syntax to indicate that the lifetime is implied.
// https://is.gd/Qt1zJH
impl Coordinator {
    pub fn new() -> Self {
        Coordinator { 
            superpositions: vec![], 
            entities: Vec::new(),
            use_diagonals: true,
            use_weights: true,
            use_transforms: true,
        }
    }

    pub fn create_rules<T>(&mut self, sample: Vec<Vec<T>>, item_size: u32) -> Result<&Vec<Entity>, WaveError> where T: Hash {
        for y in 0..sample.len() {
            for x in 0..sample[y].len() {

            }
        }

        Ok(&self.entities)
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

