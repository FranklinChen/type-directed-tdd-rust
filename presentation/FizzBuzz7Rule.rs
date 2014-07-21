fn rule(&(n, ref word): &Pair, i: int) -> Option<String> {
  if i % n == 0 {
    Some((*word).clone())
  } else {
    None
  }
}

