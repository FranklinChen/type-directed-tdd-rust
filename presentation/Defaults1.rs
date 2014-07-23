// We can store a static Config as in C.
static fizzbuzzer_config: Config<'static> =
  Config((3, "Fizz"),
         (5, "Buzz"));

pub fn fizzbuzzer(i: int) -> String {
  fizzbuzz::evaluate(fizzbuzzer_config, i)
}
