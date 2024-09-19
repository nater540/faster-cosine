use magnus::{prelude::*, Error};

pub fn cosine_distance(vector1: Vec<f64>, vector2: Vec<f64>) -> Option<f64> {
  if vector1.len() != vector2.len() {
    return None;
  }

  let (dot_product, sum_sq1, sum_sq2) = vector1
    .iter()
    .zip(vector2.iter())
    .fold((0.0, 0.0, 0.0), |(dp, sq1, sq2), (&x, &y)| {
      (dp + x * y, sq1 + x * x, sq2 + y * y)
    });

  let norm1 = sum_sq1.sqrt();
  let norm2 = sum_sq2.sqrt();

  if norm1 == 0.0 || norm2 == 0.0 {
    return None;
  }

  Some(1.0 - (dot_product / (norm1 * norm2)))
}

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<(), Error> {
  let class = ruby.define_class("FasterCosine", ruby.class_object())?;
  class.define_singleton_method("distance", magnus::function!(cosine_distance, 2))?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
