mod error;
pub mod location;

use error::WaveError;
use location::{Location, Direction};

use std::collections::HashMap;
use std::clone::Clone;
use rand::thread_rng;

pub struct Collapser<S> {
    superpos_list: Vec<Superpos>,
    sample: Option<Sample<S>>,
}

impl<S> Collapser<S> {
    pub fn new() -> Self {
        Self {
            superpos_list: vec![],
            sample: None,
        }
    }

    pub fn analyze(&mut self, sample: Sample<S>) {
        self.sample = Some(sample);

        for (id, loc) in &self.sample.as_ref().unwrap().data {
            for nb_loc in loc.positive_neighbours() {
                if let Some(nb) = self.sample
                    .as_ref()
                    .unwrap()
                    .data
                    .iter()
                    .find(|i| i.1 == nb_loc) {
                     
                }
            } 
        }
    }
}

pub struct Rule {
    dir: Direction,
    nb_id: u16,
}

#[derive(Clone, Debug)]
pub struct Superpos {
   loc: Location,
   pot: Vec<u16>,
}

impl Superpos {
    pub fn new(loc: Location, pot: Vec<u16>) -> Self {
        Self { loc, pot }
    }
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
