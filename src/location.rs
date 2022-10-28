use std::ops::{Sub, Add};
use std::clone::Clone;

#[derive(Clone)]
pub struct Location {
    x: f32,
    y: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }

    pub fn orthogonal_neighbours(&self) -> Vec<Self> {
        vec![
           Self::new(self.x, self.y + 1.0),
           Self::new(self.x + 1.0, self.y),
           Self::new(self.x, self.y - 1.0),
           Self::new(self.x - 1.0, self.y),
        ] 
    }

    pub fn diagonal_neighbours(&self) -> Vec<Self> {
        vec![
           Self::new(self.x + 1.0, self.y + 1.0),
           Self::new(self.x + 1.0, self.y - 1.0),
           Self::new(self.x - 1.0, self.y - 1.0),
           Self::new(self.x - 1.0, self.y + 1.0),
        ] 
    }

    pub fn pos_neighbours(&self) -> Vec<Self> {
        let mut v = vec![];
        v.reserve(8);
        v.append(&mut self.orthogonal_neighbours());
        v.append(&mut self.diagonal_neighbours());
        v.into_iter().filter(|l| l.x >= 0.0 && l.y >= 0.0).collect::<Vec<Location>>()
    }

    pub fn rotate(&self, deg: f32, rel_point: Self) -> Self {
        let rel_self = self.clone() - rel_point;
        let new_x = deg.to_radians().cos() * rel_self.x - deg.to_radians().sin() * rel_self.y; 
        let new_y = deg.to_radians().sin() * rel_self.x + deg.to_radians().cos() * rel_self.y; 
        Location::new(new_x, new_y)
    }
}

impl Sub for Location { 
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Location {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

