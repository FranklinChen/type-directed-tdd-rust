fn main() {
  // Will not compile yet!
  for result in run_to_seq(1i, 100).iter() {
    println!("{}", result)
  }
}
