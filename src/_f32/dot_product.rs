//! Dot Product: component-wise multiplication between two vectors, then
//! horizontal sum of the elements.

use super::*;

impl Vec2 {
  pub fn dot(self, other: Self) -> f32 {
    self.x * other.x + self.y * other.y
  }
}

impl Vec3 {
  pub fn dot(self, other: Self) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Vec4 {
  pub fn dot(self, other: Self) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
  }
}
