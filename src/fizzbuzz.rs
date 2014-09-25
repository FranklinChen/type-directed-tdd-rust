// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

//// Utilities

/*
  Add options, with an interesting optimization: push_str reuses
  the internal buffer of the first string.

  Option<String> as a Monoid since String is a Semigroup (Add in
  Rust-speak). Rust also has a Zero, so a Monoid could be Zero + Add.
*/
fn add_option_string(a1: Option<String>,
                     a2: Option<String>) -> Option<String> {
  match (a1, a2) {
    (Some(s1),     None)     => Some(s1),
    (None,         Some(s2)) => Some(s2),
    (Some(mut s1), Some(s2)) => Some({ s1.push_str(s2.as_slice()); s1 }),
    (None,         None)     => None,
  }
}

/// Convenient type synonym for a common pattern of usage.
/// More generally, Vec could be replaced by a Semigroup (Add).
type ResultVec<T, E> = Result<T, Vec<E>>;

/// Combine successful results with `f`, but accumulate errors.
/// Important: any error causes the whole result to be an error!
fn add_result<V, T, U, E>(result1: ResultVec<V, E>,
                          result2: ResultVec<T, E>,
                          f: |V, T| -> U) -> ResultVec<U, E> {
  match (result1, result2) {
    (Ok(v),       Ok(t))   => Ok(f(v, t)),
    (Ok(_),       Err(e2)) => Err(e2),
    (Err(e1),     Ok(_))   => Err(e1),
    (Err(mut e1), Err(e2)) => Err({ e1.extend(e2.into_iter()); e1 })
  }
}

//// Application configuration.

/// Map a divisor to its string, e.g., `(3, "Fizz".to_string())`.
pub type Pair<'a> = (int, &'a str);

/// A complete user configuration.
#[deriving(Show, PartialEq)]
pub struct Config<'a>(pub Vec<Pair<'a>>);

//// Validation of divisors.

static DIVISOR_MIN: int = 2;
static DIVISOR_MAX: int = 100;

#[deriving(Show, PartialEq)]
enum DivisorError {
  DivisorTooSmall(int),
  DivisorTooBig(int)
}

/// Our particular Result when validating divisors.
type ValidatedResult<T> = ResultVec<T, DivisorError>;

fn validated_min(d: int) -> Result<int, DivisorError> {
  if d >= DIVISOR_MIN {
    Ok(d)
  } else {
    Err(DivisorTooSmall(d))
  }
}

fn validated_max(d: int) -> Result<int, DivisorError> {
  if d <= DIVISOR_MAX {
    Ok(d)
  } else {
    Err(DivisorTooBig(d))
  }
}

fn validated_pair<'a>(&p: &Pair<'a>) -> ValidatedResult<Pair<'a>> {
  let (d, _) = p;
  add_result(validated_min(d).map_err(|e| vec![e]),
             validated_max(d).map_err(|e| vec![e]),
             |_, _| p)
}

//// Application config creation

impl<'a> Config<'a> {
  /// Walk the pairs to create a validated `Vec` of pairs for `Config`.
  pub fn new(pairs: &[Pair<'a>]) -> ValidatedResult<Config<'a>> {
    pairs
      .iter()
      .map(validated_pair)
      .fold(Ok(vec![]),
            |v, t|
            add_result(v, t,
                       |mut x, y| {
                         x.push(y);
                         x
                       }))
      .map(Config)
  }
}

/// Apply the rule for a particular mapp
fn rule<'a>(&(n, word): &Pair<'a>, i: int) -> Option<&'a str> {
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
    .fold(None, add_option_string)
    .unwrap_or_else(|| i.to_string())
}

#[cfg(test)]
mod test {
  use super::{DIVISOR_MIN, DIVISOR_MAX, DivisorTooBig, DivisorTooSmall, Config, evaluate};
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

  // TODO nest the checks, staging the check of i.
  // Trivial in Scala and Haskell.
  #[quickcheck]
  fn d1_but_not_d2((d1, w1): (int, String),
                   (d2, w2): (int, String),
                   i: int) -> TestResult {
    if (d1 >= DIVISOR_MIN && d1 <= DIVISOR_MAX)
      && (d2 >= DIVISOR_MIN && d2 <= DIVISOR_MAX) {
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
