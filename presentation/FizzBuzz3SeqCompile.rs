pub fn evaluate(Config(pairs): Config, i: int)
                -> String {
  // Can crash! And incorrect except for 2 pairs.
  let (d1, w1) = pairs[0];
  let (d2, w2) = pairs[1];

  match (i % d1 == 0, i % d2 == 0) {
    (true,  false) => w1.to_string(),
    (false, true)  => w2.to_string(),
    (true,  true)  => w1.to_string().append(w2),
    (false, false) => i.to_string(),
  }
}
