use std::ops::{Sub, Add};
use std::clone::Clone;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64,
}

impl Location {
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }

    pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }

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
        use Direction::*;
        let rel_point = to_point - self.clone();

        if rel_point.x < 0.0 && rel_point.y > 0.0 {
            UpLeft
        } else if rel_point.x == 0.0 && rel_point.y > 0.0 {
            Up
        } else if rel_point.x > 0.0 && rel_point.y > 0.0 {
            UpRight
        } else if rel_point.x > 0.0 && rel_point.y == 0.0 {
            Right
        } else if rel_point.x > 0.0 && rel_point.y < 0.0 {
            DownRight
        } else if rel_point.x == 0.0 && rel_point.y < 0.0 {
            Down
        } else if rel_point.x < 0.0 && rel_point.y < 0.0 {
            DownLeft
        } else {
            Left
        } 
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

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Location { }

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
    use super::Direction;

    #[test]
    fn rotations() {
        let point = Location::new(2.0, 2.0);
        let origin = Location::new(0.0, 0.0);
        let result1 = point.rotate(90.0, origin.clone(), 1);
        let result2 = point.rotate(-90.0, origin, 1);
        assert_eq!(result1, Location::new(-2.0, 2.0));
        assert_eq!(result2, Location::new(2.0, -2.0));
    }

    #[test]
    fn relative_directions() {
        use Direction::*;
        let origin = Location::new(0.0, 0.0);

        let point1 = Location::new(-1.0, 1.0);
        assert_eq!(origin.relative_direction(point1), UpLeft);
        let point2 = Location::new(0.0, 1.0);
        assert_eq!(origin.relative_direction(point2), Up);
        let point3 = Location::new(1.0, 1.0);
        assert_eq!(origin.relative_direction(point3), UpRight);
        let point4 = Location::new(1.0, 0.0);
        assert_eq!(origin.relative_direction(point4), Right);
        let point5 = Location::new(1.0, -1.0);
        assert_eq!(origin.relative_direction(point5), DownRight);
        let point6 = Location::new(0.0, -1.0);
        assert_eq!(origin.relative_direction(point6), Down);
        let point7 = Location::new(-1.0, -1.0);
        assert_eq!(origin.relative_direction(point7), DownLeft);
        let point8 = Location::new(-1.0, 0.0);
        assert_eq!(origin.relative_direction(point8), Left);
    }

    #[test]
    fn sorting() {
        let i1 = Location::new(2.0, 3.0);
        let i2 = Location::new(3.0, 3.0);
        let mut arr = vec![i2.clone(), i1.clone()];
        arr.sort();
        assert_eq!(vec![i1, i2], arr);
    }
}
