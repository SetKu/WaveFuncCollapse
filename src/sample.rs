use std::collections::HashMap;
use rand::thread_rng;

pub struct Sample<T> {
    source_map: HashMap<u16, T>,
    table: Vec<(u16, Location)>,
}

impl<T> Sample<T> {
    // Expects a sample in the following format:
    // SCLCS
    // SSCSS
    // CSSSC
    pub fn from_str(sample: String) -> Sample<char> {
        let mut map: HashMap<u16, char> = HashMap::new();
        let mut parsed: Vec<(u16, Location)> = vec![];
        parsed.reserve(sample.len());

        for (y, line) in sample.lines().enumerate() {
            for (x, ch) in line.chars().filter(|c| !c.is_whitespace()).enumerate() {
                let loc = Location::new(x as f32, y as f32);
                
                for (key, val) in &map {
                    if *val == ch {
                        parsed.push((*key, loc.clone()));
                        continue;
                    } 
                }

                let id = 0u16;
                map.insert(id, ch);
                parsed.push((id, loc));
            }
        }

        Sample { source_map: map, table: parsed }
    }
}
