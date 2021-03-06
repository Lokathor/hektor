use super::*;

impl Mat2x2 {
  pub fn determinant(self) -> f32 {
    self.r0c0 * self.r1c1 - self.r0c1 * self.r1c0
  }
}
/*
impl DMat2x2 {
  pub fn determinant(self) -> f64 {
    self.r0c0 * self.r1c1 - self.r0c1 * self.r1c0
  }
}
impl IMat2x2 {
  pub fn determinant(self) -> i32 {
    self.r0c0 * self.r1c1 - self.r0c1 * self.r1c0
  }
}
*/

// Note(Lokathor): For 3x3 and 4x4 we try to intermix the + and - terms so that
// we're less likely to hit the edge of each numeric type.

impl Mat3x3 {
  #[rustfmt::skip]
  pub fn determinant(self) -> f32 {
    let Self {
      r0c0: a11, r0c1: a12, r0c2: a13,
      r1c0: a21, r1c1: a22, r1c2: a23,
      r2c0: a31, r2c1: a32, r2c2: a33, } = self;
    //
    a11*a22*a33 - a13*a22*a31 + a12*a23*a31 - a12*a21*a33 + a13*a21*a32 - a11*a23*a32
  }
}
/*
impl DMat3x3 {
  #[rustfmt::skip]
  pub fn determinant(self) -> f64 {
    let Self {
      r0c0: a11, r0c1: a12, r0c2: a13,
      r1c0: a21, r1c1: a22, r1c2: a23,
      r2c0: a31, r2c1: a32, r2c2: a33, } = self;
    //
    a11*a22*a33 - a13*a22*a31 + a12*a23*a31 - a12*a21*a33 + a13*a21*a32 - a11*a23*a32
  }
}
impl IMat3x3 {
  #[rustfmt::skip]
  pub fn determinant(self) -> i32 {
    let Self {
      r0c0: a11, r0c1: a12, r0c2: a13,
      r1c0: a21, r1c1: a22, r1c2: a23,
      r2c0: a31, r2c1: a32, r2c2: a33, } = self;
    //
    a11*a22*a33 - a13*a22*a31 + a12*a23*a31 - a12*a21*a33 + a13*a21*a32 - a11*a23*a32
  }
}
*/

//

impl Mat4x4 {
  #[rustfmt::skip]
  pub fn determinant(self) -> f32 {
    let Self {
      r0c0: a11, r0c1: a12, r0c2: a13, r0c3: a14,
      r1c0: a21, r1c1: a22, r1c2: a23, r1c3: a24,
      r2c0: a31, r2c1: a32, r2c2: a33, r2c3: a34,
      r3c0: a41, r3c1: a42, r3c2: a43, r3c3: a44, } = self;
    //
    a11*a22*a33*a44
    -a11*a22*a34*a43
    -a11*a23*a32*a44
    +a11*a23*a34*a42
    +a11*a24*a32*a43
    -a11*a24*a33*a42
    //
    -a12*a21*a33*a44
    +a12*a21*a34*a43
    +a12*a23*a31*a44
    -a12*a23*a34*a41
    -a12*a24*a31*a43
    +a12*a24*a33*a41
    //
    +a13*a21*a32*a44
    -a13*a21*a34*a42
    -a13*a22*a31*a44
    +a13*a22*a34*a41
    +a13*a24*a31*a42
    -a13*a24*a32*a41
    //
    -a14*a21*a32*a43
    +a14*a21*a33*a42
    +a14*a22*a31*a43
    -a14*a22*a33*a41
    -a14*a23*a31*a42
    +a14*a23*a32*a41
  }
}
/*
impl DMat4x4 {
  #[rustfmt::skip]
  pub fn determinant(self) -> f64 {
    let Self {
      r0c0: a11, r0c1: a12, r0c2: a13, r0c3: a14,
      r1c0: a21, r1c1: a22, r1c2: a23, r1c3: a24,
      r2c0: a31, r2c1: a32, r2c2: a33, r2c3: a34,
      r3c0: a41, r3c1: a42, r3c2: a43, r3c3: a44, } = self;
    //
    a11*a22*a33*a44
    -a11*a22*a34*a43
    -a11*a23*a32*a44
    +a11*a23*a34*a42
    +a11*a24*a32*a43
    -a11*a24*a33*a42
    //
    -a12*a21*a33*a44
    +a12*a21*a34*a43
    +a12*a23*a31*a44
    -a12*a23*a34*a41
    -a12*a24*a31*a43
    +a12*a24*a33*a41
    //
    +a13*a21*a32*a44
    -a13*a21*a34*a42
    -a13*a22*a31*a44
    +a13*a22*a34*a41
    +a13*a24*a31*a42
    -a13*a24*a32*a41
    //
    -a14*a21*a32*a43
    +a14*a21*a33*a42
    +a14*a22*a31*a43
    -a14*a22*a33*a41
    -a14*a23*a31*a42
    +a14*a23*a32*a41
  }
}
impl IMat4x4 {
  #[rustfmt::skip]
  pub fn determinant(self) -> i32 {
    let Self {
      r0c0: a11, r0c1: a12, r0c2: a13, r0c3: a14,
      r1c0: a21, r1c1: a22, r1c2: a23, r1c3: a24,
      r2c0: a31, r2c1: a32, r2c2: a33, r2c3: a34,
      r3c0: a41, r3c1: a42, r3c2: a43, r3c3: a44, } = self;
    //
    a11*a22*a33*a44
    -a11*a22*a34*a43
    -a11*a23*a32*a44
    +a11*a23*a34*a42
    +a11*a24*a32*a43
    -a11*a24*a33*a42
    //
    -a12*a21*a33*a44
    +a12*a21*a34*a43
    +a12*a23*a31*a44
    -a12*a23*a34*a41
    -a12*a24*a31*a43
    +a12*a24*a33*a41
    //
    +a13*a21*a32*a44
    -a13*a21*a34*a42
    -a13*a22*a31*a44
    +a13*a22*a34*a41
    +a13*a24*a31*a42
    -a13*a24*a32*a41
    //
    -a14*a21*a32*a43
    +a14*a21*a33*a42
    +a14*a22*a31*a43
    -a14*a22*a33*a41
    -a14*a23*a31*a42
    +a14*a23*a32*a41
  }
}
*/
