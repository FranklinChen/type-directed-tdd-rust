// Cannot be static variable because of runtime
// validation and also use of Vector.
fn fizzbuzzer_config<'a>() -> Config<'a> {
  Config::new(vec![(3, "Fizz"),
                   (5, "Buzz")])
}

pub fn fizzbuzzer(i: int) -> String {
  fizzbuzz::evaluate(fizzbuzzer_config(), i)
}
