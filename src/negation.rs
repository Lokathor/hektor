use super::*;

macro_rules! impl_neg_for {
  ($m:ident { $($f:ident),+ }) => {
    impl core::ops::Neg for $m {
      type Output = Self;
      fn neg(self) -> Self {
        $m {
          $( $f: -self.$f ),+
        }
      }
    }
  }
}

impl_neg_for!(Mat2x2 { r0c0, r1c0, r0c1, r1c1 });
impl_neg_for!(Mat2x3 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2 });
impl_neg_for!(Mat2x4 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2, r0c3, r1c3 });
impl_neg_for!(Mat3x2 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1 });
impl_neg_for!(Mat3x3 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2 });
impl_neg_for!(Mat3x4 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2, r0c3, r1c3, r2c3 });
impl_neg_for!(Mat4x2 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1 });
impl_neg_for!(Mat4x3 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2 });
impl_neg_for!(Mat4x4 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2, r0c3, r1c3, r2c3, r3c3 });
impl_neg_for!(Vec2 { x, y });
impl_neg_for!(Vec3 { x, y, z });
impl_neg_for!(Vec4 { x, y, z, w });

impl_neg_for!(DMat2x2 { r0c0, r1c0, r0c1, r1c1 });
impl_neg_for!(DMat2x3 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2 });
impl_neg_for!(DMat2x4 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2, r0c3, r1c3 });
impl_neg_for!(DMat3x2 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1 });
impl_neg_for!(DMat3x3 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2 });
impl_neg_for!(DMat3x4 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2, r0c3, r1c3, r2c3 });
impl_neg_for!(DMat4x2 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1 });
impl_neg_for!(DMat4x3 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2 });
impl_neg_for!(DMat4x4 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2, r0c3, r1c3, r2c3, r3c3 });
impl_neg_for!(DVec2 { x, y });
impl_neg_for!(DVec3 { x, y, z });
impl_neg_for!(DVec4 { x, y, z, w });

impl_neg_for!(IMat2x2 { r0c0, r1c0, r0c1, r1c1 });
impl_neg_for!(IMat2x3 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2 });
impl_neg_for!(IMat2x4 { r0c0, r1c0, r0c1, r1c1, r0c2, r1c2, r0c3, r1c3 });
impl_neg_for!(IMat3x2 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1 });
impl_neg_for!(IMat3x3 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2 });
impl_neg_for!(IMat3x4 { r0c0, r1c0, r2c0, r0c1, r1c1, r2c1, r0c2, r1c2, r2c2, r0c3, r1c3, r2c3 });
impl_neg_for!(IMat4x2 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1 });
impl_neg_for!(IMat4x3 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2 });
impl_neg_for!(IMat4x4 { r0c0, r1c0, r2c0, r3c0, r0c1, r1c1, r2c1, r3c1, r0c2, r1c2, r2c2, r3c2, r0c3, r1c3, r2c3, r3c3 });
impl_neg_for!(IVec2 { x, y });
impl_neg_for!(IVec3 { x, y, z });
impl_neg_for!(IVec4 { x, y, z, w });
