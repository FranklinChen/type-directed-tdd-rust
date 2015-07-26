// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

//! FizzBuzz, implemented in a [type-directed way](https://github.com/FranklinChen/type-directed-tdd-rust) for a presentation for [Pittsburgh Code and Supply](http://codeandsupply.co/).

extern crate quickcheck;
extern crate rand;

mod option_utils;
mod validation;

mod divisor;
mod fizzbuzz;
mod defaults;

/// Solve the [FizzBuzz programming problem](http://c2.com/cgi/wiki?FizzBuzzTest).
///
/// Write a program that
///   prints the numbers from 1 to 100.
/// But for multiples of three,
///   print "Fizz" instead of the number.
/// And for the multiples of five,
///   print "Buzz".
/// For numbers which are multiples of both three and five,
///   print "FizzBuzz".
#[cfg(not(test))]
fn main() {
  for result in run_to_seq(1, 100) {
    println!("{}", result);
  }
}

/// Convert each integer to its correct string output.
///
/// Return a Vec for convenience. Could return an iterator instead.
#[inline]
fn run_to_seq(start: i32, end: i32) -> Vec<String> {
  (start .. end+1)
    .map(defaults::fizzbuzzer)
    .collect()
}

#[cfg(test)]
mod test {
  use super::run_to_seq;

  #[test]
  fn test_1_to_16() {
    let expected: Vec<String> =
      ["1", "2", "Fizz", "4", "Buzz", "Fizz",
       "7", "8", "Fizz", "Buzz", "11", "Fizz",
       "13", "14", "FizzBuzz", "16"]
      .into_iter()
      .map(|s| s.to_string())
      .collect();
    let actual = run_to_seq(1, 16);
    assert_eq!(actual, expected);
  }
}
