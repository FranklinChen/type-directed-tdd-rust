// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

/// Convenient type synonym for a common pattern of usage.
/// More generally, Vec could be replaced by a Semigroup (Add).
pub type Validation<T, E> = Result<T, Vec<E>>;

/// Combine successful results with `f`, but accumulate errors.
/// Important: any error causes the whole result to be an error!
pub fn add_with<V, T, U, E>(result1: Validation<V, E>,
                            result2: Validation<T, E>,
                            f: |V, T| -> U) -> Validation<U, E> {
  match (result1, result2) {
    (Ok(v),       Ok(t))   => Ok(f(v, t)),
    (Ok(_),       Err(e2)) => Err(e2),
    (Err(e1),     Ok(_))   => Err(e1),
    (Err(mut e1), Err(e2)) => Err({ e1.extend(e2.into_iter()); e1 })
  }
}
