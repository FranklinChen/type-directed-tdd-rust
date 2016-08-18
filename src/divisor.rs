// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

//! Validation of divisors.


use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

use quickcheck::{Arbitrary, Gen, empty_shrinker};

/// A legal divisor.
/// Keep field private to prevent direct construction.
/// Only allow creation with Divisor::new.
#[derive(Debug, PartialEq, Clone)]
pub struct Divisor(i32);

pub static MIN: i32 = 2;
pub static MAX: i32 = 100;

/// Our custom error type when failing to construct a Divisor.
#[derive(Debug, PartialEq)]
pub enum Error {
  TooSmall(i32),
  TooBig(i32)
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match *self {
      Error::TooSmall(size) => write!(f, "{} is too small", size),
      Error::TooBig(size) => write!(f, "{} is too big", size),
    }
  }
}

impl Divisor {
  /// Warning: this logic of if/else only makes sense if MIN <= MAX.
  /// Do not in general trust chained if/else.
  pub fn new(d: i32) -> Result<Divisor, Error> {
    if d < MIN {
      Err(Error::TooSmall(d))
    } else if d > MAX {
      Err(Error::TooBig(d))
    } else {
      Ok(Divisor(d))
    }
  }

  pub fn get(&self) -> i32 {
    match *self {
      Divisor(d) => d
    }
  }
}

// For quickcheck.
impl Arbitrary for Divisor {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let d = g.gen_range(MIN, MAX+1);
    //Divisor(d)
    // It is best to use our safe API even though in this case
    // we "know" that we are generating only valid divisors.
    Divisor::new(d).unwrap()
  }

  fn shrink(&self) -> Box<Iterator<Item=Self>+'static> {
    // TODO: no shrinker for now.
    empty_shrinker()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use super::Error::*;
  use quickcheck::TestResult;

  #[test]
  fn min_and_max_make_sense() {
    assert!(MIN <= MAX)
  }

  #[test]
  fn validate_all_cases() {
    fn validate_all_cases(d: i32) -> TestResult {
      match (d >= MIN, d <= MAX) {
        (true, true) =>
          TestResult::from_bool(Divisor::new(d) == Ok(Divisor(d))),
        (false, true) =>
          TestResult::from_bool(Divisor::new(d) == Err(TooSmall(d))),
        (true, false) =>
          TestResult::from_bool(Divisor::new(d) == Err(TooBig(d))),
        (false, false) =>
          TestResult::error("Impossible combination"),
      }
    }
    ::quickcheck::quickcheck(validate_all_cases as fn(_) -> _)
  }
}
