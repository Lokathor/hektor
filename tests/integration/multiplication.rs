use super::*;

#[test]
fn Mat2x2_mul_Vec2() {
  use h::*;
  let red = mat2x2_row(1.0, 2.0, 3.0, 4.0);
  let v = vec2(2.0, 1.0);
  assert_eq!(red * v, vec2(4.0, 10.0));

  let v = vec2(0.0, 2.0);
  assert_eq!(red * v, vec2(4.0, 8.0));
}

#[test]
fn Mat2x2_mul_Mat2x2() {
  use h::*;
  let red = mat2x2_row(1.0, 2.0, 3.0, 4.0);
  let yellow = mat2x2_row(2.0, 0.0, 1.0, 2.0);
  assert_eq!(red * yellow, mat2x2_row(4.0, 4.0, 10.0, 8.0));
  assert_eq!(yellow * red, mat2x2_row(2.0, 4.0, 7.0, 10.0));
}
