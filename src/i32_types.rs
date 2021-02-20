use super::*;

pub type IMat2 = IMat2x2;

pub type IMat3 = IMat3x3;

pub type IMat4 = IMat4x4;

//

/// 2 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat2x2 {
  pub r0c0: i32, pub r1c0: i32,
  pub r0c1: i32, pub r1c1: i32,
}

/// 2 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat2x3 {
  pub r0c0: i32, pub r1c0: i32,
  pub r0c1: i32, pub r1c1: i32,
  pub r0c2: i32, pub r1c2: i32,
}

/// 2 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat2x4 {
  pub r0c0: i32, pub r1c0: i32,
  pub r0c1: i32, pub r1c1: i32,
  pub r0c2: i32, pub r1c2: i32,
  pub r0c3: i32, pub r1c3: i32,
}

/// 3 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat3x2 {
  pub r0c0: i32, pub r1c0: i32, pub r2c0: i32,
  pub r0c1: i32, pub r1c1: i32, pub r2c1: i32,
}

/// 3 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat3x3 {
  pub r0c0: i32, pub r1c0: i32, pub r2c0: i32,
  pub r0c1: i32, pub r1c1: i32, pub r2c1: i32,
  pub r0c2: i32, pub r1c2: i32, pub r2c2: i32,
}

/// 3 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat3x4 {
  pub r0c0: i32, pub r1c0: i32, pub r2c0: i32,
  pub r0c1: i32, pub r1c1: i32, pub r2c1: i32,
  pub r0c2: i32, pub r1c2: i32, pub r2c2: i32,
  pub r0c3: i32, pub r1c3: i32, pub r2c3: i32,
}

/// 4 rows, 2 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat4x2 {
  pub r0c0: i32, pub r1c0: i32, pub r2c0: i32, pub r3c0: i32,
  pub r0c1: i32, pub r1c1: i32, pub r2c1: i32, pub r3c1: i32,
}

/// 4 rows, 3 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat4x3 {
  pub r0c0: i32, pub r1c0: i32, pub r2c0: i32, pub r3c0: i32,
  pub r0c1: i32, pub r1c1: i32, pub r2c1: i32, pub r3c1: i32,
  pub r0c2: i32, pub r1c2: i32, pub r2c2: i32, pub r3c2: i32,
}

/// 4 rows, 4 cols
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IMat4x4 {
  pub r0c0: i32, pub r1c0: i32, pub r2c0: i32, pub r3c0: i32,
  pub r0c1: i32, pub r1c1: i32, pub r2c1: i32, pub r3c1: i32,
  pub r0c2: i32, pub r1c2: i32, pub r2c2: i32, pub r3c2: i32,
  pub r0c3: i32, pub r1c3: i32, pub r2c3: i32, pub r3c3: i32,
}

//

/// 2 elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IVec2 {
  pub x: i32,
  pub y: i32,
}

/// 3 elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IVec3 {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

/// 4 elements
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(C)]
#[rustfmt::skip]
pub struct IVec4 {
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub w: i32,
}
