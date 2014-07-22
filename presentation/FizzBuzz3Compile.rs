pub fn evaluate(Config((d1, w1), (d2, w2)): Config, i: int)
                -> String {
  match (i % d1 == 0, i % d2 == 0) {
    (true,  false) => w1.to_string(),
    (false, true)  => w2.to_string(),
    (true,  true)  => w1.to_string().append(w2),
    (false, false) => i.to_string(),
  }
}
