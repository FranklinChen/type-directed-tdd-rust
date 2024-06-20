// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

//! FizzBuzz, implemented in a [type-directed way](https://github.com/FranklinChen/type-directed-tdd-rust) for a presentation for [Pittsburgh Code and Supply](http://codeandsupply.co/).

extern crate quickcheck;
extern crate rand;
extern crate itertools;

mod option_utils;
mod validation;

mod divisor;
mod fizzbuzz;
mod defaults;

use std::ops::Range;
use std::iter::Map;

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
/// Return an iterator for the most flexibility.
#[inline]
fn run_to_seq(start: u32, end: u32)
                 -> Map<Range<u32>, fn(u32) -> String> {
  (start .. end+1)
    .map(defaults::fizzbuzzer)
}

#[cfg(test)]
mod test {
  use super::run_to_seq;
  use itertools;

  #[test]
  fn test_1_to_16() {
    let expected =
      vec!["1", "2", "Fizz", "4", "Buzz", "Fizz",
           "7", "8", "Fizz", "Buzz", "11", "Fizz",
           "13", "14", "FizzBuzz", "16"]
      .into_iter()
      .map(|s| s.to_owned());
    let actual = run_to_seq(1, 16);
    itertools::assert_equal(actual, expected);
  }
}
