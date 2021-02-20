use super::*;

pub type DMat2 = DMat2x2;

pub type DMat3 = DMat3x3;

pub type DMat4 = DMat4x4;

//

/// 2 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat2x2 {
  pub r0c0: f64, pub r1c0: f64,
  pub r0c1: f64, pub r1c1: f64,
}

/// 2 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat2x3 {
  pub r0c0: f64, pub r1c0: f64,
  pub r0c1: f64, pub r1c1: f64,
  pub r0c2: f64, pub r1c2: f64,
}

/// 2 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat2x4 {
  pub r0c0: f64, pub r1c0: f64,
  pub r0c1: f64, pub r1c1: f64,
  pub r0c2: f64, pub r1c2: f64,
  pub r0c3: f64, pub r1c3: f64,
}

/// 3 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat3x2 {
  pub r0c0: f64, pub r1c0: f64, pub r2c0: f64,
  pub r0c1: f64, pub r1c1: f64, pub r2c1: f64,
}

/// 3 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat3x3 {
  pub r0c0: f64, pub r1c0: f64, pub r2c0: f64,
  pub r0c1: f64, pub r1c1: f64, pub r2c1: f64,
  pub r0c2: f64, pub r1c2: f64, pub r2c2: f64,
}

/// 3 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat3x4 {
  pub r0c0: f64, pub r1c0: f64, pub r2c0: f64,
  pub r0c1: f64, pub r1c1: f64, pub r2c1: f64,
  pub r0c2: f64, pub r1c2: f64, pub r2c2: f64,
  pub r0c3: f64, pub r1c3: f64, pub r2c3: f64,
}

/// 4 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat4x2 {
  pub r0c0: f64, pub r1c0: f64, pub r2c0: f64, pub r3c0: f64,
  pub r0c1: f64, pub r1c1: f64, pub r2c1: f64, pub r3c1: f64,
}

/// 4 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat4x3 {
  pub r0c0: f64, pub r1c0: f64, pub r2c0: f64, pub r3c0: f64,
  pub r0c1: f64, pub r1c1: f64, pub r2c1: f64, pub r3c1: f64,
  pub r0c2: f64, pub r1c2: f64, pub r2c2: f64, pub r3c2: f64,
}

/// 4 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DMat4x4 {
  pub r0c0: f64, pub r1c0: f64, pub r2c0: f64, pub r3c0: f64,
  pub r0c1: f64, pub r1c1: f64, pub r2c1: f64, pub r3c1: f64,
  pub r0c2: f64, pub r1c2: f64, pub r2c2: f64, pub r3c2: f64,
  pub r0c3: f64, pub r1c3: f64, pub r2c3: f64, pub r3c3: f64,
}

//

/// 2 elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DVec2 {
  pub x: f64,
  pub y: f64,
}

/// 3 elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DVec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

/// 4 elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct DVec4 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}
