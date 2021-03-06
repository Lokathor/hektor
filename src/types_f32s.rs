use super::*;

pub type Mat2 = Mat2x2;

pub type Mat3 = Mat3x3;

pub type Mat4 = Mat4x4;

//

/// 2 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat2x2 {
  pub r0c0: f32, pub r1c0: f32,
  pub r0c1: f32, pub r1c1: f32,
}

/// 2 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat2x3 {
  pub r0c0: f32, pub r1c0: f32,
  pub r0c1: f32, pub r1c1: f32,
  pub r0c2: f32, pub r1c2: f32,
}

/// 2 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat2x4 {
  pub r0c0: f32, pub r1c0: f32,
  pub r0c1: f32, pub r1c1: f32,
  pub r0c2: f32, pub r1c2: f32,
  pub r0c3: f32, pub r1c3: f32,
}

/// 3 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat3x2 {
  pub r0c0: f32, pub r1c0: f32, pub r2c0: f32,
  pub r0c1: f32, pub r1c1: f32, pub r2c1: f32,
}

/// 3 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat3x3 {
  pub r0c0: f32, pub r1c0: f32, pub r2c0: f32,
  pub r0c1: f32, pub r1c1: f32, pub r2c1: f32,
  pub r0c2: f32, pub r1c2: f32, pub r2c2: f32,
}

/// 3 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat3x4 {
  pub r0c0: f32, pub r1c0: f32, pub r2c0: f32,
  pub r0c1: f32, pub r1c1: f32, pub r2c1: f32,
  pub r0c2: f32, pub r1c2: f32, pub r2c2: f32,
  pub r0c3: f32, pub r1c3: f32, pub r2c3: f32,
}

/// 4 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat4x2 {
  pub r0c0: f32, pub r1c0: f32, pub r2c0: f32, pub r3c0: f32,
  pub r0c1: f32, pub r1c1: f32, pub r2c1: f32, pub r3c1: f32,
}

/// 4 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat4x3 {
  pub r0c0: f32, pub r1c0: f32, pub r2c0: f32, pub r3c0: f32,
  pub r0c1: f32, pub r1c1: f32, pub r2c1: f32, pub r3c1: f32,
  pub r0c2: f32, pub r1c2: f32, pub r2c2: f32, pub r3c2: f32,
}

/// 4 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Mat4x4 {
  pub r0c0: f32, pub r1c0: f32, pub r2c0: f32, pub r3c0: f32,
  pub r0c1: f32, pub r1c1: f32, pub r2c1: f32, pub r3c1: f32,
  pub r0c2: f32, pub r1c2: f32, pub r2c2: f32, pub r3c2: f32,
  pub r0c3: f32, pub r1c3: f32, pub r2c3: f32, pub r3c3: f32,
}

//

/// 2D
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

/// 3D
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

/// 4D
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct Vec4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}
