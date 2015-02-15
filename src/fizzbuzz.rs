// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use option_utils;

use validation;
use validation::Validation;

use divisor;
use divisor::Divisor;

//// Application configuration.

/// Map a divisor to its string, e.g., `(3, "Fizz".to_string())`.
pub type DivisorWord = (Divisor, String);

/// A complete user configuration.
#[derive(Debug, PartialEq)]
pub struct Config(pub Vec<DivisorWord>);

//// Application config creation

impl Config {
  /// Factory method with validation.
  /// Walk the pairs to create a validated `Vec` of pairs for `Config`.
  pub fn new(pairs: &[(i32, String)])
             -> Validation<Config, divisor::MyError> {
    let results_iter = pairs
      .iter()
      .map(|&(d, ref word_ref)|
           Divisor::new(d).map(|div| (div, word_ref.clone())));

    validation::combine_results(results_iter)
      .map(Config)
  }
}

/// Apply the rule for a particular mapping.
fn rule(pair: &DivisorWord,
            i: i32) -> Option<String> {
  let (ref d_ref, ref word_ref) = *pair;
  if i % d_ref.get() == 0 {
    Some(word_ref.clone())
  } else {
    None
  }
}

// TODO Use closures to "compile", when Rust supports that.
/// Use an optimization by converting from `&str` to `String`
/// right away in order to append to the first `String` in repeated
/// adds.
pub fn evaluate(&Config(ref pairs): &Config, i: i32) -> String {
  pairs
    .iter()
    .map(|pair_ref| rule(pair_ref, i))
    .fold(None, option_utils::add)
    .unwrap_or_else(|| i.to_string())
}

#[cfg(test)]
mod test {
  use super::*;
  use divisor::MyError::*;
  use quickcheck::TestResult;

  #[test]
  fn validation_works() {
    let actual = Config::new(&[(3, "Fizz".to_string()),
                              (101, "Buzz".to_string()),
                              (-5, "Pop".to_string()),
                              (102, "Boom".to_string())]);
    let expected = Err(vec![TooBig(101),
                            TooSmall(-5),
                            TooBig(102)]);
    assert_eq!(actual, expected);
  }

  // TODO nest the checks, staging the generation of i only after
  // generation of Config.
  #[test]
  fn d1_but_not_d2() {
    fn d1_but_not_d2(dw1: DivisorWord,
                     dw2: DivisorWord,
                     i: i32) -> TestResult {
      let config = Config(vec![dw1.clone(),
                               dw2.clone()]);
      let (d1, w1) = dw1;
      let (d2, _) = dw2;

      if i % d1.get() == 0 && i % d2.get() != 0 {
        TestResult::from_bool(evaluate(&config, i) == w1)
      } else {
        TestResult::discard()
      }
    }
    ::quickcheck::quickcheck(d1_but_not_d2 as fn(DivisorWord, DivisorWord, i32) -> TestResult);
  }

  // TODO the other three cases are similar.
}
