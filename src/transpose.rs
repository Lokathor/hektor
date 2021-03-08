use super::*;

impl Mat2x2 {
  pub fn transpose(self) -> Mat2x2 {
    let s = self.as_slice();
    mat2x2_rows([s[0], s[1]], [s[2], s[3]])
  }
}

impl Mat2x3 {
  pub fn transpose(self) -> Mat3x2 {
    let s = self.as_slice();
    mat3x2_rows([s[0], s[1]], [s[2], s[3]], [s[4], s[5]])
  }
}

impl Mat2x4 {
  pub fn transpose(self) -> Mat4x2 {
    let s = self.as_slice();
    mat4x2_rows([s[0], s[1]], [s[2], s[3]], [s[4], s[5]], [s[6], s[7]])
  }
}

impl Mat3x2 {
  pub fn transpose(self) -> Mat2x3 {
    let s = self.as_slice();
    mat2x3_rows([s[0], s[1], s[2]], [s[3], s[4], s[5]])
  }
}

impl Mat3x3 {
  pub fn transpose(self) -> Mat3x3 {
    let s = self.as_slice();
    mat3x3_rows([s[0], s[1], s[2]], [s[3], s[4], s[5]], [s[6], s[7], s[8]])
  }
}

impl Mat3x4 {
  pub fn transpose(self) -> Mat4x3 {
    let s = self.as_slice();
    mat4x3_rows(
      [s[0], s[1], s[2]],
      [s[3], s[4], s[5]],
      [s[6], s[7], s[8]],
      [s[9], s[10], s[11]],
    )
  }
}

impl Mat4x2 {
  pub fn transpose(self) -> Mat2x4 {
    let s = self.as_slice();
    mat2x4_rows([s[0], s[1], s[2], s[3]], [s[4], s[5], s[6], s[7]])
  }
}

impl Mat4x3 {
  pub fn transpose(self) -> Mat3x4 {
    let s = self.as_slice();
    mat3x4_rows(
      [s[0], s[1], s[2], s[3]],
      [s[4], s[5], s[6], s[7]],
      [s[8], s[9], s[10], s[11]],
    )
  }
}

impl Mat4x4 {
  pub fn transpose(self) -> Mat4x4 {
    let s = self.as_slice();
    mat4x4_rows(
      [s[0], s[1], s[2], s[3]],
      [s[4], s[5], s[6], s[7]],
      [s[8], s[9], s[10], s[11]],
      [s[12], s[13], s[14], s[15]],
    )
  }
}
