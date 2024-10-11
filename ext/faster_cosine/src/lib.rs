use magnus::{prelude::*, Error};

pub fn cosine_similarity(vector1: Vec<f64>, vector2: Vec<f64>) -> Option<f64> {
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

  Some(dot_product / (norm1 * norm2))
}

pub fn cosine_similarities(main_vector: Vec<f64>, vectors: Vec<Vec<f64>>) -> Vec<(usize, f64)> {
  let mut results: Vec<_> = vectors.into_iter().enumerate()
    .filter_map(|(idx, vector)| {
      cosine_similarity(main_vector.clone(), vector).map(|distance| (idx, distance))
    }).collect();

  results.sort_by(|a, b| b.1.total_cmp(&a.1));
  results
}

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<(), Error> {
  let class = ruby.define_class("FasterCosine", ruby.class_object())?;
  class.define_singleton_method("similarity", magnus::function!(cosine_similarity, 2))?;
  class.define_singleton_method("similarities", magnus::function!(cosine_similarities, 2))?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cosine_similarity_identical_vectors() {
    let vector = vec![0.179668877, 0.838925283, 0.078799599];
    let similarity = cosine_similarity(vector.clone(), vector.clone()).unwrap();
    assert!((similarity - 1.0).abs() < f64::EPSILON, "Expected similarity close to 1.0, got {}", similarity);
  }

  #[test]
  fn test_cosine_similarity_orthogonal_vectors() {
    let vector1 = vec![0.123456789, 0.987654321];
    let vector2 = vec![0.987654321, -0.123456789];
    let similarity = cosine_similarity(vector1, vector2).unwrap();
    assert!(similarity.abs() < 1e-6, "Similarity should be close to 0, got {}", similarity);
  }

  #[test]
  fn test_cosine_similarity_opposite_vectors() {
    let vector1 = vec![0.023051923, 0.708737462];
    let vector2 = vec![-0.023051923, -0.708737462];
    let similarity = cosine_similarity(vector1, vector2).unwrap();
    assert!((similarity + 1.0).abs() < 1e-6, "Similarity should be close to -1, got {}", similarity);
  }

  #[test]
  fn test_cosine_similarity_different_lengths() {
    let vector1 = vec![0.541289089, 0.675833429];
    let vector2 = vec![0.631605468, 0.993065893, 0.302933494];
    assert_eq!(cosine_similarity(vector1, vector2), None);
  }

  #[test]
  fn test_cosine_similarity_zero_vector() {
    let vector1 = vec![0.0, 0.0];
    let vector2 = vec![0.588091732, 0.668975486];
    assert_eq!(cosine_similarity(vector1, vector2), None);
  }

  #[test]
  fn test_cosine_similarities() {
    let main_vector = vec![0.403601789, 0.240241778];
    let vectors = vec![
      vec![0.153490871, 0.887654321],
      vec![0.032816935, 0.384659721],
      vec![0.260898325, 0.051766647],
      vec![0.241973366, 0.007815162]
    ];

    let similarities = cosine_similarities(main_vector.clone(), vectors.clone());

    let expected = vec![
      (2, 0.942406456612674),
      (3, 0.875353837667601),
      (0, 0.650421911960787),
      (1, 0.5826809675242339)
    ];

    for ((idx, sim), (exp_idx, exp_sim)) in similarities.iter().zip(expected.iter()) {
      assert_eq!(*idx, *exp_idx);
      assert!((*sim - *exp_sim).abs() < 1e-6, "Similarity mismatch at index {}: expected {}, got {}", idx, exp_sim, sim);
    }
  }
}
