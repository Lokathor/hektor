use super::*;

#[test]
fn Mat2x2_mul_Vec2() {
  use h::*;
  let red = mat2x2_rows(1.0, 2.0, 3.0, 4.0);
  let v = vec2(2.0, 1.0);
  assert_eq!(red * v, vec2(4.0, 10.0));

  let v = vec2(0.0, 2.0);
  assert_eq!(red * v, vec2(4.0, 8.0));
}

#[test]
fn Mat2x2_mul_Mat2x2() {
  use h::*;
  let red = mat2x2_rows(1.0, 2.0, 3.0, 4.0);
  let yellow = mat2x2_rows(2.0, 0.0, 1.0, 2.0);
  assert_eq!(red * yellow, mat2x2_rows(4.0, 4.0, 10.0, 8.0));
  assert_eq!(yellow * red, mat2x2_rows(2.0, 4.0, 7.0, 10.0));
}

#[test]
fn Mat2x2_mul_Mat2x3() {
  use h::*;
  let red = mat2x2_rows(1.0, 2.0, 3.0, 4.0);
  let yellow = mat2x3_rows(2.0, 0.0, 1.0, 2.0, -1.0, -3.0);
  assert_eq!(red * yellow, mat2x3_rows(6.0, -2.0, -5.0, 14.0, -4.0, -9.0));
}

#[test]
fn Mat2x2_mul_Mat2x4() {
  use h::*;
  let red = mat2x2_rows(1.0, 2.0, 3.0, 4.0);
  let yellow = mat2x4_rows(2.0, 0.0, 1.0, 2.0, -1.0, -3.0, 1.0, 2.0);
  assert_eq!(red * yellow, mat2x4_rows(0.0, -6.0, 3.0, 6.0, 2.0, -12.0, 7.0, 14.0));
}
