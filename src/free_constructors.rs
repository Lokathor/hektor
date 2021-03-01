use super::*;

pub const fn vec2(x: f32, y: f32) -> Vec2 {
  Vec2 { x, y }
}

pub const fn mat2x2_row(r0c0: f32, r0c1: f32, r1c0: f32, r1c1: f32) -> Mat2x2 {
  Mat2x2 { r0c0, r0c1, r1c0, r1c1 }
}
