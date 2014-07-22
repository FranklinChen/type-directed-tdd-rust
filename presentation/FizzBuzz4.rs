// Make the decision to allocate a String here.
fn rule(&(n, word): &Pair, i: int) -> String {
  if i % n == 0 {
    word.to_string()
  } else {
    String::new()
  }
}
