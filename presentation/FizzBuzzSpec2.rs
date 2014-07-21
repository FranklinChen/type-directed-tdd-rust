  #[quickcheck]
  fn multiple_of_both_3_and_5(i: int) -> TestResult {
    if i % 3 == 0 && i % 5 == 0 {
      TestResult::from_bool(evaluate(i) ==
                            "FizzBuzz".to_string())
    } else {
      TestResult::discard()
    }
  }
