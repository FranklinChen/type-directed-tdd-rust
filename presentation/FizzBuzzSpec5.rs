  #[test] fn test_fizzbuzzpopper_2() {
    assert_eq!(fizzbuzzpopper(2), "2".to_string())
  }
  #[test] fn test_fizzbuzzpopper_21() {
    assert_eq!(fizzbuzzpopper(21), "FizzPop".to_string())
  }
  #[test] fn test_fizzbuzzpopper_9() {
    assert_eq!(fizzbuzzpopper(9), "Fizz".to_string())
  }
  #[test] fn test_fizzbuzzpopper_7() {
    assert_eq!(fizzbuzzpopper(7), "Pop".to_string())
  }
  #[test] fn test_fizzbuzzpopper_35() {
    assert_eq!(fizzbuzzpopper(35), "BuzzPop".to_string())
  }
