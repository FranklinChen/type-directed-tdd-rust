// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use validation;

/// A legal divisor.
/// Only create with Divisor::new.
/// TODO later, enforce with private field and a public getter
#[deriving(Show, PartialEq)]
pub struct Divisor(pub int);

//// Validation of divisors.

pub static MIN: int = 2;
pub static MAX: int = 100;

#[deriving(Show, PartialEq)]
pub enum DivisorError {
  DivisorTooSmall(int),
  DivisorTooBig(int)
}

fn validate_for_min(d: int) -> Result<(), DivisorError> {
  if d >= MIN {
    Ok(())
  } else {
    Err(DivisorTooSmall(d))
  }
}

fn validate_for_max(d: int) -> Result<(), DivisorError> {
  if d <= MAX {
    Ok(())
  } else {
    Err(DivisorTooBig(d))
  }
}

impl Divisor {
  pub fn new(d: int) -> validation::Validation<Divisor, DivisorError> {
    validation::add_with(validate_for_min(d).map_err(|e| vec![e]),
                         validate_for_max(d).map_err(|e| vec![e]),
                         |(), ()| Divisor(d))
  }
}
