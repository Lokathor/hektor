#![allow(bad_style)]
#![allow(unused_imports)]

use nalgebra as na;

use hektor as h;

mod determinant;
mod multiplication;

fn approx_eq_f32(a: f32, b: f32) -> bool {
  (a - b).abs() < 0.00000001
}

fn approx_eq_f64(a: f64, b: f64) -> bool {
  (a - b).abs() < 0.00000000001
}
