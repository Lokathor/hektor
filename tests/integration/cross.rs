use super::*;

#[test]
fn test_Vec3_cross() {
  let a = vec3(5.0, 6.0, 7.0);
  let b = vec3(-2.0, 3.0, 1.0);
  assert_eq!(a.cross(b), vec3(-15.0, -19.0, 27.0));
}
