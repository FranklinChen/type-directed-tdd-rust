// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use std::rand::Rng;
use quickcheck::{Arbitrary, Gen, Shrinker, empty_shrinker};

/// A legal divisor.
/// Keep field private to prevent direct construction.
/// Only allow creation with Divisor::new.
#[derive(Show, PartialEq, Clone)]
pub struct Divisor(isize);

//// Validation of divisors.

pub static MIN: isize = 2;
pub static MAX: isize = 100;

#[derive(Show, PartialEq)]
pub enum Error {
  TooSmall(isize),
  TooBig(isize)
}

impl Divisor {
  /// Warning: this logic of if/else only makes sense if MIN <= MAX.
  /// Do not in general trust chained if/else.
  pub fn new(d: isize) -> Result<Divisor, Error> {
    if d < MIN {
      Err(Error::TooSmall(d))
    } else if d > MAX {
      Err(Error::TooBig(d))
    } else {
      Ok(Divisor(d))
    }
  }

  pub fn get(&self) -> isize {
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
  use super::{Divisor, MIN, MAX};
  use super::Error::{TooBig, TooSmall};
  use quickcheck::TestResult;

  #[test]
  fn min_and_max_make_sense() {
    assert!(MIN <= MAX)
  }

  #[quickcheck]
  fn validate_all_cases(d: isize) -> TestResult {
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
