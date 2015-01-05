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
#[derive(Show, PartialEq)]
pub struct Config(pub Vec<DivisorWord>);

//// Application config creation

impl Config {
  /// Factory method with validation.
  /// Walk the pairs to create a validated `Vec` of pairs for `Config`.
  pub fn new(pairs: &[(int, String)])
             -> Validation<Config, divisor::Error> {
    let results_iter = pairs
      .iter()
      .map(|&(d, ref word)|
           Divisor::new(d).map(|div| (div, word.clone())));

    validation::combine_results(results_iter)
      .map(Config)
  }
}

/// Apply the rule for a particular mapping.
fn rule(pair: &DivisorWord,
            i: int) -> Option<String> {
  let (ref d, ref word) = *pair;
  if i % d.get() == 0 {
    Some(word.clone())
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
    .map(|pair| rule(pair, i))
    .fold(None, option_utils::add)
    .unwrap_or_else(|| i.to_string())
}

#[cfg(test)]
mod test {
  use super::{DivisorWord, Config, evaluate};
  use divisor::Error::{TooBig, TooSmall};
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
  #[quickcheck]
  fn d1_but_not_d2(dw1: DivisorWord,
                   dw2: DivisorWord,
                   i: int) -> TestResult {
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

  // TODO the other three cases are similar.
}
