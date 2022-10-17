#[derive(Debug, PartialEq)]
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
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Loc({}, {})", self.x, self.y)
    }
}

// No defined size.
#[derive(Debug)]
#[allow(unused)]
struct Superposition {
    location: Location,
    candidates: Vec<Box<Entity>>,
}

#[allow(unused)]
impl Superposition {
    fn new(l: Location) -> Self {
        Superposition { location: l, candidates: vec![] }
    }

    fn is_collapsed(&self) -> bool {
        return self.candidates.len() == 1;
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
        for x in 0..self.width {
            for y in 0..self.height {
                let loc = Location::new(x as usize, y as usize);
                let mut superpos = Superposition::new(loc);
                superpos.candidates = self.entities.clone().into_iter().map(|e| Box::new(e)).collect();
                self.superpositions.push(superpos);
            }
        }
    }

    pub fn collapse_once(&mut self) {
        for superpos in &self.superpositions {
            // Find superpos with lowest entropy and collapse it.
            // Otherwise choose among those with the same entropy using weights.
        }
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

        println!("{:#?}", self.entities);
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
}