// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

//! FizzBuzz.

#![feature(phase)]

extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;

use std::iter::range_inclusive;

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
#[allow(dead_code)]
fn main() {
  for result in run_to_seq(1i, 100).iter() {
    println!("{}", result)
  }
}

/// Convert each integer to its correct string output.
/// For convenience, collect each result into a Vec.
fn run_to_seq(start: int, end: int) -> Vec<String> {
  range_inclusive(start, end)
    .map(defaults::fizzbuzzer)
    .collect()
}

#[cfg(test)]
mod test {
  use super::run_to_seq;

  #[test]
  fn test_1_to_16() {
    let expected = vec![
      "1", "2", "Fizz", "4", "Buzz", "Fizz",
      "7", "8", "Fizz", "Buzz", "11", "Fizz",
      "13", "14", "FizzBuzz", "16",
      ]
      .iter()
      .map(|&s| s.to_string())
      .collect();
    assert_eq!(run_to_seq(1, 16), expected)
  }
}