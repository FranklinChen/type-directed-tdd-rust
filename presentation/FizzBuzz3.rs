pub type Pair<'a> = (int, &'a str);
pub struct Config<'a>(pub Pair<'a>, pub Pair<'a>);

pub fn evaluate(Config((d1, w1), (d2, w2)): Config, i: int)
                -> String {
  fail!()
}
