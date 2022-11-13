mod error;
pub mod location;

use location::{Location, Direction};
use error::WaveError;

use std::collections::HashMap;
use std::clone::Clone;
use rand::thread_rng;
use rand::prelude::*;

pub struct Collapser<S> {
    pub superpos_list: Vec<Superpos>,
    pub sample: Option<Sample<S>>,
    pub rules: Vec<Rule>,
    pub use_transforms: bool,
    pub use_weights: bool,
    pub max_contradictions: u32,
}

impl<S> Default for Collapser<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Collapser<S> {
    pub fn new() -> Self {
        Self {
            superpos_list: vec![],
            sample: None,
            rules: vec![],
            use_transforms: true,
            use_weights: true,
            max_contradictions: 20,
        }
    }

    pub fn collapse(&mut self) {
        let mut target_sps: Vec<&mut Superpos> = vec![];
        
        {
            let mut target_ent = u16::MAX;

            for sp in &mut self.superpos_list {
                if sp.entropy() < target_ent && !sp.is_collapsed() {
                    target_ent = sp.entropy();
                    target_sps.clear();
                    target_sps.push(sp);
                } else if sp.entropy() == target_ent && !sp.is_collapsed() {
                    target_sps.push(sp);
                } 
            }
        }

        if target_sps.is_empty() {
            return;
        }

        let target_sp: &mut Superpos = target_sps.into_iter().choose(&mut thread_rng()).unwrap();

        let possibilities: &Vec<u16> = target_sp.vals.as_ref();
        let chosen_val: Option<u16> = if self.use_weights {
            let weights = self.sample.as_ref().unwrap().weight_map();
            let choice = possibilities.choose_weighted(&mut thread_rng(), |v| weights.get(v).unwrap()).unwrap();
            Some(*choice)
        } else {
            Some(*possibilities.choose(&mut thread_rng()).unwrap())
        };

        target_sp.vals.clear();
        target_sp.vals.push(chosen_val.unwrap());

        let target_loc = target_sp.loc.clone();
        let neighbours = target_loc.positive_neighbours();
        let cur_rules: Vec<&Rule> = self.rules
            .iter()
            .filter(|r| r.root_id == chosen_val.unwrap())
            .collect();
        let total_weight = cur_rules.clone().into_iter().map(|r| r.weight).reduce(|acc, itm| acc + itm).unwrap();

        for nb_loc in neighbours {
            if let Some(found_sp) = self.superpos_list
                .iter_mut()
                .find(|s| s.loc == nb_loc) {
                let mut vals_removed = 0;

                for (i, val) in found_sp.vals.clone().into_iter().enumerate() {
                    let mut found_rule = false;

                    for rule in &cur_rules {
                        if self.use_weights {
                            let chance = rule.weight as f32 / total_weight as f32;
                            let rdm: f32 = thread_rng().gen();

                            if rdm > chance {
                                if rule.nb_id == val && rule.dir == target_loc.relative_direction(nb_loc.clone()) {
                                    found_rule = true;
                                    break;
                                }
                            }
                        } else {
                            if rule.nb_id == val && rule.dir == target_loc.relative_direction(nb_loc.clone()) {
                                found_rule = true;
                                break;
                            }
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

    pub fn collapse_all(&mut self, size: (u32, u32)) -> Result<(Vec<(&S, Location)>, u32), WaveError> {
        let mut fails = 0;

        self.fill_positions(size);

        while !self.superpos_list.iter().all(|s| s.is_collapsed()) {
            self.collapse();
 
            if self.superpos_list.iter().any(|s| s.vals.is_empty()) {
                fails += 1;

                if fails > self.max_contradictions - 1 {
                    return Err(WaveError::Contradiction);
                }
                
                self.fill_positions(size);
            }
        }

        Ok((self.mapped_sp_list(), fails))
    }

    pub fn mapped_sp_list(&self) -> Vec<(&S, Location)> {
        let map = &self.sample.as_ref().unwrap().source_map;

        self.superpos_list
            .clone()
            .iter()
            .map(|s| (
                map.get(s.vals.first().unwrap()).unwrap(), 
                s.loc.clone()
            ))
            .collect()
    }

    pub fn mapped_multi_sp_list(&self) -> Vec<(Vec<&S>, Location)> {
        let map = &self.sample.as_ref().unwrap().source_map;

        self.superpos_list
            .clone()
            .iter()
            .map(|s| (
                s.vals.iter().map(|s| map.get(s).unwrap()).collect::<Vec<&S>>(),
                s.loc.clone()
            ))
            .collect()
    }

    pub fn fill_positions(&mut self, size: (u32, u32)) {
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

                        if let Some(existing) = self.rules.iter_mut().find(|r| **r == rule) {
                            existing.weight += 1;
                        } else {
                            self.rules.push(rule); 
                        }
                    }
                }
            } 
        }
    }
}

pub fn collapse_all_str(collapser: &mut Collapser<char>, size: (u32, u32), print: bool, interval: std::time::Duration) -> Result<(String, u32), WaveError> {
    let mut iters = 0_u32;
    let mut fails = 0;

    collapser.fill_positions(size);

    while !collapser.superpos_list.iter().all(|s| s.is_collapsed()) {
        collapser.collapse();

        if collapser.superpos_list.iter().any(|s| s.vals.is_empty()) {
            fails += 1;

            if fails > collapser.max_contradictions - 1 {
                return Err(WaveError::Contradiction);
            }
            
            collapser.fill_positions(size);
        }

        if print {
            let tot_ids = collapser.sample.as_ref().unwrap().unique_sources();

            let temp = collapser.mapped_multi_sp_list()
                .into_iter()
                .map(|i| {
                    let mut compact = i.0
                        .into_iter()
                        .map(|c| c.to_string())
                        .reduce(|acc, itm| acc + &itm)
                        .unwrap();

                    if compact.chars().count() < tot_ids {
                        for _ in 0..(tot_ids - compact.chars().count()) {
                            compact.push(' ');
                        }
                    }

                    (
                        format!("({})", compact),
                        i.1
                    )
                })
                .collect();
                
            let rep = Parser::parse(temp);
            println!("Iteration: {}, Attempt: {}\n{}\n", iters + 1, fails + 1, rep);
            
            std::thread::sleep(interval);
        }

        iters += 1;
    }

    let result = collapser.mapped_sp_list()
        .into_iter()
        .map(|i| (i.0.to_string(), i.1))
        .collect();
    let parsed = Parser::parse(result);
    Ok((parsed, fails))
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    root_id: u16,
    nb_id: u16,
    dir: Direction,
    weight: u16,
}

impl Rule {
    pub fn new(root_id: u16, nb_id: u16, dir: Direction) -> Self { Self { root_id, nb_id, dir, weight: 1 } }
}

#[derive(Clone, Debug)]
pub struct Superpos {
   loc: Location,
   vals: Vec<u16>,
}

impl Superpos {
    pub fn new(loc: Location, pot: Vec<u16>) -> Self { Self { loc, vals: pot } }
    pub fn entropy(&self) -> u16 { self.vals.len() as u16 }

    pub fn is_collapsed(&self) -> bool {
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
    pub fn new_str(sample: String) -> Sample<char> {
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
    
    pub fn weight_map(&self) -> HashMap<u16, u16> {
        let mut map = HashMap::new();

        for pair in &self.data {
            if let Some(count) = map.get(&pair.0) {
                map.insert(pair.0, count + 1);
                continue;
            }

            map.insert(pair.0, 1);
        }

        map
    }

    pub fn unique_sources(&self) -> usize {
        self.source_map.keys().count()
    }
}

pub struct Parser { }

impl Parser {
    pub fn parse(result: Vec<(String, Location)>) -> String {
        let mut organized = result;
        organized.sort_by_key(|i| i.1.clone());
        
        let mut output = String::new();
        let mut line: i64 = 0;

        for (st, loc) in organized {
            if line < loc.y as i64 {
                output.push('\n');
                line = loc.y as i64;
            }
            
            output.push_str(&st);
        }

        output
    }

    pub fn insert_commas(str: &mut String) {
        *str = str
            .chars()
            .map(|c| if c.is_whitespace() { c.to_string() } else { format!("{}, ", c) })
            .reduce(|accum, item| accum + &item)
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::{Collapser, Sample};

    #[test]
    fn analysis() {
        let ex = "SCL".to_string();
        let sample = Sample::<char>::new_str(ex); 
        let mut collapser = Collapser::new(); 
        collapser.analyze(sample.clone());
        assert_eq!(collapser.rules.len(), 16);
        collapser.use_transforms = false;
        collapser.analyze(sample);
        assert_eq!(collapser.rules.len(), 4, "Analysis failed. Rules: {:#?}", collapser.rules);
    }

    #[test]
    pub fn weight_map() {
        let ex = "SSCCLL".to_string();
        let sample = Sample::<char>::new_str(ex); 
        assert!(sample.weight_map().iter().all(|e| *e.1 == 2));
    }
}
