  #[quickcheck]
  fn d1_but_not_d2(p1: Pair, p2: Pair, i: int) -> TestResult {
    let (d1, w1) = p1.clone();
    let (d2, _) = p2;

    if (d1 >= DIVISOR_MIN && d1 <= DIVISOR_MAX)
      && (d2 >= DIVISOR_MIN && d2 <= DIVISOR_MAX) {
      let config = Config::new(p1, p2);
      
      if i % d1 == 0 && i % d2 != 0 {
        TestResult::from_bool(evaluate(config, i) == w1)
      } else { TestResult::discard() }
    } else { TestResult::discard() }
  }
