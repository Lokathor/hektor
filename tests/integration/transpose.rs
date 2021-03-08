use super::*;

#[test]
fn test_Mat2x2_transpose() {
  let a = mat2x2_rows([1.0, 2.0], [3.0, 4.0]);
  let b = mat2x2_rows([1.0, 3.0], [2.0, 4.0]);
  assert_eq!(a.transpose(), b);
  assert_eq!(a.transpose().transpose(), a);
}

#[test]
fn test_Mat2x3_and_Mat3x2_transpose() {
  let a = mat2x3_rows([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
  let b = mat3x2_rows([1.0, 4.0], [2.0, 5.0], [3.0, 6.0]);
  assert_eq!(a.transpose(), b);
  assert_eq!(a.transpose().transpose(), a);
}

#[test]
fn test_Mat2x4_and_Mat4x2_transpose() {
  let a = mat2x4_rows([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]);
  let b = mat4x2_rows([1.0, 5.0], [2.0, 6.0], [3.0, 7.0], [4.0, 8.0]);
  assert_eq!(a.transpose(), b);
  assert_eq!(a.transpose().transpose(), a);
}

#[test]
fn test_Mat3x3_transpose() {
  let a = mat3x3_rows([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]);
  let b = mat3x3_rows([1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0]);
  assert_eq!(a.transpose(), b);
  assert_eq!(a.transpose().transpose(), a);
}

#[test]
fn test_Mat3x4_and_Mat4x3_transpose() {
  let a = mat3x4_rows(
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
  );
  let b = mat4x3_rows(
    [1.0, 5.0, 9.0],
    [2.0, 6.0, 10.0],
    [3.0, 7.0, 11.0],
    [4.0, 8.0, 12.0],
  );
  assert_eq!(a.transpose(), b);
  assert_eq!(a.transpose().transpose(), a);
}

#[test]
fn test_Mat4x4_transpose() {
  let a = mat4x4_rows(
    [1.0, 2.0, 3.0, 4.0],
    [5.0, 6.0, 7.0, 8.0],
    [9.0, 10.0, 11.0, 12.0],
    [13.0, 14.0, 15.0, 16.0],
  );
  let b = mat4x4_rows(
    [1.0, 5.0, 9.0, 13.0],
    [2.0, 6.0, 10.0, 14.0],
    [3.0, 7.0, 11.0, 15.0],
    [4.0, 8.0, 12.0, 16.0],
  );
  assert_eq!(a.transpose(), b);
  assert_eq!(a.transpose().transpose(), a);
}
