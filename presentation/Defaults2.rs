fn fizzbuzzpopper_config<'a>() -> Config<'a> {
  Config::new(vec![(3, "Fizz"),
                   (5, "Buzz"),
                   (7, "Pop")])
}

pub fn fizzbuzzpopper(i: int) -> String {
  fizzbuzz::evaluate(fizzbuzzpopper_config(), i)
}
