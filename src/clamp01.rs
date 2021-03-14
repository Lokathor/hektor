/// Provides the `clamp01` operation.
///
/// ## Safety
/// Because this trait is `unsafe`, other `unsafe` code can rely on all
/// implementations of this trait upholding the contract.
///
/// Most importantly, when you use the output of this method to scale a value
/// within a 0 to X range, you can be assured that it's still in that range.
pub unsafe trait Clamp01 {
  /// Clamps a floating value to `0.0 ..= 1.0`.
  ///
  /// * All possible input values must be clamped into the specified range.
  /// * Infinity becomes 1.0, and Negative Infinity becomes 0.0.
  /// * NaN becomes an implementation defined value in range.
  /// * -0.0 must have the sign bit cleared.
  fn clamp01(self) -> Self;
}

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

// TODO: impls for vector and matrix types?

/// A free-function way to use the [`Clamp01`] trait.
pub fn clamp01<C: Clamp01>(x: C) -> C {
  x.clamp01()
}
