fn rule(&(n, ref word): &Pair, i: int) -> String {
  if i % n == 0 {
    (*word).clone()
  } else {
    String::new()
  }
}
