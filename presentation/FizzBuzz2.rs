pub fn evaluate(i: int) -> String {
  match (i % 3 == 0, i % 5 == 0) {
    (true,  false) => "Fizz".to_string(),
    (false, true)  => "Buzz".to_string(),
    (true,  true)  => "FizzBuzz".to_string(),
    (false, false) => i.to_string(),
  }
}
