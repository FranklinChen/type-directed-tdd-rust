pub fn evaluate(Config((d1, w1), (d2, w2)): Config, i: int)
                -> String {
  match (i % d1 == 0, i % d2 == 0) {
    (true,  false) => w1,
    (false, true)  => w2,
    (true,  true)  => w1 + w2,
    (false, false) => i.to_string(),
  }
}
