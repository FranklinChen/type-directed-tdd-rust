// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use fizzbuzz;
use fizzbuzz::Config;

#[allow(dead_code)]
fn buggy_fizzbuzzer(i: int) -> String {
  if i % 3 == 0 {
    "Fizz".to_string()
  } else if i % 5 == 0 {
    "Buzz".to_string()
  } else if i % 3 == 0 && i % 5 == 0 {
    "FizzBuzz".to_string()
  } else {
    i.to_string()
  }
}

#[allow(dead_code)]
fn old_fizzbuzzer(i: int) -> String {
  match (i % 3 == 0, i % 5 == 0) {
    (true,  false) => "Fizz".to_string(),
    (false, true)  => "Buzz".to_string(),
    (true,  true)  => "FizzBuzz".to_string(),
    (false, false) => i.to_string(),
  }
}

// Cannot be static variable because of runtime
// validation and also use of Vector.
#[inline]
fn fizzbuzzer_config<'a>() -> Config<'a> {
  Config::new([(3, "Fizz"),
               (5, "Buzz")])
    .unwrap()
}

pub fn fizzbuzzer(i: int) -> String {
  fizzbuzz::evaluate(&fizzbuzzer_config(), i)
}

// Cannot be static variable because of runtime
// validation and also use of Vector.
#[inline]
fn fizzbuzzpopper_config<'a>() -> Config<'a> {
  Config::new([(3, "Fizz"),
               (5, "Buzz"),
               (7, "Pop")])
    .unwrap()
}

pub fn fizzbuzzpopper(i: int) -> String {
  fizzbuzz::evaluate(&fizzbuzzpopper_config(), i)
}

// Rust needs a test framework allowing fixtures.
#[cfg(test)]
mod test {
  use super::{fizzbuzzer, fizzbuzzpopper};

  use quickcheck::TestResult;

  //// FizzBuzz

  #[test] fn test_15() {
    assert_eq!(fizzbuzzer(15), "FizzBuzz".to_string())
  }

  #[test] fn test_20() {
    assert_eq!(fizzbuzzer(20), "Buzz".to_string())
  }

  #[test] fn test_6() {
    assert_eq!(fizzbuzzer(6), "Fizz".to_string())
  }

  #[test] fn test_17() {
    assert_eq!(fizzbuzzer(17), "17".to_string())
  }

  //// FizzBuzzPop
  #[test] fn test_fizzbuzzpopper_2() {
    assert_eq!(fizzbuzzpopper(2), "2".to_string())
  }

  #[test] fn test_fizzbuzzpopper_21() {
    assert_eq!(fizzbuzzpopper(21), "FizzPop".to_string())
  }

  #[test] fn test_fizzbuzzpopper_9() {
    assert_eq!(fizzbuzzpopper(9), "Fizz".to_string())
  }

  #[test] fn test_fizzbuzzpopper_7() {
    assert_eq!(fizzbuzzpopper(7), "Pop".to_string())
  }

  #[test] fn test_fizzbuzzpopper_35() {
    assert_eq!(fizzbuzzpopper(35), "BuzzPop".to_string())
  }

  //// QuickCheck

  #[quickcheck]
  fn multiple_of_both_3_and_5(i: int) -> TestResult {
    if i % 3 == 0 && i % 5 == 0 {
      TestResult::from_bool(fizzbuzzer(i) == "FizzBuzz".to_string())
    } else {
      TestResult::discard()
    }
  }

  #[quickcheck]
  fn multiple_of_only_3(i: int) -> TestResult {
    if i % 3 == 0 && i % 5 != 0 {
      TestResult::from_bool(fizzbuzzer(i) == "Fizz".to_string())
    } else {
      TestResult::discard()
    }
  }

  #[quickcheck]
  fn multiple_of_only_5(i: int) -> TestResult {
    if i % 3 != 0 && i % 5 == 0 {
      TestResult::from_bool(fizzbuzzer(i) == "Buzz".to_string())
    } else {
      TestResult::discard()
    }
  }

  #[quickcheck]
  fn not_multiple_of_3_and_5(i: int) -> TestResult {
    if i % 3 != 0 && i % 5 != 0 {
      TestResult::from_bool(fizzbuzzer(i) == i.to_string())
    } else {
      TestResult::discard()
    }
  }
}
