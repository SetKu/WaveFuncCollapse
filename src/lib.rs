mod error;
use std::clone::Clone;
use rand::thread_rng;
use rand::prelude::*;
use ndarray::prelude::*;
use cgmath::{Point2, Vector2};

enum Flags {
    UseWeights = 1,
    UseRotations,
}

enum BorderMode {
    // don't include border patterns
    Exclude = 1,
    // include border patterns and their neighbours
    Clamp,
    // include border patterns and all their neighbours wrapping across the input
    Wrap,
}

pub struct Wave {
    flags: Vec<Flags>,
    border_mode: BorderMode,
    output: Array2<u8>,
    // the wave can take in a pre-existing collection of elements to substitute in
    preset: Option<Vec<Element>>,
    new_size: Vector2<u8>,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            flags: vec![Flags::UseWeights, Flags::UseRotations],
            border_mode: BorderMode::Clamp,
            output: array!([]),
            preset: None,
            new_size: Vector2 { x: 0, y: 0 },
        }
    }

    pub fn analyze(input: Array2<u8>, n_size: u8) {
        assert!(input.len_of(Axis(1)) >= n_size as usize);

    }
}

struct Pattern {
    frequency: f32,
    entropy: f32,
    contents: Array2<u8>,
}

impl Pattern {
    fn new(contents: Array2<u8>) -> Self {
        Pattern { frequency: 1.0, entropy: 0.0, contents }
    }
}

struct Element {
    values: Vec<Pattern>,
    // https://youtu.be/-Rb868HKCo8
    entropy: f32,
    position: Point2<u16>,
}
