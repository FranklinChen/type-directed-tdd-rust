// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use std::rand::Rng;
use quickcheck::{Arbitrary, Gen, Shrinker, empty_shrinker};

/// A legal divisor.
/// Keep field private to prevent direct construction.
/// Only allow creation with Divisor::new.
#[deriving(Show, PartialEq, Clone)]
pub struct Divisor(int);

//// Validation of divisors.

pub static MIN: int = 2;
pub static MAX: int = 100;

#[deriving(Show, PartialEq)]
pub enum Error {
  TooSmall(int),
  TooBig(int)
}

impl Divisor {
  /// Warning: this logic of if/else only makes sense if MIN <= MAX.
  /// Do not in general trust chained if/else.
  pub fn new(d: int) -> Result<Divisor, Error> {
    if d < MIN {
      Err(TooSmall(d))
    } else if d > MAX {
      Err(TooBig(d))
    } else {
      Ok(Divisor(d))
    }
  }

  pub fn get(&self) -> int {
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
  use super::{Divisor, TooSmall, TooBig, MIN, MAX};
  use quickcheck::TestResult;

  #[test]
  fn min_and_max_make_sense() {
    assert!(MIN <= MAX)
  }

  #[quickcheck]
  fn validate_all_cases(d: int) -> TestResult {
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
}