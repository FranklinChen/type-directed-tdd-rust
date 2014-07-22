pub type Pair<'a> = (int, &'a str);
pub struct Config(pub Pair, pub Pair);

pub fn evaluate(Config((d1, w1), (d2, w2)): Config, i: int)
                -> String {
  fail!()
}
