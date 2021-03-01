use super::*;

use core::ops::Mul;

#[rustfmt::skip]
impl Mul<Vec2> for Mat2x2 {
  type Output = Vec2;

  fn mul(self, v: Vec2) -> Vec2 {
    let m = self;
    Vec2 {
      x: vec2(m.r0c0, m.r0c1).dot(v),
      y: vec2(m.r1c0, m.r1c1).dot(v),
    }
  }
}

#[rustfmt::skip]
impl Mul<Mat2x2> for Mat2x2 {
  type Output = Mat2x2;

  fn mul(self, k: Mat2x2) -> Mat2x2 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    Mat2x2 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
    }
  }
}

#[rustfmt::skip]
impl Mul<Mat2x3> for Mat2x2 {
  type Output = Mat2x3;

  fn mul(self, _: Mat2x3) -> Mat2x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat2x4> for Mat2x2 {
  type Output = Mat2x4;

  fn mul(self, _: Mat2x4) -> Mat2x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec3> for Mat2x3 {
  type Output = Vec2;

  fn mul(self, _: Vec3) -> Vec2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x2> for Mat2x3 {
  type Output = Mat2x2;

  fn mul(self, _: Mat3x2) -> Mat2x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x3> for Mat2x3 {
  type Output = Mat2x3;

  fn mul(self, _: Mat3x3) -> Mat2x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x4> for Mat2x3 {
  type Output = Mat2x4;

  fn mul(self, _: Mat3x4) -> Mat2x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec4> for Mat2x4 {
  type Output = Vec2;

  fn mul(self, _: Vec4) -> Vec2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x2> for Mat2x4 {
  type Output = Mat2x2;

  fn mul(self, _: Mat4x2) -> Mat2x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x3> for Mat2x4 {
  type Output = Mat2x3;

  fn mul(self, _: Mat4x3) -> Mat2x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x4> for Mat2x4 {
  type Output = Mat2x4;

  fn mul(self, _: Mat4x4) -> Mat2x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec2> for Mat3x2 {
  type Output = Vec3;

  fn mul(self, _: Vec2) -> Vec3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat2x2> for Mat3x2 {
  type Output = Mat3x2;

  fn mul(self, _: Mat2x2) -> Mat3x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat2x3> for Mat3x2 {
  type Output = Mat3x3;

  fn mul(self, _: Mat2x3) -> Mat3x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat2x4> for Mat3x2 {
  type Output = Mat3x4;

  fn mul(self, _: Mat2x4) -> Mat3x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec3> for Mat3x3 {
  type Output = Vec3;

  fn mul(self, _: Vec3) -> Vec3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x2> for Mat3x3 {
  type Output = Mat3x2;

  fn mul(self, _: Mat3x2) -> Mat3x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x3> for Mat3x3 {
  type Output = Mat3x3;

  fn mul(self, _: Mat3x3) -> Mat3x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x4> for Mat3x3 {
  type Output = Mat3x4;

  fn mul(self, _: Mat3x4) -> Mat3x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec4> for Mat3x4 {
  type Output = Vec3;

  fn mul(self, _: Vec4) -> Vec3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x2> for Mat3x4 {
  type Output = Mat3x2;

  fn mul(self, _: Mat4x2) -> Mat3x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x3> for Mat3x4 {
  type Output = Mat3x3;

  fn mul(self, _: Mat4x3) -> Mat3x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x4> for Mat3x4 {
  type Output = Mat3x4;

  fn mul(self, _: Mat4x4) -> Mat3x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec2> for Mat4x2 {
  type Output = Vec4;

  fn mul(self, _: Vec2) -> Vec4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat2x2> for Mat4x2 {
  type Output = Mat4x2;

  fn mul(self, _: Mat2x2) -> Mat4x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat2x3> for Mat4x2 {
  type Output = Mat4x3;

  fn mul(self, _: Mat2x3) -> Mat4x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat2x4> for Mat4x2 {
  type Output = Mat4x4;

  fn mul(self, _: Mat2x4) -> Mat4x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec3> for Mat4x3 {
  type Output = Vec4;

  fn mul(self, _: Vec3) -> Vec4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x2> for Mat4x3 {
  type Output = Mat4x2;

  fn mul(self, _: Mat3x2) -> Mat4x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x3> for Mat4x3 {
  type Output = Mat4x3;

  fn mul(self, _: Mat3x3) -> Mat4x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat3x4> for Mat4x3 {
  type Output = Mat4x4;

  fn mul(self, _: Mat3x4) -> Mat4x4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Vec4> for Mat4x4 {
  type Output = Vec4;

  fn mul(self, _: Vec4) -> Vec4 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x2> for Mat4x4 {
  type Output = Mat4x2;

  fn mul(self, _: Mat4x2) -> Mat4x2 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x3> for Mat4x4 {
  type Output = Mat4x3;

  fn mul(self, _: Mat4x3) -> Mat4x3 {
    todo!()
  }
}

#[rustfmt::skip]
impl Mul<Mat4x4> for Mat4x4 {
  type Output = Mat4x4;

  fn mul(self, _: Mat4x4) -> Mat4x4 {
    todo!()
  }
}

// TODO: Once all the impls are filled out, copy all the impls for Double and
// Int types as well.
