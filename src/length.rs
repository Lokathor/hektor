use super::*;

impl Vec2 {
  pub fn length_squared(self) -> f32 {
    self.dot(self)
  }
  pub fn length(self) -> f32 {
    self.length_squared().sqrt()
  }
}

impl Vec3 {
  pub fn length_squared(self) -> f32 {
    self.dot(self)
  }
  pub fn length(self) -> f32 {
    self.length_squared().sqrt()
  }
}

impl Vec4 {
  pub fn length_squared(self) -> f32 {
    self.dot(self)
  }
  pub fn length(self) -> f32 {
    self.length_squared().sqrt()
  }
}
