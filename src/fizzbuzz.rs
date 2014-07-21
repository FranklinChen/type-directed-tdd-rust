// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

pub type Pair = (int, String);

#[deriving(Show, Clone)]
pub struct Config(pub Vec<Pair>);

static DIVISOR_MIN: int = 2;
static DIVISOR_MAX: int = 100;

fn validate_pair(&(d, _): &Pair) {
  assert!(d >= DIVISOR_MIN,
          "divisor {} must be >= {}", d, DIVISOR_MIN);
  assert!(d <= DIVISOR_MAX,
          "divisor {} must be <= {}", d, DIVISOR_MAX);
}

impl Config {
  pub fn new(pairs: Vec<Pair>) -> Config {
    for pair in pairs.iter() {
      validate_pair(pair);
    }
    Config(pairs)
  }
}

fn rule(&(n, ref word): &Pair, i: int) -> Option<String> {
  if i % n == 0 {
    Some((*word).clone())
  } else {
    None
  }
}

fn add_option(a1: Option<String>, a2: Option<String>)
              -> Option<String> {
  match (a1, a2) {
    (Some(s1), None)     => Some(s1),
    (None,     Some(s2)) => Some(s2),
    (Some(s1), Some(s2)) => Some(s1 + s2),
    (None,     None)     => None,
  }
}

// TODO Use closures to "compile", when Rust supports that.
pub fn evaluate(Config(pairs): Config, i: int)
                -> String {
  let combined: Option<String> = pairs.iter()
            .map(|pair| rule(pair, i))
            .fold(None, add_option);
  combined.unwrap_or_else(|| i.to_string())
}

#[cfg(test)]
mod test {
  use super::{DIVISOR_MIN, DIVISOR_MAX, Pair, Config, evaluate};
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
  fn d1_but_not_d2(p1: Pair,
                   p2: Pair,
                   i: int) -> TestResult {
    let (d1, w1) = p1.clone();
    let (d2, _) = p2;

    if (d1 >= DIVISOR_MIN && d1 <= DIVISOR_MAX)
      && (d2 >= DIVISOR_MIN && d2 <= DIVISOR_MAX) {
      let config = Config::new(vec![p1, p2]);
      
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
