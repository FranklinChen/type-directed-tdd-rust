// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use std::iter::Iterator;

/// Convenient type synonym for a common pattern of usage.
/// More generally, Vec could be replaced by a Semigroup (Add).
pub type Validation<T, E> = Result<T, Vec<E>>;

#[inline]
pub fn single<T, E>(result: Result<T, E>) -> Validation<T, E> {
  result.map_err(|e| vec![e])
}

/// Combine successful results with `f`, but accumulate errors.
/// Important: any error causes the whole result to be an error!
///
/// Optimization trick: mutate result1's error vector.
pub fn add_with<V, T, U, E, F>(result1: Validation<V, E>,
                               result2: Validation<T, E>,
                               f: F) -> Validation<U, E>
  where F: FnOnce(V, T) -> U
{
  match (result1, result2) {
    (Ok(v),       Ok(t))   => Ok(f(v, t)),
    (Ok(..),      Err(e2)) => Err(e2),
    (Err(e1),     Ok(..))  => Err(e1),
    (Err(mut e1), Err(e2)) => Err({ e1.extend(e2.into_iter()); e1 })
  }
}

/// Combine a stream of Result to a Validation, accumulating successes.
///
/// Optimization trick: mutate success vector.
pub fn combine_results<T,
                       E,
                       I>(iter: I)
  -> Validation<Vec<T>, E> where
  I: Iterator<Item = Result<T, E>>
{
  iter.map(single)
      .fold(Ok(vec![]),
            |v, t|
            add_with(v, t,
                     |mut x, y| {
                       x.push(y);
                       x
                     }))
}
