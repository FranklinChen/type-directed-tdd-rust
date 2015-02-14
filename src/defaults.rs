// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use fizzbuzz;
use fizzbuzz::Config;

#[allow(dead_code)]
fn buggy_fizzbuzzer(i: i32) -> String {
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
fn old_fizzbuzzer(i: i32) -> String {
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
fn fizzbuzzer_config() -> Config {
  Config::new(&[(3, "Fizz".to_string()),
               (5, "Buzz".to_string())])
    .unwrap()
}

pub fn fizzbuzzer(i: i32) -> String {
  fizzbuzz::evaluate(&fizzbuzzer_config(), i)
}

// Cannot be static variable because of runtime
// validation and also use of Vector.
#[inline]
fn fizzbuzzpopper_config() -> Config {
  Config::new(&[(3, "Fizz".to_string()),
               (5, "Buzz".to_string()),
               (7, "Pop".to_string())])
    .unwrap()
}

pub fn fizzbuzzpopper(i: i32) -> String {
  fizzbuzz::evaluate(&fizzbuzzpopper_config(), i)
}

// Rust needs a test framework allowing fixtures.
#[cfg(test)]
mod test {
  use super::*;

  use quickcheck::TestResult;

  //// FizzBuzz

  #[test] fn test_15() {
    assert_eq!(&*fizzbuzzer(15), "FizzBuzz")
  }

  #[test] fn test_20() {
    assert_eq!(&*fizzbuzzer(20), "Buzz")
  }

  #[test] fn test_6() {
    assert_eq!(&*fizzbuzzer(6), "Fizz")
  }

  #[test] fn test_17() {
    assert_eq!(&*fizzbuzzer(17), "17")
  }

  //// FizzBuzzPop
  #[test] fn test_fizzbuzzpopper_2() {
    assert_eq!(&*fizzbuzzpopper(2), "2")
  }

  #[test] fn test_fizzbuzzpopper_21() {
    assert_eq!(&*fizzbuzzpopper(21), "FizzPop")
  }

  #[test] fn test_fizzbuzzpopper_9() {
    assert_eq!(&*fizzbuzzpopper(9), "Fizz")
  }

  #[test] fn test_fizzbuzzpopper_7() {
    assert_eq!(&*fizzbuzzpopper(7), "Pop")
  }

  #[test] fn test_fizzbuzzpopper_35() {
    assert_eq!(&*fizzbuzzpopper(35), "BuzzPop")
  }

  //// QuickCheck

  #[test]
  fn multiple_of_both_3_and_5() {
    fn multiple_of_both_3_and_5(i: i32) -> TestResult {
      if i % 3 == 0 && i % 5 == 0 {
        TestResult::from_bool(&*fizzbuzzer(i) == "FizzBuzz")
      } else {
        TestResult::discard()
      }
    }
    ::quickcheck::quickcheck(multiple_of_both_3_and_5 as fn(i32) -> TestResult)
  }

  #[test]
  fn multiple_of_only_3() {
    fn multiple_of_only_3(i: i32) -> TestResult {
      if i % 3 == 0 && i % 5 != 0 {
        TestResult::from_bool(&*fizzbuzzer(i) == "Fizz")
      } else {
        TestResult::discard()
      }
    }
    ::quickcheck::quickcheck(multiple_of_only_3 as fn(i32) -> TestResult)
  }

  #[test]
  fn multiple_of_only_5() {
    fn multiple_of_only_5(i: i32) -> TestResult {
      if i % 3 != 0 && i % 5 == 0 {
        TestResult::from_bool(&*fizzbuzzer(i) == "Buzz")
      } else {
        TestResult::discard()
      }
    }
    ::quickcheck::quickcheck(multiple_of_only_5 as fn(i32) -> TestResult)
  }

  #[test]
  fn not_multiple_of_3_and_5() {
    fn not_multiple_of_3_and_5(i: i32) -> TestResult {
      if i % 3 != 0 && i % 5 != 0 {
        TestResult::from_bool(fizzbuzzer(i) == i.to_string())
      } else {
        TestResult::discard()
      }
    }
    ::quickcheck::quickcheck(not_multiple_of_3_and_5 as fn(i32) -> TestResult);
  }
}
