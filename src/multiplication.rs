use super::*;

use core::ops::Mul;

impl Mul<Vec2> for Mat2x2 {
  type Output = Vec2;

  fn mul(self, v: Vec2) -> Vec2 {
    let m = self;
    Vec2 { x: vec2(m.r0c0, m.r0c1).dot(v), y: vec2(m.r1c0, m.r1c1).dot(v) }
  }
}

impl Mul<Mat2x2> for Mat2x2 {
  type Output = Mat2x2;

  fn mul(self, k: Mat2x2) -> Mat2x2 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    Mat2x2 { r0c0: lr0.dot(rc0), r0c1: lr0.dot(rc1), r1c0: lr1.dot(rc0), r1c1: lr1.dot(rc1) }
  }
}

impl Mul<Mat2x3> for Mat2x2 {
  type Output = Mat2x3;

  fn mul(self, k: Mat2x3) -> Mat2x3 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    let rc2 = vec2(k.r0c2, k.r1c2);
    Mat2x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
    }
  }
}

impl Mul<Mat2x4> for Mat2x2 {
  type Output = Mat2x4;

  fn mul(self, k: Mat2x4) -> Mat2x4 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    let rc2 = vec2(k.r0c2, k.r1c2);
    let rc3 = vec2(k.r0c3, k.r1c3);
    Mat2x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr1.dot(rc3),
    }
  }
}

impl Mul<Vec3> for Mat2x3 {
  type Output = Vec2;

  fn mul(self, v: Vec3) -> Vec2 {
    let m = self;
    Vec2 { x: vec3(m.r0c0, m.r0c1, m.r0c2).dot(v), y: vec3(m.r1c0, m.r1c1, m.r1c2).dot(v) }
  }
}

impl Mul<Mat3x2> for Mat2x3 {
  type Output = Mat2x2;

  fn mul(self, k: Mat3x2) -> Mat2x2 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    Mat2x2 { r0c0: lr0.dot(rc0), r0c1: lr0.dot(rc1), r1c0: lr1.dot(rc0), r1c1: lr1.dot(rc1) }
  }
}

impl Mul<Mat3x3> for Mat2x3 {
  type Output = Mat2x3;

  fn mul(self, k: Mat3x3) -> Mat2x3 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    let rc2 = vec3(k.r0c2, k.r1c2, k.r2c2);
    Mat2x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
    }
  }
}

impl Mul<Mat3x4> for Mat2x3 {
  type Output = Mat2x4;

  fn mul(self, k: Mat3x4) -> Mat2x4 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    let rc2 = vec3(k.r0c2, k.r1c2, k.r2c2);
    let rc3 = vec3(k.r0c3, k.r1c3, k.r2c3);
    Mat2x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr0.dot(rc3),
    }
  }
}

impl Mul<Vec4> for Mat2x4 {
  type Output = Vec2;

  fn mul(self, v: Vec4) -> Vec2 {
    let m = self;
    Vec2 {
      x: vec4(m.r0c0, m.r0c1, m.r0c2, m.r0c3).dot(v),
      y: vec4(m.r1c0, m.r1c1, m.r1c2, m.r1c3).dot(v),
    }
  }
}

impl Mul<Mat4x2> for Mat2x4 {
  type Output = Mat2x2;

  fn mul(self, k: Mat4x2) -> Mat2x2 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    Mat2x2 { r0c0: lr0.dot(rc0), r0c1: lr0.dot(rc1), r1c0: lr1.dot(rc0), r1c1: lr1.dot(rc1) }
  }
}

impl Mul<Mat4x3> for Mat2x4 {
  type Output = Mat2x3;

  fn mul(self, k: Mat4x3) -> Mat2x3 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    let rc2 = vec4(k.r0c2, k.r1c2, k.r2c2, k.r3c2);
    Mat2x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
    }
  }
}

impl Mul<Mat4x4> for Mat2x4 {
  type Output = Mat2x4;

  fn mul(self, k: Mat4x4) -> Mat2x4 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    let rc2 = vec4(k.r0c2, k.r1c2, k.r2c2, k.r3c2);
    let rc3 = vec4(k.r0c3, k.r1c3, k.r2c3, k.r3c3);
    Mat2x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr0.dot(rc3),
    }
  }
}

impl Mul<Vec2> for Mat3x2 {
  type Output = Vec3;

  fn mul(self, v: Vec2) -> Vec3 {
    let m = self;
    Vec3 {
      x: vec2(m.r0c0, m.r0c1).dot(v),
      y: vec2(m.r1c0, m.r1c1).dot(v),
      z: vec2(m.r2c0, m.r2c1).dot(v),
    }
  }
}

impl Mul<Mat2x2> for Mat3x2 {
  type Output = Mat3x2;

  fn mul(self, k: Mat2x2) -> Mat3x2 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let lr2 = vec2(self.r2c0, self.r2c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    Mat3x2 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
    }
  }
}

impl Mul<Mat2x3> for Mat3x2 {
  type Output = Mat3x3;

  fn mul(self, k: Mat2x3) -> Mat3x3 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let lr2 = vec2(self.r2c0, self.r2c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    let rc2 = vec2(k.r0c2, k.r1c2);
    Mat3x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
    }
  }
}

impl Mul<Mat2x4> for Mat3x2 {
  type Output = Mat3x4;

  fn mul(self, k: Mat2x4) -> Mat3x4 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let lr2 = vec2(self.r2c0, self.r2c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    let rc2 = vec2(k.r0c2, k.r1c2);
    let rc3 = vec2(k.r0c2, k.r1c2);
    Mat3x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr1.dot(rc3),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r2c3: lr2.dot(rc3),
    }
  }
}

impl Mul<Vec3> for Mat3x3 {
  type Output = Vec3;

  fn mul(self, v: Vec3) -> Vec3 {
    let m = self;
    Vec3 {
      x: vec3(m.r0c0, m.r0c1, m.r0c2).dot(v),
      y: vec3(m.r1c0, m.r1c1, m.r1c2).dot(v),
      z: vec3(m.r2c0, m.r2c1, m.r2c2).dot(v),
    }
  }
}

impl Mul<Mat3x2> for Mat3x3 {
  type Output = Mat3x2;

  fn mul(self, k: Mat3x2) -> Mat3x2 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let lr2 = vec3(self.r2c0, self.r2c1, self.r2c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    Mat3x2 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
    }
  }
}

impl Mul<Mat3x3> for Mat3x3 {
  type Output = Mat3x3;

  fn mul(self, k: Mat3x3) -> Mat3x3 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let lr2 = vec3(self.r2c0, self.r2c1, self.r2c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    let rc2 = vec3(k.r0c2, k.r1c2, k.r2c2);
    Mat3x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
    }
  }
}

impl Mul<Mat3x4> for Mat3x3 {
  type Output = Mat3x4;

  fn mul(self, k: Mat3x4) -> Mat3x4 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let lr2 = vec3(self.r2c0, self.r2c1, self.r2c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    let rc2 = vec3(k.r0c2, k.r1c2, k.r2c2);
    let rc3 = vec3(k.r0c2, k.r1c2, k.r2c2);
    Mat3x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr1.dot(rc3),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r2c3: lr2.dot(rc3),
    }
  }
}

impl Mul<Vec4> for Mat3x4 {
  type Output = Vec3;

  fn mul(self, v: Vec4) -> Vec3 {
    let m = self;
    Vec3 {
      x: vec4(m.r0c0, m.r0c1, m.r0c2, m.r0c3).dot(v),
      y: vec4(m.r1c0, m.r1c1, m.r1c2, m.r1c3).dot(v),
      z: vec4(m.r2c0, m.r2c1, m.r2c2, m.r2c3).dot(v),
    }
  }
}

impl Mul<Mat4x2> for Mat3x4 {
  type Output = Mat3x2;

  fn mul(self, k: Mat4x2) -> Mat3x2 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let lr2 = vec4(self.r2c0, self.r2c1, self.r2c2, self.r2c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    Mat3x2 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
    }
  }
}

impl Mul<Mat4x3> for Mat3x4 {
  type Output = Mat3x3;

  fn mul(self, k: Mat4x3) -> Mat3x3 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let lr2 = vec4(self.r2c0, self.r2c1, self.r2c2, self.r2c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    let rc2 = vec4(k.r0c2, k.r1c2, k.r2c2, k.r3c2);
    Mat3x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
    }
  }
}

impl Mul<Mat4x4> for Mat3x4 {
  type Output = Mat3x4;

  fn mul(self, k: Mat4x4) -> Mat3x4 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let lr2 = vec4(self.r2c0, self.r2c1, self.r2c2, self.r2c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    let rc2 = vec4(k.r0c2, k.r1c2, k.r2c2, k.r3c2);
    let rc3 = vec4(k.r0c2, k.r1c2, k.r2c2, k.r3c2);
    Mat3x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr1.dot(rc3),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r2c3: lr2.dot(rc3),
    }
  }
}

impl Mul<Vec2> for Mat4x2 {
  type Output = Vec4;

  fn mul(self, v: Vec2) -> Vec4 {
    let m = self;
    Vec4 {
      x: vec2(m.r0c0, m.r0c1).dot(v),
      y: vec2(m.r1c0, m.r1c1).dot(v),
      z: vec2(m.r2c0, m.r2c1).dot(v),
      w: vec2(m.r3c0, m.r3c1).dot(v),
    }
  }
}

impl Mul<Mat2x2> for Mat4x2 {
  type Output = Mat4x2;

  fn mul(self, k: Mat2x2) -> Mat4x2 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let lr2 = vec2(self.r2c0, self.r2c1);
    let lr3 = vec2(self.r3c0, self.r3c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    Mat4x2 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
    }
  }
}

impl Mul<Mat2x3> for Mat4x2 {
  type Output = Mat4x3;

  fn mul(self, k: Mat2x3) -> Mat4x3 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let lr2 = vec2(self.r2c0, self.r2c1);
    let lr3 = vec2(self.r3c0, self.r3c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    let rc2 = vec2(k.r0c2, k.r1c2);
    Mat4x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
      r3c2: lr3.dot(rc2),
    }
  }
}

impl Mul<Mat2x4> for Mat4x2 {
  type Output = Mat4x4;

  fn mul(self, k: Mat2x4) -> Mat4x4 {
    let lr0 = vec2(self.r0c0, self.r0c1);
    let lr1 = vec2(self.r1c0, self.r1c1);
    let lr2 = vec2(self.r2c0, self.r2c1);
    let lr3 = vec2(self.r3c0, self.r3c1);
    let rc0 = vec2(k.r0c0, k.r1c0);
    let rc1 = vec2(k.r0c1, k.r1c1);
    let rc2 = vec2(k.r0c2, k.r1c2);
    let rc3 = vec2(k.r0c3, k.r1c3);
    Mat4x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr1.dot(rc3),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r2c3: lr2.dot(rc3),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
      r3c2: lr3.dot(rc2),
      r3c3: lr3.dot(rc3),
    }
  }
}

impl Mul<Vec3> for Mat4x3 {
  type Output = Vec4;

  fn mul(self, v: Vec3) -> Vec4 {
    let m = self;
    Vec4 {
      x: vec3(m.r0c0, m.r0c1, m.r0c2).dot(v),
      y: vec3(m.r1c0, m.r1c1, m.r1c2).dot(v),
      z: vec3(m.r2c0, m.r2c1, m.r2c2).dot(v),
      w: vec3(m.r3c0, m.r3c1, m.r3c2).dot(v),
    }
  }
}

impl Mul<Mat3x2> for Mat4x3 {
  type Output = Mat4x2;

  fn mul(self, k: Mat3x2) -> Mat4x2 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let lr2 = vec3(self.r2c0, self.r2c1, self.r2c2);
    let lr3 = vec3(self.r3c0, self.r3c1, self.r3c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    Mat4x2 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
    }
  }
}

impl Mul<Mat3x3> for Mat4x3 {
  type Output = Mat4x3;

  fn mul(self, k: Mat3x3) -> Mat4x3 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let lr2 = vec3(self.r2c0, self.r2c1, self.r2c2);
    let lr3 = vec3(self.r3c0, self.r3c1, self.r3c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    let rc2 = vec3(k.r0c2, k.r1c2, k.r2c2);
    Mat4x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
      r3c2: lr3.dot(rc2),
    }
  }
}

impl Mul<Mat3x4> for Mat4x3 {
  type Output = Mat4x4;

  fn mul(self, k: Mat3x4) -> Mat4x4 {
    let lr0 = vec3(self.r0c0, self.r0c1, self.r0c2);
    let lr1 = vec3(self.r1c0, self.r1c1, self.r1c2);
    let lr2 = vec3(self.r2c0, self.r2c1, self.r2c2);
    let lr3 = vec3(self.r3c0, self.r3c1, self.r3c2);
    let rc0 = vec3(k.r0c0, k.r1c0, k.r2c0);
    let rc1 = vec3(k.r0c1, k.r1c1, k.r2c1);
    let rc2 = vec3(k.r0c2, k.r1c2, k.r2c2);
    let rc3 = vec3(k.r0c3, k.r1c3, k.r2c3);
    Mat4x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr1.dot(rc3),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r2c3: lr2.dot(rc3),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
      r3c2: lr3.dot(rc2),
      r3c3: lr3.dot(rc3),
    }
  }
}

impl Mul<Vec4> for Mat4x4 {
  type Output = Vec4;

  fn mul(self, v: Vec4) -> Vec4 {
    let m = self;
    Vec4 {
      x: vec4(m.r0c0, m.r0c1, m.r0c2, m.r0c3).dot(v),
      y: vec4(m.r1c0, m.r1c1, m.r1c2, m.r1c3).dot(v),
      z: vec4(m.r2c0, m.r2c1, m.r2c2, m.r2c3).dot(v),
      w: vec4(m.r3c0, m.r3c1, m.r3c2, m.r3c3).dot(v),
    }
  }
}

impl Mul<Mat4x2> for Mat4x4 {
  type Output = Mat4x2;

  fn mul(self, k: Mat4x2) -> Mat4x2 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let lr2 = vec4(self.r2c0, self.r2c1, self.r2c2, self.r2c3);
    let lr3 = vec4(self.r2c0, self.r2c1, self.r2c2, self.r2c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    Mat4x2 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
    }
  }
}

impl Mul<Mat4x3> for Mat4x4 {
  type Output = Mat4x3;

  fn mul(self, k: Mat4x3) -> Mat4x3 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let lr2 = vec4(self.r2c0, self.r2c1, self.r2c2, self.r2c3);
    let lr3 = vec4(self.r3c0, self.r3c1, self.r3c2, self.r3c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    let rc2 = vec4(k.r0c2, k.r1c2, k.r2c2, k.r3c2);
    Mat4x3 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
      r3c2: lr3.dot(rc2),
    }
  }
}

impl Mul<Mat4x4> for Mat4x4 {
  type Output = Mat4x4;

  fn mul(self, k: Mat4x4) -> Mat4x4 {
    let lr0 = vec4(self.r0c0, self.r0c1, self.r0c2, self.r0c3);
    let lr1 = vec4(self.r1c0, self.r1c1, self.r1c2, self.r1c3);
    let lr2 = vec4(self.r2c0, self.r2c1, self.r2c2, self.r2c3);
    let lr3 = vec4(self.r3c0, self.r3c1, self.r3c2, self.r3c3);
    let rc0 = vec4(k.r0c0, k.r1c0, k.r2c0, k.r3c0);
    let rc1 = vec4(k.r0c1, k.r1c1, k.r2c1, k.r3c1);
    let rc2 = vec4(k.r0c2, k.r1c2, k.r2c2, k.r3c2);
    let rc3 = vec4(k.r0c3, k.r1c3, k.r2c3, k.r3c3);
    Mat4x4 {
      r0c0: lr0.dot(rc0),
      r0c1: lr0.dot(rc1),
      r0c2: lr0.dot(rc2),
      r0c3: lr0.dot(rc3),
      r1c0: lr1.dot(rc0),
      r1c1: lr1.dot(rc1),
      r1c2: lr1.dot(rc2),
      r1c3: lr1.dot(rc3),
      r2c0: lr2.dot(rc0),
      r2c1: lr2.dot(rc1),
      r2c2: lr2.dot(rc2),
      r2c3: lr2.dot(rc3),
      r3c0: lr3.dot(rc0),
      r3c1: lr3.dot(rc1),
      r3c2: lr3.dot(rc2),
      r3c3: lr3.dot(rc3),
    }
  }
}
