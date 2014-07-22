// Rust limitation: cannot be static variable
// because Config stores String.
fn fizzbuzzer_config<'a>() -> Config<'a> {
  Config((3, "Fizz"),
         (5, "Buzz"))
}

pub fn fizzbuzzer(i: int) -> String {
  fizzbuzz::evaluate(fizzbuzzer_config(), i)
}
