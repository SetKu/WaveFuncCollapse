mod error;
mod superpos;
mod sample;
pub mod location;
use error::*;
use location::*;
use superpos::*;
use sample::*;

pub struct Collapser {
    superpos_list: Vec<Superpos>,
}

impl Collapser {
    pub fn new(superpos_list: Vec<Superpos>) -> Self { Self { superpos_list } }

    pub fn analyze<S>(sample: Sample<S>) {

    }
}

