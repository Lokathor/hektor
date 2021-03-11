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

  /// normalized.
  #[rustfmt::skip]
  pub fn from_axis_angle(axis: Vec3, radians: f32) -> Self {
    let s = radians.sin();
    let c = radians.cos();
    let omc = 1.0 - c;
    mat4x4_rows(
      [omc * axis.x * axis.x + c,          omc * axis.x * axis.y - axis.z * s, omc * axis.z * axis.x + axis.y * s, 0.0],
      [omc * axis.x * axis.y + axis.z * s, omc * axis.y * axis.y + c,          omc * axis.y * axis.z - axis.x * s, 0.0],
      [omc * axis.z * axis.x - axis.y * s, omc * axis.y * axis.z + axis.x * s, omc * axis.z * axis.z + c,          0.0],
      [0.0,                                0.0,                                0.0,                                1.0],
    )
  }

  #[rustfmt::skip]
  pub fn look_at(eye_pos: Vec3, target: Vec3, up: Vec3) -> Self {
    let front = (target - eye_pos).normalized();
    let right = front.cross(up).normalized();
    let up = right.cross(front);
    mat4x4_rows(
      [right.x, up.x, -front.x, 0.0],
      [right.y, up.y, -front.y, 0.0],
      [right.z, up.z, -front.z, 0.0],
      [-right.dot(eye_pos), -up.dot(eye_pos), front.dot(eye_pos), 1.0],
    )
  }

  #[rustfmt::skip]
  pub fn perspective(v_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
    let t = (v_fov * 0.5).tan();
    let sy = 1.0 / t;
    let sx = sy / aspect_ratio;
    let n_m_f = z_near - z_far;
    mat4x4_rows(
      [sx , 0.0, 0.0, 0.0],
      [0.0, sy , 0.0, 0.0],
      [0.0, 0.0, (z_far+z_near)/n_m_f, -1.0],
      [0.0, 0.0, (2.0*z_near*z_far)/n_m_f, 0.0],
    )
  }

  #[rustfmt::skip]
  pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
    let r_m_l = right - left;
    let r_p_l = right + left;
    let t_m_b = top - bottom;
    let t_p_b = top + bottom;
    let f_m_n = far - near;
    let f_p_n = far + near;
    mat4x4_rows(
      [2.0/r_m_l, 0.0,       0.0,        -(r_p_l/r_m_l)],
      [0.0,       2.0/t_m_b, 0.0,        -(t_p_b/t_m_b)],
      [0.0,       0.0,       -2.0/f_m_n, -(f_p_n/f_m_n)],
      [0.0,       0.0,       0.0,        1.0],
    )
  }
}
