// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use option_utils;

use validation;
use validation::Validation;

use divisor;
use divisor::Divisor;

//// Application configuration.

/// Map a divisor to its string, e.g., `(3, "Fizz".to_string())`.
#[deriving(Show, PartialEq)]
pub struct DivisorWord<'a>(Divisor, &'a str);

/// A complete user configuration.
#[deriving(Show, PartialEq)]
pub struct Config<'a>(pub Vec<DivisorWord<'a>>);

//// Application config creation

impl<'a> Config<'a> {
  /// Walk the pairs to create a validated `Vec` of pairs for `Config`.
  pub fn new(pairs: &[(int, &'a str)]) -> Validation<Config<'a>, divisor::Error> {
    pairs
      .iter()
      .map(|&(d, word)|
           Divisor::new(d)
           .map(|div| DivisorWord(div, word)))
      .map(validation::single)
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
fn rule<'a>(&DivisorWord(d, word): &DivisorWord<'a>,
            i: int) -> Option<&'a str> {
  if i % d.get() == 0 {
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
  use divisor::{TooBig, TooSmall};
  use quickcheck::TestResult;

  #[test]
  fn validation_works() {
    let actual = Config::new([(3, "Fizz"),
                              (101, "Buzz"),
                              (-5, "Pop"),
                              (102, "Boom")]);
    let expected = Err(vec![TooBig(101),
                            TooSmall(-5),
                            TooBig(102)]);
    assert_eq!(actual, expected);
  }

  // TODO Write arbitrary for Divisor, Config
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
