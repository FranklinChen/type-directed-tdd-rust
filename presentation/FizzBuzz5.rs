pub fn evaluate(Config(pairs): Config, i: int)
                -> String {
  let combined: String = pairs.iter()
            .map(|pair| rule(pair, i))
            .fold(String::new(),
                  |result, s| result + s);
  if combined.is_empty() {
    i.to_string()
  } else {
    combined
  }
}
