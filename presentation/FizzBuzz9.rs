pub fn evaluate(Config(pairs): Config, i: int)
                -> String {
  pairs.par
       .iter()
       .map(|pair| rule(pair, i))
       .reduce(add_option)
       .unwrap_or_else(|| i.to_string())
}
