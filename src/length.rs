use super::*;

impl Vec2 {
  pub fn length_squared(self) -> f32 {
    self.dot(self)
  }
  pub fn length(self) -> f32 {
    self.length_squared().sqrt()
  }
  pub fn normalized(self) -> Self {
    let l = self.length();
    debug_assert!(l != 0.0);
    Self { x: self.x / l, y: self.y / l }
  }
}

impl Vec3 {
  pub fn length_squared(self) -> f32 {
    self.dot(self)
  }
  pub fn length(self) -> f32 {
    self.length_squared().sqrt()
  }
  pub fn normalized(self) -> Self {
    let l = self.length();
    debug_assert!(l != 0.0);
    Self { x: self.x / l, y: self.y / l, z: self.z / l }
  }
}

impl Vec4 {
  pub fn length_squared(self) -> f32 {
    self.dot(self)
  }
  pub fn length(self) -> f32 {
    self.length_squared().sqrt()
  }
  pub fn normalized(self) -> Self {
    let l = self.length();
    debug_assert!(l != 0.0);
    Self { x: self.x / l, y: self.y / l, z: self.z / l, w: self.w / l }
  }
}
