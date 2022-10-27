use std::clone::Clone;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum WaveError {
    Contradiction,
    InvalidSample,
}

impl Display for WaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use WaveError::*;

        let str = match self {
            Contradiction => "There was a contradiction during a collapse.",
            InvalidSample => "The sample provided was invalid. The item_size might not have been a factor or the sample could have been empty."
        };

        write!(f, "{}", str)
    }
}

impl Error for WaveError {}
