fn run_to_seq(start: int, end: int) -> Vec<String> {
  range_inclusive(start, end)
    .map(defaults::fizzbuzzer)
    .collect()
}
