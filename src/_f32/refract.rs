use super::*;

impl Vec2 {
  /// inputs must be normalized!
  pub fn refract(self, normal: Self, refraction_index: f32) -> Self {
    let eta = refraction_index;
    let s_d_n = self.dot(normal);
    let k = 1.0 - ((eta * eta) * (1.0 - (s_d_n * s_d_n)));
    if k < 0.0 {
      <Self as bytemuck::Zeroable>::zeroed()
    } else {
      (eta * self) - (((eta * normal.dot(self)) + k.sqrt()) * normal)
    }
  }
}

impl Vec3 {
  /// inputs must be normalized!
  pub fn refract(self, normal: Self, refraction_index: f32) -> Self {
    let eta = refraction_index;
    let s_d_n = self.dot(normal);
    let k = 1.0 - ((eta * eta) * (1.0 - (s_d_n * s_d_n)));
    if k < 0.0 {
      <Self as bytemuck::Zeroable>::zeroed()
    } else {
      (eta * self) - (((eta * normal.dot(self)) + k.sqrt()) * normal)
    }
  }
}

impl Vec4 {
  /// inputs must be normalized!
  pub fn refract(self, normal: Self, refraction_index: f32) -> Self {
    let eta = refraction_index;
    let s_d_n = self.dot(normal);
    let k = 1.0 - ((eta * eta) * (1.0 - (s_d_n * s_d_n)));
    if k < 0.0 {
      <Self as bytemuck::Zeroable>::zeroed()
    } else {
      (eta * self) - (((eta * normal.dot(self)) + k.sqrt()) * normal)
    }
  }
}
