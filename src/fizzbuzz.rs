// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use option_utils;
use validation;
use validation::Validation;
use divisor::{Divisor, DivisorError};

//// Application configuration.

/// Map a divisor to its string, e.g., `(3, "Fizz".to_string())`.
#[deriving(Show, PartialEq)]
pub struct DivisorWord<'a>(Divisor, &'a str);

impl<'a> DivisorWord<'a> {
  pub fn new(d: int, word: &'a str)
             -> Validation<DivisorWord<'a>, DivisorError> {
    Divisor::new(d)
      .map(|div| DivisorWord(div, word))
  }
}

/// A complete user configuration.
#[deriving(Show, PartialEq)]
pub struct Config<'a>(pub Vec<DivisorWord<'a>>);

//// Application config creation

impl<'a> Config<'a> {
  /// Walk the pairs to create a validated `Vec` of pairs for `Config`.
  pub fn new(pairs: &[(int, &'a str)])
             -> Validation<Config<'a>, DivisorError> {
    pairs
      .iter()
      .map(|&(d, word)| DivisorWord::new(d, word))
      .fold(Ok(vec![]),
            |v, t|
            validation::add_with(v, t,
                                 |mut x, y| {
                                   x.push(y);
                                   x
                                 }))
      .map(Config)
  }
}

/// Apply the rule for a particular mapping.
fn rule<'a>(&DivisorWord(Divisor(n), word): &DivisorWord<'a>,
            i: int) -> Option<&'a str> {
  if i % n == 0 {
    Some(word)
  } else {
    None
  }
}

// TODO Use closures to "compile", when Rust supports that.
/// Use an optimization by converting from `&str` to `String`
/// right away in order to append to the first `String` in repeated
/// adds.
pub fn evaluate(&Config(ref pairs): &Config, i: int) -> String {
  pairs
    .iter()
    .map(|pair| rule(pair, i)
         .map(|s| s.to_string()))
    .fold(None, option_utils::add)
    .unwrap_or_else(|| i.to_string())
}

#[cfg(test)]
mod test {
  use super::{Config, evaluate};
  use divisor;
  use divisor::{DivisorTooBig, DivisorTooSmall};
  use quickcheck::TestResult;

  #[test]
  fn validation_works() {
    let actual = Config::new([(3, "Fizz"),
                              (101, "Buzz"),
                              (-5, "Pop"),
                              (102, "Boom")]);
    let expected = Err(vec![DivisorTooBig(101),
                            DivisorTooSmall(-5),
                            DivisorTooBig(102)]);
    assert_eq!(actual, expected);
  }

/*
  // TODO Create own Arbitrary instances for Config.
  // Too complex in Rust for presentation, unfortunately.
  use quickcheck::{Arbitrary, Gen, Shrinker};

  // Trivial in Scala and Haskell.
  impl Arbitrary for Config {
    fn arbitrary<G: Gen>(g: &mut G) -> Config {
      // TODO
      fail!()
    }
    fn shrink(&self) -> Box<Shrinker<Config>> {
      // TODO
      fail!()
    }
  }
*/

  // TODO Write arbitrary for Divisor
  // TODO nest the checks, staging the check of i.
  // Trivial in Scala and Haskell.
  #[quickcheck]
  fn d1_but_not_d2((d1, w1): (int, String),
                   (d2, w2): (int, String),
                   i: int) -> TestResult {
    if (d1 >= divisor::MIN && d1 <= divisor::MAX)
      && (d2 >= divisor::MIN && d2 <= divisor::MAX) {
        // TODO do not unwrap
      let config = Config::new([(d1, w1.as_slice()),
                                (d2, w2.as_slice())])
          .unwrap();

      if i % d1 == 0 && i % d2 != 0 {
        TestResult::from_bool(evaluate(&config, i) == w1)
      } else {
        TestResult::discard()
      }
    } else {
      TestResult::discard()
    }
  }

  // TODO the other three cases are similar.
}
