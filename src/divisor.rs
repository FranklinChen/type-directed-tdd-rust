// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

use rand::Rng;
use quickcheck::{Arbitrary, Gen, Shrinker, empty_shrinker};

/// A legal divisor.
/// Keep field private to prevent direct construction.
/// Only allow creation with Divisor::new.
#[derive(Debug, PartialEq, Clone)]
pub struct Divisor(i32);

//// Validation of divisors.

pub static MIN: i32 = 2;
pub static MAX: i32 = 100;

#[derive(Debug, PartialEq)]
pub enum MyError {
  TooSmall(i32),
  TooBig(i32)
}

impl Display for MyError {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    match self {
      &MyError::TooSmall(size) => write!(f, "{} is too small", size),
      &MyError::TooBig(size) => write!(f, "{} is too big", size),
    }
  }
}

impl Display for Vec<MyError> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "Errors: ["));
    for e in self {
      try!(write!(f, "{}", e));
      try!(write!(f, "\n"));
    }
    write!(f, "]")
  }
}

impl Divisor {
  /// Warning: this logic of if/else only makes sense if MIN <= MAX.
  /// Do not in general trust chained if/else.
  pub fn new(d: i32) -> Result<Divisor, MyError> {
    if d < MIN {
      Err(MyError::TooSmall(d))
    } else if d > MAX {
      Err(MyError::TooBig(d))
    } else {
      Ok(Divisor(d))
    }
  }

  pub fn get(&self) -> i32 {
    match self {
      &Divisor(d) => d
    }
  }
}

// For quickcheck.
impl Arbitrary for Divisor {
  fn arbitrary<G: Gen>(g: &mut G) -> Divisor {
    let d = g.gen_range(MIN, MAX+1);
    //Divisor(d)
    // It is best to use our safe API even though in this case
    // we "know" that we are generating only valid divisors.
    Divisor::new(d).unwrap()
  }

  fn shrink(&self) -> Box<Shrinker<Divisor>+'static> {
    // TODO: no shrinker for now.
    empty_shrinker()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use super::MyError::*;
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
    ::quickcheck::quickcheck(validate_all_cases as fn(i32) -> TestResult)
  }
}
