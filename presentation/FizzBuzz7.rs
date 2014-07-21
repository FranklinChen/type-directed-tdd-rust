pub fn evaluate(Config(pairs): Config, i: int)
                -> String {
  let combined: Option<String> = pairs.iter()
            .map(|pair| rule(pair, i))
            .fold(None, add_option);
  combined.unwrap_or_else(|| i.to_string())
}
