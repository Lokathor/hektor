use super::*;

#[test]
fn test_Mat2x2_layout() {
  let m = mat2x2_rows([1.0, 2.0], [3.0, 4.0]);
  assert_eq!(m.as_slice(), &[1.0, 3.0, 2.0, 4.0]);
}

#[test]
fn test_Mat2x3_layout() {
  let m = mat2x3_rows([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
  assert_eq!(m.as_slice(), &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
}

#[test]
fn test_Mat2x4_layout() {
  let m = mat2x4_rows([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]);
  assert_eq!(m.as_slice(), &[1.0, 5.0, 2.0, 6.0, 3.0, 7.0, 4.0, 8.0]);
}

#[test]
fn test_Mat3x2_layout() {
  let m = mat3x2_rows([1.0, 2.0], [3.0, 4.0], [5.0, 6.0]);
  assert_eq!(m.as_slice(), &[1.0, 3.0, 5.0, 2.0, 4.0, 6.0]);
}

#[test]
fn test_Mat3x3_layout() {
  let m = mat3x3_rows([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]);
  assert_eq!(m.as_slice(), &[1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0]);
}

#[test]
fn test_Mat3x4_layout() {
  let m = mat3x4_rows(
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
  );
  assert_eq!(
    m.as_slice(),
    &[1.0, 5.0, 9.0, 2.0, 6.0, 10.0, 3.0, 7.0, 11.0, 4.0, 8.0, 12.0]
  );
}

#[test]
fn test_Mat4x2_layout() {
  let m = mat4x2_rows([1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]);
  assert_eq!(m.as_slice(), &[1.0, 3.0, 5.0, 7.0, 2.0, 4.0, 6.0, 8.0]);
}

#[test]
fn test_Mat4x3_layout() {
  let m = mat4x3_rows(
    [1.0, 2.0, 3.0],
    [4.0, 5.0, 6.0],
    [7.0, 8.0, 9.0],
    [10.0, 11.0, 12.0],
  );
  assert_eq!(
    m.as_slice(),
    &[1.0, 4.0, 7.0, 10.0, 2.0, 5.0, 8.0, 11.0, 3.0, 6.0, 9.0, 12.0]
  );
}

#[test]
fn test_Mat4x4_layout() {
  let m = mat4x4_rows(
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
  );
  assert_eq!(
    m.as_slice(),
    &[
      1.0, 5.0, 9.0, 13.0, 2.0, 6.0, 10.0, 14.0, 3.0, 7.0, 11.0, 15.0, 4.0,
      8.0, 12.0, 16.0
    ]
  );
}
