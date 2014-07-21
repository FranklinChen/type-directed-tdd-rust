pub fn evaluate(Config(pairs): Config, i: int)
                -> String {
  // Can crash! And incorrect except for 2 pairs.
  let (d1, ref w1) = pairs[0];
  let (d2, ref w2) = pairs[1];

  match (i % d1 == 0, i % d2 == 0) {
    (true,  false) => (*w1).clone(),
    (false, true)  => (*w2).clone(),
    (true,  true)  => *w1 + *w2,
    (false, false) => i.to_string(),
  }
}
