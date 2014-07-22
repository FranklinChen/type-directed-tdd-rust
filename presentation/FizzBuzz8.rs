fn add_option(a1: Option<String>, a2: Option<String>)
              -> Option<String> {
  match (a1, a2) {
    (Some(s1), None)     => Some(s1),
    (None,     Some(s2)) => Some(s2),
    (Some(s1), Some(s2)) => Some(s1.append(s2)),
    (None,     None)     => None,
  }
}
