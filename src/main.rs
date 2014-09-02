// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

//! FizzBuzz.

#![feature(phase)]

extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;

use std::iter::range_inclusive;
use std::iter::RangeInclusive;

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
  for result in run_to_seq(1i, 100) {
    println!("{}", result)
  }
}

/// Ugly, should be hidden: waiting for Rust proposal of impl trait.
type OurIterator<'a, T> = std::iter::Map<'a, int, T, RangeInclusive<int>>;

/// Convert each integer to its correct string output.
/// Return an `Iterator` for maximum flexibility.
fn run_to_seq<'a>(start: int, end: int) -> OurIterator<'a, String> {
  range_inclusive(start, end)
    .map(defaults::fizzbuzzer)
}

#[cfg(test)]
mod test {
  use super::run_to_seq;

  #[test]
  fn test_1_to_16() {
    let expected_slices = vec![
      "1", "2", "Fizz", "4", "Buzz", "Fizz",
      "7", "8", "Fizz", "Buzz", "11", "Fizz",
      "13", "14", "FizzBuzz", "16"];
    let actual = run_to_seq(1, 16)
      .collect::<Vec<String>>();
    let actual_slices = actual
      .iter()
      .map(|s| s.as_slice())
      .collect::<Vec<&str>>();
    assert_eq!(actual_slices, expected_slices)
  }
}