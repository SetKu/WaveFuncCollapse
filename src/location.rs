use std::ops::{Sub, Add};
use std::clone::Clone;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Location {
    x: f64,
    y: f64,
}

impl Location {
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }

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

    pub fn positive_neighbours(&self) -> Vec<Self> {
        let mut v = vec![];
        v.reserve(8);
        v.append(&mut self.orthogonal_neighbours());
        v.append(&mut self.diagonal_neighbours());
        v.into_iter().filter(|l| l.x >= 0.0 && l.y >= 0.0).collect::<Vec<Location>>()
    }

    pub fn rotate(&self, deg: f64, org_point: Self, precision: u8) -> Self {
        let rel_self = self.clone() - org_point.clone();
        let new_x = deg.to_radians().cos() * rel_self.x - deg.to_radians().sin() * rel_self.y; 
        let new_y = deg.to_radians().sin() * rel_self.x + deg.to_radians().cos() * rel_self.y; 
        let res = Location::new(new_x, new_y) + org_point;
        res.round(precision)
    }

    pub fn round(mut self, decimals: u8) -> Self {
        let factor = 10_f64.powf(decimals as f64);
        self.x = (self.x * factor).round() / factor;
        self.y = (self.y * factor).round() / factor;
        self
    }

    pub fn relative_direction(&self, to_point: Location) -> Direction {
        let rel_point = self.clone() - to_point;
        return Direction::Up;
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

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

#[cfg(test)]
mod tests {
    use super::Location;

    #[test]
    fn rotations() {
        let point = Location::new(2.0, 2.0);
        let origin = Location::new(0.0, 0.0);
        let result1 = point.rotate(90.0, origin.clone(), 1);
        let result2 = point.rotate(-90.0, origin, 1);
        assert_eq!(result1, Location::new(-2.0, 2.0));
        assert_eq!(result2, Location::new(2.0, -2.0));
    }
}
