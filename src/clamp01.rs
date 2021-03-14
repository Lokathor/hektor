/// Provides the `clamp01` operation.
///
/// The clamped output will be in the range `0.0 ..= 1.0` (the "unit" range).
///
/// ## Safety
/// * All possible input values must be clamped into the unit range.
/// * Infinite values must become a finite value:
///   * Infinity becomes `1.0`
///   * Negative Infinity becomes `0.0`
///   * NaN becomes `0.0`
/// * The value `-0.0` must have the sign bit cleared.
pub unsafe trait Clamp01 {
  /// Clamps the value to the range `0.0 ..= 1.0`
  fn clamp01(self) -> Self;
}

// TODO: impls for vector and matrix types?

unsafe impl Clamp01 for f32 {
  fn clamp01(self) -> f32 {
    if self > 0.0 {
      if self <= 1.0 {
        self
      } else {
        1.0
      }
    } else {
      0.0
    }
  }
}

unsafe impl Clamp01 for f64 {
  fn clamp01(self) -> f64 {
    if self > 0.0 {
      if self <= 1.0 {
        self
      } else {
        1.0
      }
    } else {
      0.0
    }
  }
}

/// A free-function way to use the [`Clamp01`] trait.
pub fn clamp01<C: Clamp01>(x: C) -> C {
  x.clamp01()
}
