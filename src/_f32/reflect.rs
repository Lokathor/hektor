use super::*;

// R_reflect = R_in − (2N · R_in)N

impl Vec2 {
  /// inputs must be normalized!
  pub fn reflect(self, normal: Self) -> Self {
    self - (((2.0 * normal).dot(self)) * normal)
  }
}

impl Vec3 {
  /// inputs must be normalized!
  pub fn reflect(self, normal: Self) -> Self {
    self - (((2.0 * normal).dot(self)) * normal)
  }
}

impl Vec4 {
  /// inputs must be normalized!
  pub fn reflect(self, normal: Self) -> Self {
    self - (((2.0 * normal).dot(self)) * normal)
  }
}
