#![allow(bad_style)]

use nalgebra as na;

use hektor as h;

fn approx_eq_f32(a: f32, b: f32) -> bool {
  (a - b).abs() < 0.00000001
}

fn approx_eq_f64(a: f64, b: f64) -> bool {
  (a - b).abs() < 0.00000000001
}

#[test]
fn test_Mat2x2_determinant() {
  let na_m = na::Matrix2::<f32>::new(1.0, 2.0, 3.0, 4.0);
  let h_m = h::Mat2x2 { r0c0: 1.0, r0c1: 2.0, r1c0: 3.0, r1c1: 4.0 };
  assert!(approx_eq_f32(na_m.determinant(), h_m.determinant()));
}

#[test]
fn test_DMat2x2_determinant() {
  let na_m = na::Matrix2::<f64>::new(1.0, 2.0, 3.0, 4.0);
  let h_m = h::DMat2x2 { r0c0: 1.0, r0c1: 2.0, r1c0: 3.0, r1c1: 4.0 };
  assert!(approx_eq_f64(na_m.determinant(), h_m.determinant()));
}

/*
#[test]
fn test_IMat2x2_determinant() {
  let na_m = na::Matrix2::<i32>::new(1, 2, 3, 4);
  let h_m = h::IMat2x2 { r0c0: 1, r0c1: 2, r1c0: 3, r1c1: 4 };
  assert!(na_m.determinant(), h_m.determinant());
}
*/

#[test]
fn test_Mat3x3_determinant() {
  let na_m = na::Matrix3::<f32>::new(10.0, 2.0, -5.0, 4.0, 8.0, 1.0, 30.0, -99.0, 9.0);
  let h_m = h::Mat3x3 { r0c0: 10.0, r1c0: 2.0, r2c0: -5.0, r0c1: 4.0, r1c1: 8.0, r2c1: 1.0, r0c2: 30.0, r1c2: -99.0, r2c2: 9.0 };
  assert!(approx_eq_f32(na_m.determinant(), h_m.determinant()));
}

#[test]
fn test_DMat3x3_determinant() {
  let na_m = na::Matrix3::<f64>::new(10.0, 2.0, -5.0, 4.0, 8.0, 1.0, 30.0, -99.0, 9.0);
  let h_m = h::DMat3x3 { r0c0: 10.0, r1c0: 2.0, r2c0: -5.0, r0c1: 4.0, r1c1: 8.0, r2c1: 1.0, r0c2: 30.0, r1c2: -99.0, r2c2: 9.0 };
  assert!(approx_eq_f64(na_m.determinant(), h_m.determinant()));
}

#[test]
fn test_Mat4x4_determinant() {
  let na_m = na::Matrix4::<f32>::new(10.0, 2.0, -5.0, 4.0, 8.0, 1.0, 3.0, -10.0, 9.0, 7.0, 12.0, -9.0, -6.0, 2.0, 5.0, -11.0);
  let h_m = h::Mat4x4 { r0c0: 10.0, r1c0: 2.0, r2c0: -5.0, r3c0: 4.0, r0c1: 8.0, r1c1: 1.0, r2c1: 3.0, r3c1: -10.0, r0c2: 9.0, r1c2: 7.0, r2c2: 12.0, r3c2: -9.0, r0c3: -6.0, r1c3: 2.0, r2c3: 5.0, r3c3: -11.0 };
  assert!(approx_eq_f32(na_m.determinant(), h_m.determinant()));
}

#[test]
fn test_DMat4x4_determinant() {
  let na_m = na::Matrix4::<f64>::new(10.0, 2.0, -5.0, 4.0, 8.0, 1.0, 3.0, -10.0, 9.0, 7.0, 12.0, -9.0, -6.0, 2.0, 5.0, -11.0);
  let h_m = h::DMat4x4 { r0c0: 10.0, r1c0: 2.0, r2c0: -5.0, r3c0: 4.0, r0c1: 8.0, r1c1: 1.0, r2c1: 3.0, r3c1: -10.0, r0c2: 9.0, r1c2: 7.0, r2c2: 12.0, r3c2: -9.0, r0c3: -6.0, r1c3: 2.0, r2c3: 5.0, r3c3: -11.0 };
  assert!(approx_eq_f64(na_m.determinant(), h_m.determinant()), "na: {}, h: {}", na_m.determinant(), h_m.determinant());
}
