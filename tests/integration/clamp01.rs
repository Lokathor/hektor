use super::*;

#[test]
fn test_impl_clamp01_for_f32() {
  for input in
    [-0.0, 0.0, 1.0, 1.1, 0.567, f32::NAN, f32::INFINITY, f32::NEG_INFINITY]
      .iter()
      .copied()
  {
    let f = input.clamp01();
    assert!(f >= 0.0);
    assert!(f <= 1.0);
    assert!(f.is_sign_positive());
    assert!(f.is_finite());
  }
}

#[test]
fn test_impl_clamp01_for_f64() {
  for input in
    [-0.0, 0.0, 1.0, 1.1, 0.567, f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
      .iter()
      .copied()
  {
    let f = input.clamp01();
    assert!(f >= 0.0);
    assert!(f <= 1.0);
    assert!(f.is_sign_positive());
    assert!(f.is_finite());
  }
}
