mod error;
pub mod location;

use error::WaveError;
use location::Location;

use std::collections::HashMap;
use std::clone::Clone;
use rand::thread_rng;

pub struct Collapser {
    superpos_list: Vec<Superpos>,
}

impl Collapser {
    pub fn new(superpos_list: Vec<Superpos>) -> Self { Self { superpos_list } }

    pub fn analyze<S>(sample: Sample<S>) {

    }
}

#[derive(Clone)]
pub struct Superpos {
   loc: Location,
   candidates: Vec<u16>,
}

impl Superpos {
    pub fn new(loc: Location, candidates: Vec<u16>) -> Self {
        Self { loc, candidates }
    }
}

#[derive(Debug)]
pub struct Sample<T> {
    source_map: HashMap<u16, T>,
    table: Vec<(u16, Location)>,
}

impl<T> Sample<T> {
    // Expects a sample in the following format:
    //   SCLCS
    //   SSCSS
    //   CSSSC
    pub fn from_str(sample: String) -> Sample<char> {
        let mut map: HashMap<u16, char> = HashMap::new();
        let mut parsed: Vec<(u16, Location)> = vec![];
        parsed.reserve(sample.len());

        let mut next_id = 0u16;
        for (y, line) in sample.lines().enumerate() {
            for (x, ch) in line.chars().filter(|c| !c.is_whitespace()).enumerate() {
                let loc = Location::new(x as f32, y as f32);
               
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

        Sample { source_map: map, table: parsed }
    }
}
