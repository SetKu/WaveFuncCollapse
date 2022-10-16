// No defined size.
#[derive(Debug)]
struct Superposition {
    candidates: Vec<Tile>
}

impl Superposition {
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
struct Tile {
    contradictions: Vec<(Box<Tile>, Box<Tile>, Direction)>,
    weight: f32,
}

#[derive(Debug)]
struct Coordinator {
    superpositions: Vec<Box<Superposition>>
}

impl Coordinator {
    fn new() -> Self {
        Coordinator { superpositions: vec![] }
    }
}

fn main() {
    let mut coord = Coordinator::new();
}
