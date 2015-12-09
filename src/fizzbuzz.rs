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
             -> Validation<Config, divisor::Error> {
    let results_iter = pairs
      .iter()
      .cloned()
      .map(|(d, word)|
           Divisor::new(d).map(|div| (div, word)));

    validation::combine_results(results_iter)
      .map(Config)
  }
}

/// Apply the rule for a particular mapping.
fn rule(pair: &DivisorWord,
        i: i32) -> Option<&String> {
  let (ref d_ref, ref word_ref) = *pair;
  if i % d_ref.get() == 0 {
    Some(word_ref)
  } else {
    None
  }
}

// TODO Use closures to "compile" by staging.
/// Evaluate the rule for each DivisorWord, and combine the results.
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
  use divisor::Error::*;
  use quickcheck::TestResult;

  #[test]
  fn validation_works() {
    let actual = Config::new(&[(3, "Fizz".to_owned()),
                               (101, "Buzz".to_owned()),
                               (-5, "Pop".to_owned()),
                               (102, "Boom".to_owned())]);
    let expected = Err(vec![TooBig(101),
                            TooSmall(-5),
                            TooBig(102)]);
    assert_eq!(actual, expected);
  }

  /// TODO nest the checks, staging the generation of i only after
  /// generation of Config.
  ///
  /// [Blocked by Rust bug?](https://github.com/BurntSushi/quickcheck/issues/56)
  #[test]
  fn d1_but_not_d2() {
    fn d1_but_not_d2(dw1: DivisorWord,
                     dw2: DivisorWord,
                     i: i32) -> TestResult {
      let config = Config(vec![dw1.to_owned(),
                               dw2.to_owned()]);
      let (d1, w1) = dw1;
      let (d2, _) = dw2;

      if i % d1.get() == 0 && i % d2.get() != 0 {
        TestResult::from_bool(evaluate(&config, i) == w1)
      } else {
        TestResult::discard()
      }
    }
    ::quickcheck::quickcheck(
      d1_but_not_d2 as fn(_, _, _) -> _);
  }

  // TODO the other three cases are similar.
}
