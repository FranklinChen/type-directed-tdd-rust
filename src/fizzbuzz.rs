// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

pub type Pair<'a> = (int, &'a str);

#[deriving(Show, Clone)]
pub struct Config<'a>(pub Vec<Pair<'a>>);

static DIVISOR_MIN: int = 2;
static DIVISOR_MAX: int = 100;

/*
  Simplistically, just crash the task if invalid.

  TODO

  In reality, would want to at least use Rust's
  Result type http://doc.rust-lang.org/std/result/index.html

  Better yet, a full-fledged validation framework like Scalaz's.
*/
fn validate_pair(&(d, _): &Pair) {
  assert!(d >= DIVISOR_MIN,
          "divisor {} must be >= {}", d, DIVISOR_MIN);
  assert!(d <= DIVISOR_MAX,
          "divisor {} must be <= {}", d, DIVISOR_MAX);
}

impl<'a> Config<'a> {
  pub fn new(pairs: Vec<Pair>) -> Config {
    for pair in pairs.iter() {
      validate_pair(pair);
    }
    Config(pairs)
  }
}

// Make the decision to allocate a String here.
fn rule(&(n, word): &Pair, i: int) -> Option<String> {
  if i % n == 0 {
    Some(word.to_string())
  } else {
    None
  }
}

/*
  Add options, with an interesting optimization: String append reuses
  the internal buffer of the first string.

  TODO

  Ideally, Rust library should provide Semigroup and Monoid traits
  so that Option<T> is a Monoid if T is an Add (Rust-speak for Semigroup),
  like Scalaz: https://github.com/scalaz/scalaz/blob/series/7.2.x/core/src/main/scala/scalaz/std/Option.scala

  Note that a Monoid is a Zero (Rust-speak) with Add (Semigroup).

  The code was written for clarity rather than full efficiency.
  Returning Some(s1) could just be a1 and Some(s2) could be just a2.
*/
fn add_option(a1: Option<String>, a2: Option<String>)
              -> Option<String> {
  match (a1, a2) {
    (Some(s1), None)     => Some(s1),
    (None,     Some(s2)) => Some(s2),
    (Some(s1), Some(s2)) => Some(s1.append(s2.as_slice())),
    (None,     None)     => None,
  }
}

// TODO Use closures to "compile", when Rust supports that.
pub fn evaluate(Config(pairs): Config, i: int) -> String {
  pairs
    .iter()
    .map(|pair| rule(pair, i))
    .fold(None, add_option)
    .unwrap_or_else(|| i.to_string())
}

#[cfg(test)]
mod test {
  use super::{DIVISOR_MIN, DIVISOR_MAX, Config, evaluate};
  use quickcheck::TestResult;

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
      let config = Config::new(vec![(d1, w1.as_slice()),
                                    (d2, w2.as_slice())]);
      
      if i % d1 == 0 && i % d2 != 0 {
        TestResult::from_bool(evaluate(config, i) == w1)
      } else {
        TestResult::discard()
      }
    } else {
      TestResult::discard()
    }
  }

  // TODO the other three cases are similar.
}
