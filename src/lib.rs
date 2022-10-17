#[derive(Debug, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

type OptionalLoc = Option<Location>;

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Location { x: x, y: y }
    }

    // Returns the top, right, down, left neighbours to a specified location.
    fn orthogonal_neighbours(&self) -> (OptionalLoc, OptionalLoc, OptionalLoc, OptionalLoc) {
        (
            if self.y > 0 { Some(Self::new(self.x, self.y - 1)) } else { None }, 
            Some(Self::new(self.x + 1, self.y)), 
            Some(Self::new(self.x, self.y + 1)), 
            if self.x > 0 { Some(Self::new(self.x - 1, self.y)) } else { None },
        )
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Loc({}, {})", self.x, self.y)
    }
}

// No defined size.
#[derive(Debug)]
struct Superposition {
    location: Location,
    candidates: Vec<Entity>,
}

impl Superposition {
    fn new(l: Location) -> Self {
        Superposition { location: l, candidates: vec![] }
    }

    fn is_collapsed(&self) -> bool {
        return self.candidates.len() == 1;
    }
}

#[derive(Debug, std::clone::Clone, PartialEq)]
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

#[derive(Debug, std::clone::Clone)]
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
        let existingVal = iter.find(|&e| e.0 == v.0 && e.1 == v.1 && e.2 == v.2);
        
        if None == existingVal {
            self.validations.push(v);
        }
    }
}



#[derive(Debug)]
pub struct Coordinator {
    superpositions: Vec<Box<Superposition>>,
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
                // Location is consumed by the superposition.
                let p = Superposition::new(tmp_loc);
                let neighbours = p.location.orthogonal_neighbours();

                use Direction::*;

                // Up Rule Analysis
                if let Some(loc) = neighbours.0 {
                    let mut tmp_copy = lines.clone();

                    if let Some((_, found_line)) = tmp_copy.find(|&e| e.0 == loc.y) {
                        if let Some(found_ch) = found_line.chars().nth(loc.x) {
                            // Use the existing entity.
                            let existing_entity = self.existing_entity(&found_ch);
                            let validation = (c, found_ch, Up);

                            if let Some(found_ent) = existing_entity {
                                found_ent.add_unknown_validation(validation);
                                continue;
                            }

                            // Create a new entity.
                            let mut new_ent = Entity::new(c.clone());
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
}

#[cfg(test)]
mod tests {
    use crate::Location;

    #[test]
    fn orthogonal_location_members_works() {
        let m = Location::new(1, 1).orthogonal_neighbours();
        let l = |x: usize, y: usize| Location::new(x, y);
        assert_eq!(m, (Some(l(1, 0)), Some(l(2, 1)), Some(l(1, 2)), Some(l(0, 1))));
    }
}