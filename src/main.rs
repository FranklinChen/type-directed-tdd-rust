// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

//! FizzBuzz, implemented in a [type-directed way](https://github.com/FranklinChen/type-directed-tdd-rust) for a presentation for [Pittsburgh Code and Supply](http://codeandsupply.co/).

#![feature(core)]
#![feature(collections)]
#![feature(rand)]

extern crate quickcheck;

use std::iter::range_inclusive;

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
  for result in run_to_seq(1i, 100).iter() {
    println!("{}", result)
  }
}

/// Convert each integer to its correct string output.
///
/// Used to return an `Iterator` for maximum flexibility, but
/// Rust just changed to make this impossible without wrapping:
/// http://stackoverflow.com/questions/27496278/how-to-return-a-generic-map-struct/27497032#27497032
/// TODO: wait for Rust to provide clean way of returning `Iterator`.
#[inline]
fn run_to_seq(start: isize, end: isize) -> Vec<String> {
  range_inclusive(start, end)
    .map(defaults::fizzbuzzer)
    .collect::<Vec<String>>()
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
    let actual = run_to_seq(1, 16);
    let actual_slices = actual
      .iter()
      .map(|s| &*(*s))
      .collect::<Vec<&str>>();
    assert_eq!(actual_slices, expected_slices)
  }
}
