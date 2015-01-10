/*
  Add options, specialized for String.

  Option<String> is a Monoid since String is a Semigroup (Add in
  Rust-speak). Rust also has a Zero, so a Monoid could be Zero + Add.

  Use an isizeeresting optimization: push_str reuses the isizeernal buffer
  of the first string. One could imagine a special ReuseAdd trait
  to support any type that knows how to reuse its isizeernal buffer.
*/
pub fn add(a1: Option<String>, a2: Option<String>) -> Option<String> {
  match (a1, a2) {
    (Some(s1),     None)     => Some(s1),
    (None,         Some(s2)) => Some(s2),
    (Some(mut s1), Some(s2)) => Some({ s1.push_str(s2.as_slice()); s1 }),
    (None,         None)     => None,
  }
}
