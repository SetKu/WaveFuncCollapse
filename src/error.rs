use std::clone::Clone;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum WaveError {
    Contradiction,
}

impl Display for WaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use WaveError::*;

        let str = match self {
            Contradiction => "There was a contradiction during a collapse.",
        };

        write!(f, "{}", str)
    }
}

impl Error for WaveError {}
