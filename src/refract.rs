use super::*;

impl Vec2 {
  pub fn refract(self, normal: Self, refraction_index: f32) -> Self {
    let eta = refraction_index;
    let s_d_n = self.dot(normal);
    let k = 1.0 - ((eta * eta) * (1.0 - (s_d_n * s_d_n)));
    if k < 0.0 {
      Self::zeroed()
    } else {
      (eta * self) - (((eta * normal.dot(self)) + k.sqrt()) * normal)
    }
  }
}

impl Vec3 {
  pub fn refract(self, normal: Self, refraction_index: f32) -> Self {
    let eta = refraction_index;
    let s_d_n = self.dot(normal);
    let k = 1.0 - ((eta * eta) * (1.0 - (s_d_n * s_d_n)));
    if k < 0.0 {
      Self::zeroed()
    } else {
      (eta * self) - (((eta * normal.dot(self)) + k.sqrt()) * normal)
    }
  }
}

impl Vec4 {
  pub fn refract(self, normal: Self, refraction_index: f32) -> Self {
    let eta = refraction_index;
    let s_d_n = self.dot(normal);
    let k = 1.0 - ((eta * eta) * (1.0 - (s_d_n * s_d_n)));
    if k < 0.0 {
      Self::zeroed()
    } else {
      (eta * self) - (((eta * normal.dot(self)) + k.sqrt()) * normal)
    }
  }
}
