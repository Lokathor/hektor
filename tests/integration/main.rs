#![allow(bad_style)]
#![allow(unused_imports)]

use nalgebra as na;

use hektor::*;

mod layout;
mod transpose;

#[allow(dead_code)]
fn approx_eq_f32(a: f32, b: f32) -> bool {
  (a - b).abs() < 0.00000001
}

#[allow(dead_code)]
fn approx_eq_f64(a: f64, b: f64) -> bool {
  (a - b).abs() < 0.00000000001
}

/// Clamps into the range 0.0 to 1.0.
///
/// This will even clamp -0.0 and NaN.
#[allow(dead_code)]
pub fn clamp01(v: f32) -> f32 {
  if v > 0.0 {
    if v <= 1.0 {
      v
    } else {
      1.0
    }
  } else {
    0.0
  }
}
