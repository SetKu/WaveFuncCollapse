use std::rc::Rc;
use rand::seq::SliceRandom;
use rand::thread_rng;
pub mod errors;
use errors::*;

#[derive(Debug, PartialEq, std::clone::Clone)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Location { x: x, y: y }
    }

    // Returns the top, right, down, left neighbours to a specified location.
    fn orthogonal_neighbours(&self) -> [Option<(Location, Direction)>; 4] {
        use Direction::*;
        [
            if self.y > 0 { Some((Self::new(self.x, self.y - 1), Up)) } else { None }, 
            Some((Self::new(self.x + 1, self.y), Right)), 
            Some((Self::new(self.x, self.y + 1), Down)), 
            if self.x > 0 { Some((Self::new(self.x - 1, self.y), Left)) } else { None },
        ]
    }

    // Here would be a good place to begin to implement diagonal neighbours analysis.
    // As a thought, we could only assert in this case, having more info, that an
    // entity is valid when next to a tile if its also next to one or both of its neighbours.
    // The issue, I could immagine, however, with this more 'accurate' approach is that
    // It would reduce the number of potential combinations.
    //
    // ... Circling back to this, isn't this already handled by orthogonal analysis?

    fn diagonal_neighbours(&self) -> [Option<(Location, Direction)>; 4] {
        use Direction::*;
        [
            if self.y > 0 && self.x > 0 { Some((Self::new(self.x - 1, self.y - 1), UpLeft)) } else { None }, 
            if self.y > 0 { Some((Self::new(self.x + 1, self.y - 1), UpRight)) } else { None },  
            Some((Self::new(self.x + 1, self.y - 1), DownRight)), 
            if self.x > 0 { Some((Self::new(self.x - 1, self.y - 1), DownLeft)) } else { None },
        ]
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Loc({}, {})", self.x, self.y)
    }
}

// No defined size.
#[derive(Debug, std::clone::Clone)]
#[allow(unused)]
struct Superposition {
    location: Location,
    candidates: Vec<Rc<Entity>>,
}

#[allow(unused)]
impl Superposition {
    fn new(l: Location) -> Self {
        Superposition { location: l, candidates: vec![] }
    }

    fn is_collapsed(&self) -> bool {
        return self.candidates.len() == 1;
    }

    fn entropy(&self) -> usize {
        return self.candidates.len();
    }

    fn candidate_id_strings(&self) -> Vec<String> {
        self.candidates.iter().map(|a| a.identifier.to_string()).collect::<Vec<String>>()
    }
}

#[derive(Debug, std::clone::Clone, PartialEq)]
#[allow(unused)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

#[derive(Debug, std::clone::Clone, PartialEq)]
struct Entity {
    identifier: char,
    validations: Vec<(char, char, Direction)>,
    weight: f32,
}

// Generic syntax for implementations: https://is.gd/gYBL5c
impl Entity {
    fn new(id: char) -> Self {
        Entity { identifier: id, validations: vec![], weight: 1.0 }
    }

    // Adds a validation only if it doesn't already existing in validations.
    fn add_unknown_validation(&mut self, v: (char, char, Direction)) {
        let mut iter = self.validations.iter();
        let existing_val = iter.find(|&e| e.0 == v.0 && e.1 == v.1 && e.2 == v.2);
        
        if None == existing_val {
            self.validations.push(v);
        }
    }

    fn increment_weight(&mut self, x: &f32) {
        self.weight += *x;
    }
}



#[derive(Debug)]
pub struct Coordinator {
    superpositions: Vec<Superposition>,
    width: u32,
    height: u32,
    entities: Vec<Entity>,
}

// Anonymous lifetimes ('_) are just syntax to indicate that the lifetime is implied.
// https://is.gd/Qt1zJH
impl Coordinator {
    pub fn new() -> Self {
        Coordinator { superpositions: vec![], width: 0, height: 0, entities: Vec::new() }
    }

    pub fn populate_superpositions(&mut self) {
        self.superpositions.clear();

        // Reference counters in Rust: https://doc.rust-lang.org/book/ch15-04-rc.html
        let candidate_refs: Vec<Rc<Entity>> = self.entities.clone().into_iter().map(|c| Rc::new(c)).collect();

        for x in 0..self.width {
            for y in 0..self.height {
                let loc = Location::new(x as usize, y as usize);
                let mut superpos = Superposition::new(loc);
                superpos.candidates = candidate_refs.clone();
                self.superpositions.push(superpos);
            }
        }
    }

    pub fn collapse_once(&mut self) -> Result<(), WaveError> {
        let mut lowests: Vec<&mut Superposition> = Vec::new();

        for superpos in &mut self.superpositions {
            if superpos.is_collapsed() {
                continue;
            }

            // Find superpos with lowest entropy and collapse it.
            // Otherwise choose among those with the same entropy using weights.
            if lowests.len() == 0 {
                lowests.push(superpos);
                continue;
            }

            let old = lowests.first().unwrap().entropy();
            let new = superpos.entropy();

            if old > new {
                lowests.clear();
            }

            if old >= new {
                lowests.push(superpos);
            }
        }

        if lowests.len() == 0 {
            return Err(WaveError::no_uncollapsed_superpositions());
        }

        let mut rng = thread_rng();
        // Choose a random superposition among those with equal low entropy.
        let chosen_sp = lowests.choose_mut(&mut rng).unwrap();

        if chosen_sp.candidates.is_empty() {
            // This is a contradiction!
            return Err(WaveError::contradiction());
        }

        // Choose a weighted random entity from the possible candidates for the superposition.
        let entity = chosen_sp.candidates.choose_weighted(&mut rng, |c| c.weight).unwrap().clone();
        // Clear the superposition's entities and then add the chosen entity as the only one.
        chosen_sp.candidates.clear();
        chosen_sp.candidates.push(entity);

        let neighbours = chosen_sp.location.orthogonal_neighbours();
        let validations = chosen_sp.candidates.first().unwrap().validations.clone();

        // Start propogating ripples!
        for neighbour in &neighbours {
            if let Some((pos, dir)) = neighbour {
                if let Some(found_sp) = self.superposition_for(&pos) {
                    let mut indexes_removed = 0;

                    for i in 0..found_sp.candidates.len() {
                        let mut found_valid = false;

                        // Try to match the candidate to a valid rule.
                        for validation in &validations {
                            let candidate = &found_sp.candidates[i - indexes_removed];

                            if candidate.identifier == validation.1 {
                                if validation.2 == *dir {
                                    // This is a valid candidate.
                                    found_valid = true;
                                    break;
                                }
                            }
                        }

                        if !found_valid {
                            // The candidate is invalid at this point.
                            found_sp.candidates.remove(i - indexes_removed);
                            indexes_removed += 1;
                        }
                    }
                }
            }
        }

        return Ok(());
    }

    fn superposition_for(&mut self, pos: &Location) -> Option<&mut Superposition> {
        for sp in self.superpositions.iter_mut() {
            if sp.location == *pos {
                return Some(sp);
            }
        }

        None
    }

    fn all_collapsed(&self) -> bool {
        for sp in &self.superpositions {
            if !sp.is_collapsed() {
                return false;
            }
        }

        return true;
    }

    pub fn collapse_all(&mut self, show_output: bool) -> Result<(), WaveError> {
        let threshold = 1000;
        let mut failures = 0;

        let mut iteration = 0;
        while !self.all_collapsed() {
            let res = self.collapse_once();

            if let Err(_) = res {
                failures += 1;

                if failures > threshold - 1 {
                    return Err(WaveError::threshhold(threshold));
                }

                // The algorithm needs to be restarted (keeping 
                // the same rules) to try for a different outcome.
                self.populate_superpositions();

                iteration = 0;
                assert!(false);
            }

            if show_output {
                // ANSI Escape Codes: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
                let rep = self.get_rep();
                println!("\x1B7Attempt {} Iteration {}:\n{}", failures + 1, iteration + 1, &rep);

                // for i in 0..(rep.lines().count() + 2) {
                //     print!("\x1B[1A\x1B[0G\x1B[J");
                // }

                // print!("\x1B7");

                std::thread::sleep(std::time::Duration::new(0, 6u32 * 10u32.pow(8)));
            }

            iteration += 1;
        }

        Ok(())
    }

    pub fn process_sample(&mut self, s: &String) {
        let lines = s.lines().enumerate();
        let lines_copy = lines.clone();

        self.height = lines.clone().count() as u32;
        
        if self.height != 0 {
            self.width = lines.clone().next().unwrap().1.chars().count() as u32;
        }

        for (iy, line) in lines_copy {
            // Ampersand in paramter simply indicates an immediate derefence.
            // https://is.gd/iXvDC0
            for (ix, c) in line.chars().enumerate() {
                let tmp_loc = Location::new(ix, iy);
                let neighbours = tmp_loc.orthogonal_neighbours();
                let unwrapped_neighbours = neighbours.into_iter().filter_map(|nb| nb);

                for (loc, dir) in unwrapped_neighbours {
                    let mut tmp_copy = lines.clone();

                    // Find the line.
                    if let Some((_, found_line)) = tmp_copy.find(|&e| e.0 == loc.y) {
                        // Find the character.
                        if let Some(found_ch) = found_line.chars().nth(loc.x) {
                            let existing_entity = self.existing_entity(&c);
                            let validation = (c, found_ch, dir);
    
                            // Use the existing entity.
                            if let Some(found_ent) = existing_entity {
                                found_ent.add_unknown_validation(validation);
                                found_ent.increment_weight(&1.0);
                                continue;
                            }
    
                            // Create a new entity.
                            let mut new_ent = Entity::new(c);
                            new_ent.validations.push(validation);
                            self.entities.push(new_ent); // Consumption occurs!
                        }
                    }
                }
            }
        }
    }

    fn existing_entity(&mut self, id: &char) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|ent| ent.identifier == *id)
    }

    pub fn entities_found(&self) -> usize {
        self.entities.len()
    }

    pub fn superpositions_count(&self) -> usize {
        self.superpositions.len()
    }

    pub fn get_rep(&self) -> String {
        let mut rep = String::new();
        let mut sps_copy = self.superpositions.clone();
        sps_copy.sort_by_key(|sp| sp.location.x);
        let mut sps_organized: Vec<Vec<Superposition>> = Vec::new();

        for _ in 0..self.height as usize {
            sps_organized.push(Vec::new());
        }

        for sp in sps_copy {
            sps_organized[sp.location.y].push(sp);
        }

        for line in sps_organized {
            for sp in line {
                let chars_iter = sp.candidates.into_iter().map(|c| c.identifier.to_string());
                let cand_strs = chars_iter.collect::<Vec<String>>();
                let str = format!("({}) ", cand_strs.concat());
                rep.push_str(&str);
            }

            rep.push_str("\n");
        }
        
        return rep;
    }

    pub fn set_dimensions(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }
}

#[cfg(test)]
mod tests {
    use crate::Location;
    use crate::Coordinator;
    use crate::Entity;

    #[test]
    fn orthogonal_location_members_works() {
        let m = Location::new(1, 1).orthogonal_neighbours();
        let l = |x: usize, y: usize| Location::new(x, y);
        use crate::Direction::*;
        assert_eq!(m, [
            Some((l(1, 0), Up)), 
            Some((l(2, 1), Right)), 
            Some((l(1, 2), Down)), 
            Some((l(0, 1), Left))
        ]);
    }

    // Tests whether getting an existing entity affects the entities vector itself:
    // It shouldn't.
    #[test]
    fn existing_entity_independent() {
        let mut c = Coordinator::new();
        c.entities.push(Entity::new('A'));
        let copy = c.entities.clone();
        assert_eq!(c.existing_entity(&'A').is_some(), true);
        assert_eq!(c.existing_entity(&'B').is_some(), false);
        assert_eq!(copy, c.entities);
    }

    #[test]
    fn entity_count_is_correct() {
        let s = "LCS";
        let mut c = Coordinator::new();
        c.process_sample(&s.to_string());
        assert_eq!(c.entities.len(), 3);
    }

    #[test]
    fn populate_superpositions_works() {
        let s = "LCS";
        let mut c = Coordinator::new();
        c.process_sample(&s.to_string());
        c.populate_superpositions();
        assert_eq!(c.superpositions_count(), 3);
    }

    #[test]
    fn collapse_once_works() {
        let s = "LCS";
        let mut c = Coordinator::new();
        c.process_sample(&s.to_string());
        c.populate_superpositions();
        let err = c.collapse_once().is_err();
        assert!(!err);
        assert!(c.superpositions.iter().any(|x| x.is_collapsed()));
    }
}