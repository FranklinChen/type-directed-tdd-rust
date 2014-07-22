  #[quickcheck]
  fn d1_but_not_d2((d1, w1): (int, String),
                   (d2, w2): (int, String),
                   i: int) -> TestResult {
    if (d1 >= DIVISOR_MIN && d1 <= DIVISOR_MAX)
      && (d2 >= DIVISOR_MIN && d2 <= DIVISOR_MAX) {
      let config = Config::new(vec![(d1, w1.as_slice()),
                                    (d2, w2.as_slice())]);
      
      if i % d1 == 0 && i % d2 != 0 {
        TestResult::from_bool(evaluate(config, i) == w1)
      } else { TestResult::discard() }
    } else { TestResult::discard() }
  }
