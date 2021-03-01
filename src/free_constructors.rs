use super::*;

pub const fn vec2(x: f32, y: f32) -> Vec2 {
  Vec2 { x, y }
}

pub const fn mat2x2_row(r0c0: f32, r0c1: f32, r1c0: f32, r1c1: f32) -> Mat2x2 {
  Mat2x2 { r0c0, r0c1, r1c0, r1c1 }
}

pub const fn mat2x3_row(r0c0: f32, r0c1: f32, r0c2: f32, r1c0: f32, r1c1: f32, r1c2: f32) -> Mat2x3 {
  Mat2x3 { r0c0, r0c1, r0c2, r1c0, r1c1, r1c2 }
}

pub const fn mat2x4_row(r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32, r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32) -> Mat2x4 {
  Mat2x4 { r0c0, r0c1, r0c2, r1c0, r1c1, r1c2, r0c3, r1c3 }
}
