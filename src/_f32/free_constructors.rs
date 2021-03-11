use super::*;

pub const fn vec2(x: f32, y: f32) -> Vec2 {
  Vec2 { x, y }
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
  Vec3 { x, y, z }
}

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
  Vec4 { x, y, z, w }
}

pub const fn mat2x2_rows(
  [r0c0, r0c1]: [f32; 2], [r1c0, r1c1]: [f32; 2],
) -> Mat2x2 {
  Mat2x2 { r0c0, r1c0, r0c1, r1c1 }
}

pub const fn mat2x3_rows(
  [r0c0, r0c1, r0c2]: [f32; 3], [r1c0, r1c1, r1c2]: [f32; 3],
) -> Mat2x3 {
  Mat2x3 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2 }
}

pub const fn mat2x4_rows(
  [r0c0, r0c1, r0c2, r0c3]: [f32; 4], [r1c0, r1c1, r1c2, r1c3]: [f32; 4],
) -> Mat2x4 {
  Mat2x4 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2, r0c3, r1c3 }
}

pub const fn mat3x2_rows(
  [r0c0, r0c1]: [f32; 2], [r1c0, r1c1]: [f32; 2], [r2c0, r2c1]: [f32; 2],
) -> Mat3x2 {
  Mat3x2 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1 }
}

pub const fn mat3x3_rows(
  [r0c0, r0c1, r0c2]: [f32; 3], [r1c0, r1c1, r1c2]: [f32; 3],
  [r2c0, r2c1, r2c2]: [f32; 3],
) -> Mat3x3 {
  Mat3x3 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2 }
}

pub const fn mat3x4_rows(
  [r0c0, r0c1, r0c2, r0c3]: [f32; 4], [r1c0, r1c1, r1c2, r1c3]: [f32; 4],
  [r2c0, r2c1, r2c2, r2c3]: [f32; 4],
) -> Mat3x4 {
  Mat3x4 {
    r0c0,
    r1c0,
    r2c0,
    r0c1,
    r1c1,
    r2c1,
    r0c2,
    r1c2,
    r2c2,
    r0c3,
    r1c3,
    r2c3,
  }
}

pub const fn mat4x2_rows(
  [r0c0, r0c1]: [f32; 2], [r1c0, r1c1]: [f32; 2], [r2c0, r2c1]: [f32; 2],
  [r3c0, r3c1]: [f32; 2],
) -> Mat4x2 {
  Mat4x2 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1 }
}

pub const fn mat4x3_rows(
  [r0c0, r0c1, r0c2]: [f32; 3], [r1c0, r1c1, r1c2]: [f32; 3],
  [r2c0, r2c1, r2c2]: [f32; 3], [r3c0, r3c1, r3c2]: [f32; 3],
) -> Mat4x3 {
  Mat4x3 {
    r0c0,
    r1c0,
    r2c0,
    r3c0,
    r0c1,
    r1c1,
    r2c1,
    r3c1,
    r0c2,
    r1c2,
    r2c2,
    r3c2,
  }
}

pub const fn mat4x4_rows(
  [r0c0, r0c1, r0c2, r0c3]: [f32; 4], [r1c0, r1c1, r1c2, r1c3]: [f32; 4],
  [r2c0, r2c1, r2c2, r2c3]: [f32; 4], [r3c0, r3c1, r3c2, r3c3]: [f32; 4],
) -> Mat4x4 {
  Mat4x4 {
    r0c0,
    r1c0,
    r2c0,
    r3c0,
    r0c1,
    r1c1,
    r2c1,
    r3c1,
    r0c2,
    r1c2,
    r2c2,
    r3c2,
    r0c3,
    r1c3,
    r2c3,
    r3c3,
  }
}
