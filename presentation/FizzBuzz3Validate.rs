static DIVISOR_MIN: int = 2; static DIVISOR_MAX: int = 100;

fn validate_pair(&(d, _): &Pair) {
  assert!(d >= DIVISOR_MIN,
          "divisor {} must be >= {}", d, DIVISOR_MIN);
  assert!(d <= DIVISOR_MAX,
          "divisor {} must be <= {}", d, DIVISOR_MAX);
}

impl<'a> Config<'a> {
  pub fn new(pair1: Pair, pair2: Pair) -> Config {
    validate_pair(&pair1); validate_pair(&pair2);
    Config(pair1, pair2)
  }
}
