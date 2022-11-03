mod error;
pub mod location;

use location::{Location, Direction};
use error::WaveError;

use std::collections::HashMap;
use std::clone::Clone;
use rand::thread_rng;
use rand::prelude::*;

pub struct Collapser<S> {
    superpos_list: Vec<Superpos>,
    sample: Option<Sample<S>>,
    rules: Vec<Rule>,
    pub use_transforms: bool,
    pub use_weights: bool,
}

impl<S> Collapser<S> {
    pub fn new() -> Self {
        Self {
            superpos_list: vec![],
            sample: None,
            rules: vec![],
            use_transforms: true,
            use_weights: true,
        }
    }

    fn collapse(&mut self) {
        let mut target_sp: Option<&mut Superpos> = None;
        
        {
            let mut target_ent = u16::MAX;

            for sp in &mut self.superpos_list {
                if sp.entropy() < target_ent && !sp.is_collapsed() {
                    target_ent = sp.entropy();
                    target_sp = Some(sp);
                } 
            }
        }

        if target_sp.is_none() {
            return;
        }

        let chosen_val: Option<u16>;
        let possibilities: &Vec<u16> = target_sp.as_ref().unwrap().vals.as_ref();

        if self.use_weights {
            let weights = self.sample.as_ref().unwrap().weight_map();
            let choice = possibilities.choose_weighted(&mut thread_rng(), |v| weights.get(v).unwrap()).unwrap();
            chosen_val = Some(*choice);
        } else {
            chosen_val = Some(possibilities.choose(&mut thread_rng()).unwrap().clone());
        }

        target_sp.as_mut().unwrap().vals.clear();
        target_sp.as_mut().unwrap().vals.push(chosen_val.unwrap().clone());

        let target_loc = target_sp.as_ref().unwrap().loc.clone();
        let neighbours = target_loc.positive_neighbours();
        let cur_rules: Vec<&Rule> = self.rules
            .iter()
            .filter(|r| r.root_id == chosen_val.unwrap())
            .collect();

        for nb_loc in neighbours {
            if let Some(found_sp) = self.superpos_list
                .iter_mut()
                .find(|s| s.loc == nb_loc) {
                let mut vals_removed = 0;

                for (i, val) in found_sp.vals.clone().into_iter().enumerate() {
                    let mut found_rule = false;

                    for rule in &cur_rules {
                        if rule.nb_id == val && rule.dir == target_loc.relative_direction(nb_loc.clone()) {
                            found_rule = true;
                            break;
                        }
                    }

                    if !found_rule {
                        found_sp.vals.remove(i - vals_removed);
                        vals_removed += 1;
                    }
                }
            }
        }
    }

    pub fn collapse_all(&mut self, size: (u32, u32)) -> Result<Vec<(&S, Location)>, WaveError> {
        let mut iters = 0;
        let mut fails = 0;
        let max_fails = 20;

        self.fill_positions(size.clone());

        while !self.superpos_list.iter().all(|s| s.is_collapsed()) {
            self.collapse();
 
            if let Some(_) = self.superpos_list.iter().find(|s| s.vals.is_empty()) {
                fails += 1;

                if fails > max_fails - 1 {
                    return Err(WaveError::Contradiction);
                }
                
                self.fill_positions(size.clone());
            }

            iters += 1;
        }

        Ok(self.mapped_sp_list())
    }

    fn mapped_sp_list(&self) -> Vec<(&S, Location)> {
        let map = &self.sample.as_ref().unwrap().source_map;

        self.superpos_list
            .clone()
            .iter()
            .map(|s| (
                map.get(s.vals.first().unwrap()).unwrap().clone(), 
                s.loc.clone()
            ))
            .collect()
    }

    fn fill_positions(&mut self, size: (u32, u32)) {
        self.superpos_list.clear();
        let vals: Vec<u16> = self.sample
            .as_ref()
            .unwrap()
            .source_map
            .keys()
            .cloned()
            .collect();

        for y in 0..(size.1) {
            for x in 0..(size.0) {
                let loc = Location::new(x as f64, y as f64);
                let sp = Superpos::new(loc, vals.clone());
                self.superpos_list.push(sp);
            }
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
   vals: Vec<u16>,
}

impl Superpos {
    fn new(loc: Location, pot: Vec<u16>) -> Self { Self { loc, vals: pot } }
    fn entropy(&self) -> u16 { self.vals.len() as u16 }

    fn is_collapsed(&self) -> bool {
        let len = self.vals.len();
        len == 1
    }
}

#[derive(Clone)]
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
    
    fn weight_map(&self) -> HashMap<u16, u16> {
        let mut map = HashMap::new();

        for pair in &self.data {
            if let Some(count) = map.get(&pair.0) {
                map.insert(pair.0, count + 1);
                continue;
            }

            map.insert(pair.0, 1);
        }

        return map;
    }
}

#[cfg(test)]
mod tests {
    use super::{Collapser, Sample};
    use std::collections::HashMap;

    #[test]
    fn analysis() {
        let ex = "SCL".to_string();
        let sample = Sample::<char>::from_str(ex); 
        let mut collapser = Collapser::new(); 
        collapser.analyze(sample.clone());
        assert_eq!(collapser.rules.len(), 16);
        collapser.use_transforms = false;
        collapser.analyze(sample);
        assert_eq!(collapser.rules.len(), 4, "Analysis failed. Rules: {:#?}", collapser.rules);
    }

    #[test]
    fn weight_map() {
        let ex = "SSCCLL".to_string();
        let sample = Sample::<char>::from_str(ex); 
        assert!(sample.weight_map().iter().all(|e| *e.1 == 2));
    }
}
