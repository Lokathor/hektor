use super::*;

impl Mat2x2 {
  pub const fn identity() -> Self {
    Self { r0c0: 1.0, r1c0: 0.0, r0c1: 0.0, r1c1: 1.0 }
  }
}

impl DMat2x2 {
  pub const fn identity() -> Self {
    Self { r0c0: 1.0, r1c0: 0.0, r0c1: 0.0, r1c1: 1.0 }
  }
}

impl IMat2x2 {
  pub const fn identity() -> Self {
    Self { r0c0: 1, r1c0: 0, r0c1: 0, r1c1: 1 }
  }
}

//

impl Mat3x3 {
  pub const fn identity() -> Self {
    Self { r0c0: 1.0, r1c0: 0.0, r2c0: 0.0, r0c1: 0.0, r1c1: 1.0, r2c1: 0.0, r0c2: 0.0, r1c2: 0.0, r2c2: 1.0 }
  }
}

impl DMat3x3 {
  pub const fn identity() -> Self {
    Self { r0c0: 1.0, r1c0: 0.0, r2c0: 0.0, r0c1: 0.0, r1c1: 1.0, r2c1: 0.0, r0c2: 0.0, r1c2: 0.0, r2c2: 1.0 }
  }
}

impl IMat3x3 {
  pub const fn identity() -> Self {
    Self { r0c0: 1, r1c0: 0, r2c0: 0, r0c1: 0, r1c1: 1, r2c1: 0, r0c2: 0, r1c2: 0, r2c2: 1 }
  }
}

//

impl Mat4x4 {
  pub const fn identity() -> Self {
    Self { r0c0: 1.0, r1c0: 0.0, r2c0: 0.0, r3c0: 0.0, r0c1: 0.0, r1c1: 1.0, r2c1: 0.0, r3c1: 0.0, r0c2: 0.0, r1c2: 0.0, r2c2: 1.0, r3c2: 0.0, r0c3: 0.0, r1c3: 0.0, r2c3: 0.0, r3c3: 1.0 }
  }
}

impl DMat4x4 {
  pub const fn identity() -> Self {
    Self { r0c0: 1.0, r1c0: 0.0, r2c0: 0.0, r3c0: 0.0, r0c1: 0.0, r1c1: 1.0, r2c1: 0.0, r3c1: 0.0, r0c2: 0.0, r1c2: 0.0, r2c2: 1.0, r3c2: 0.0, r0c3: 0.0, r1c3: 0.0, r2c3: 0.0, r3c3: 1.0 }
  }
}

impl IMat4x4 {
  pub const fn identity() -> Self {
    Self { r0c0: 1, r1c0: 0, r2c0: 0, r3c0: 0, r0c1: 0, r1c1: 1, r2c1: 0, r3c1: 0, r0c2: 0, r1c2: 0, r2c2: 1, r3c2: 0, r0c3: 0, r1c3: 0, r2c3: 0, r3c3: 1 }
  }
}
