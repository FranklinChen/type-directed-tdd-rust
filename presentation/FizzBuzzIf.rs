  // Buggy and ugly!
  if i % 3 == 0 {
    "Fizz".to_string()
  } else if i % 5 == 0 {
    "Buzz".to_string()
  } else if i % 3 == 0 && i % 5 == 0 {
    "FizzBuzz".to_string()
  } else {
    i.to_string()
  }
