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

impl DVec2 {
  pub fn dot(self, other: Self) -> f64 {
    self.x * other.x + self.y * other.y
  }
}

impl DVec3 {
  pub fn dot(self, other: Self) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl DVec4 {
  pub fn dot(self, other: Self) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
  }
}

impl IVec2 {
  pub fn dot(self, other: Self) -> i32 {
    self.x * other.x + self.y * other.y
  }
}

impl IVec3 {
  pub fn dot(self, other: Self) -> i32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl IVec4 {
  pub fn dot(self, other: Self) -> i32 {
    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
  }
}
