mod error;
pub mod location;

use location::{Location, Direction};

use std::collections::HashMap;
use std::clone::Clone;
use rand::thread_rng;

pub struct Collapser<S> {
    superpos_list: Vec<Superpos>,
    sample: Option<Sample<S>>,
    rules: Vec<Rule>,
    pub use_transforms: bool,
}

impl<S> Collapser<S> {
    pub fn new() -> Self {
        Self {
            superpos_list: vec![],
            sample: None,
            rules: vec![],
            use_transforms: true,
        }
    }

    pub fn analyze(&mut self, sample: Sample<S>) {
        self.sample = Some(sample);
        self.rules.clear();

        for (id, loc) in &self.sample.as_ref().unwrap().data {
            for nb_loc in loc.positive_neighbours() {
                if let Some((nb_id, _)) = self.sample
                    .as_ref()
                    .unwrap()
                    .data
                    .iter()
                    .find(|i| i.1 == nb_loc) { 
                    let all = if self.use_transforms {
                        vec![
                            nb_loc.rotate(90.0, Location::zero(), 2),
                            nb_loc.rotate(180.0, Location::zero(), 2),
                            nb_loc.rotate(270.0, Location::zero(), 2),
                            nb_loc,
                        ]
                    } else {
                        vec![ nb_loc, ]
                    };

                    for rot_loc in all {
                       let dir = loc.relative_direction(rot_loc);
                       let rule = Rule::new(*id, *nb_id, dir);
                       self.rules.push(rule); 
                    }
                }
            } 
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    root_id: u16,
    nb_id: u16,
    dir: Direction,
}

impl Rule {
    pub fn new(root_id: u16, nb_id: u16, dir: Direction) -> Self { Self { root_id, nb_id, dir } }
}

#[derive(Clone, Debug)]
struct Superpos {
   loc: Location,
   pot: Vec<u16>,
}

impl Superpos {
    fn new(loc: Location, pot: Vec<u16>) -> Self { Self { loc, pot } }

    fn is_collapsed(&self) -> bool { self.pot.len() == 1 }    
}

pub struct Sample<T> {
    source_map: HashMap<u16, T>,
    data: Vec<(u16, Location)>,
}

impl<T> Sample<T> {
    // Expects a sample in the following format:
    //    SCLCS
    //    SSCSS
    //    CSSSC
    pub fn from_str(sample: String) -> Sample<char> {
        let mut map: HashMap<u16, char> = HashMap::new();
        let mut parsed: Vec<(u16, Location)> = vec![];
        parsed.reserve(sample.len());

        let mut next_id = 0u16;
        for (y, line) in sample.lines().enumerate() {
            for (x, ch) in line.chars().filter(|c| !c.is_whitespace()).enumerate() {
                let loc = Location::new(x as f64, y as f64);
               
                let mut cont = true;
                for (key, val) in &map {
                    if *val == ch {
                        parsed.push((*key, loc.clone()));
                        cont = false;
                        break;
                    } 
                }

                if !cont {
                    continue;
                }

                map.insert(next_id, ch);
                parsed.push((next_id, loc));
                next_id += 1;
            }
        }

        Sample { source_map: map, data: parsed }
    }
}

#[cfg(test)]
mod tests {
    use super::{Collapser, Sample};

    #[test]
    fn analysis() {
        let ex = "SCL".to_string();
        let sample = Sample::<char>::from_str(ex); 
        let mut collapser = Collapser::new(); 
        collapser.use_transforms = false;
        collapser.analyze(sample);
        assert_eq!(collapser.rules.len(), 4, "Analysis failed. Rules: {:#?}", collapser.rules);
    }
}
