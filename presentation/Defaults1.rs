// Rust limitation: cannot be static variable
// because Config stores String.
fn fizzbuzzer_config() -> Config {
  Config((3, "Fizz".to_string()),
         (5, "Buzz".to_string()))
}

pub fn fizzbuzzer(i: int) -> String {
  fizzbuzz::evaluate(fizzbuzzer_config(), i)
}
