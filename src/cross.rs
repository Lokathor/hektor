use crate::Vec3;

impl Vec3 {
  /// order matters.
  pub fn cross(self, other: Self) -> Self {
    Self {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }
}
