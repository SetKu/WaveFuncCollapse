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

#[derive(Debug)]
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

#[derive(Debug)]
struct Entity {
    contradictions: Vec<(Box<Entity>, Box<Entity>, Direction)>,
    weight: f32,
}

#[derive(Debug)]
pub struct Coordinator {
    superpositions: Vec<Box<Superposition>>
}

impl Coordinator {
    pub fn new() -> Self {
        Coordinator { superpositions: vec![] }
    }

    pub fn process_sample(&mut self, s: &String) {
        let lines = s.lines().enumerate();
        let lines_copy = lines.clone();

        for (ix, line) in lines_copy {
            // Ampersand in paramter simply indicates an immediate derefence.
            // https://is.gd/iXvDC0
            for (iy, c) in line.chars().enumerate() {
                let tmp_loc = Location::new(ix, iy);
                // Location is consumed by the superposition.
                let p = Superposition::new(tmp_loc);
                let neighbours = p.location.orthogonal_neighbours();

                // North
                if let Some(loc) = neighbours.0 {
                    println!("Checking out {}", loc);

                    let mut tmp_copy = lines.clone();

                    if let Some((_, found_line)) = tmp_copy.find(|&e| e.0 == loc.y) {
                        if let Some(found_ch) = found_line.chars().nth(loc.x) {
                            println!("Found character north of L{}C{} '{}'", ix, iy, found_ch);
                        }
                    }
                }

                // println!("{} L{}C{}", c, x, y);
            }
        }

        // println!("{:?}", lines.collect::<Vec<(usize, &str)>>());
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