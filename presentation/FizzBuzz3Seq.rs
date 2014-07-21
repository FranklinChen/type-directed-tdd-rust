pub struct Config(pub Vec<Pair>);

impl Config {
  pub fn new(pairs: Vec<Pair>) -> Config {
    for pair in pairs.iter() {
      validate_pair(pair);
    }
    Config(pairs)
  }
}
