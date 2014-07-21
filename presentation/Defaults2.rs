fn fizzbuzzpopper_config() -> Config {
  Config::new(vec![(3, "Fizz".to_string()),
                   (5, "Buzz".to_string()),
                   (7, "Pop".to_string())])
}

pub fn fizzbuzzpopper(i: int) -> String {
  fizzbuzz::evaluate(fizzbuzzpopper_config(), i)
}
