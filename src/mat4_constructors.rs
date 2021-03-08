use super::*;

impl Mat4x4 {
  pub fn translation(Vec3 { x, y, z }: Vec3) -> Self {
    mat4x4_rows(
      [1.0, 0.0, 0.0, x],
      [0.0, 1.0, 0.0, y],
      [0.0, 0.0, 1.0, z],
      [0.0, 0.0, 0.0, 1.0],
    )
  }

  pub fn scale(Vec3 { x, y, z }: Vec3) -> Self {
    mat4x4_rows(
      [x, 0.0, 0.0, 0.0],
      [0.0, y, 0.0, 0.0],
      [0.0, 0.0, z, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    )
  }

  /// radians!
  pub fn x_rotation(r: f32) -> Self {
    mat4x4_rows(
      [1.0, 0.0, 0.0, 0.0],
      [0.0, r.cos(), r.sin(), 0.0],
      [0.0, -r.sin(), r.cos(), 0.0],
      [0.0, 0.0, 0.0, 1.0],
    )
  }

  /// radians!
  pub fn y_rotation(r: f32) -> Self {
    mat4x4_rows(
      [r.cos(), 0.0, -(r.sin()), 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [r.sin(), 0.0, r.cos(), 0.0],
      [0.0, 0.0, 0.0, 1.0],
    )
  }

  /// radians!
  pub fn z_rotation(r: f32) -> Self {
    mat4x4_rows(
      [r.cos(), -r.sin(), 0.0, 0.0],
      [r.sin(), r.cos(), 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    )
  }

  /// radians!
  #[rustfmt::skip]
  pub fn xyz_rotation(x: f32, y: f32, z: f32) -> Self {
    let cx = x.cos();
    let cy = y.cos();
    let cz = z.cos();
    let sx = x.sin();
    let sy = y.sin();
    let sz = z.sin();
    let cy_cz = cy*cz;
    let sx_sy = sx*sy;
    let cx_sy = cx*sy;
    mat4x4_rows(
      [ cy_cz, cy*sz+sx_sy*cz, sx*sz-cx_sy*cz, 0.0],
      [-cy_cz, cx*cz-sx_sy*sz, sx*cz+cx_sy*sz, 0.0],
      [    sy,         -sx*cy,          cx*cy, 0.0],
      [   0.0,            0.0,            0.0, 1.0],
    )
  }

  // TODO: axis_angle

  // TODO: look_at

  // TODO: perspective

  // TODO: orthographic
}
