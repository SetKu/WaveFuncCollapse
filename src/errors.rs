use std::clone::Clone;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum WaveErrorTypes {
  ThresholdBreach(u32),
  NoUncollapsedSuperpositions,
  Contradiction,
}

#[derive(Debug, Clone)]
pub struct WaveError {
  data: WaveErrorTypes,
}

impl WaveError {
  pub fn new(t: WaveErrorTypes) -> Self {
    WaveError { data: t }
  }

  pub fn threshhold(t: u32) -> Self {
    WaveError { data: WaveErrorTypes::ThresholdBreach(t) }
  }

  pub fn no_uncollapsed_superpositions() -> Self {
    WaveError { data: WaveErrorTypes::NoUncollapsedSuperpositions }
  }

  pub fn contradiction() -> Self {
    WaveError { data: WaveErrorTypes::Contradiction }
  }
}

impl Error for WaveError { }

impl fmt::Display for WaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      use WaveErrorTypes::*;

      let str = match self.data {
        ThresholdBreach(t) => format!("The threshold ({}) for allowed contradictions (attempts) has been reached.", t),
        NoUncollapsedSuperpositions => "No uncollapsed superpositions available to collapse.".to_string(),
        Contradiction => "There was a contradiction during a collapse.".to_string(),
      };

      write!(f, "{}", str)
    }
}
