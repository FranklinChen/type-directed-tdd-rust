  #[quickcheck]
  fn multiple_of_only_3(i: int) -> TestResult {
    if i % 3 == 0 && i % 5 != 0 {
      TestResult::from_bool(evaluate(i) == "Fizz".to_string())
    } else {
      TestResult::discard()
    }
  }

  #[quickcheck]
  fn not_multiple_of_3_and_5(i: int) -> TestResult {
    if i % 3 != 0 && i % 5 != 0 {
      TestResult::from_bool(evaluate(i) == i.to_string())
    } else {
      TestResult::discard()
    }
  }
