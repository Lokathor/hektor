use super::*;

macro_rules! impl_deref_for {
  ($m:ident, [[$element:ty; $row_count:literal]; $col_count:literal]) => {
    impl $m {
      /// Since these are col-major, you get `col` arrays of `row` each.
      pub fn as_array(&self) -> &[[$element; $row_count]; $col_count] {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<[[$element; $row_count]; $col_count]>());
        unsafe { &*(self as *const $m as *const [[$element; $row_count]; $col_count]) }
      }
      /// Since these are col-major, you get `col` arrays of `row` each.
      pub fn as_array_mut(&mut self) -> &mut [[$element; $row_count]; $col_count] {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<[[$element; $row_count]; $col_count]>());
        unsafe { &mut *(self as *mut $m as *mut [[$element; $row_count]; $col_count]) }
      }
      /// All the (col-major) matrix data as a slice
      pub fn as_slice(&self) -> &[$element] {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<$element>() * $col_count * $row_count);
        unsafe {
          core::slice::from_raw_parts(self as *const $m as *const $element, core::mem::size_of::<$element>() * $col_count * $row_count)
        }
      }
      /// All the (col-major) matrix data as a mutable slice
      pub fn as_slice_mut(&mut self) -> &mut [$element] {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<$element>() * $col_count * $row_count);
        unsafe {
          core::slice::from_raw_parts_mut(self as *mut $m as *mut $element, core::mem::size_of::<$element>() * $col_count * $row_count)
        }
      }
    }
  };
  ($v:ident, [$element:ty; $e_count:literal]) => {
    impl $v {
      pub fn as_array(&self) -> &[$element; $e_count] {
        assert_eq!(core::mem::size_of::<$v>(), core::mem::size_of::<[$element; $e_count]>());
        unsafe { &*(self as *const $v as *const [$element; $e_count]) }
      }
      pub fn as_array_mut(&mut self) -> &mut [$element; $e_count] {
        assert_eq!(core::mem::size_of::<$v>(), core::mem::size_of::<[$element; $e_count]>());
        unsafe { &mut *(self as *mut $v as *mut [$element; $e_count]) }
      }
      pub fn as_slice(&self) -> &[$element] {
        assert_eq!(core::mem::size_of::<$v>(), core::mem::size_of::<$element>() * $e_count);
        unsafe {
          core::slice::from_raw_parts(self as *const $v as *const $element, core::mem::size_of::<$element>() * $e_count)
        }
      }
      pub fn as_slice_mut(&mut self) -> &mut [$element] {
        assert_eq!(core::mem::size_of::<$v>(), core::mem::size_of::<$element>() * $e_count);
        unsafe {
          core::slice::from_raw_parts_mut(self as *mut $v as *mut $element, core::mem::size_of::<$element>() * $e_count)
        }
      }
    }
  }
}

impl_deref_for!(Mat2x2, [[f32; 2]; 2]);
impl_deref_for!(Mat2x3, [[f32; 2]; 3]);
impl_deref_for!(Mat2x4, [[f32; 2]; 4]);
impl_deref_for!(Mat3x2, [[f32; 3]; 2]);
impl_deref_for!(Mat3x3, [[f32; 3]; 3]);
impl_deref_for!(Mat3x4, [[f32; 3]; 4]);
impl_deref_for!(Mat4x2, [[f32; 4]; 2]);
impl_deref_for!(Mat4x3, [[f32; 4]; 3]);
impl_deref_for!(Mat4x4, [[f32; 4]; 4]);
impl_deref_for!(Vec2, [f32; 2]);
impl_deref_for!(Vec3, [f32; 3]);
impl_deref_for!(Vec4, [f32; 4]);

impl_deref_for!(DMat2x2, [[f64; 2]; 2]);
impl_deref_for!(DMat2x3, [[f64; 2]; 3]);
impl_deref_for!(DMat2x4, [[f64; 2]; 4]);
impl_deref_for!(DMat3x2, [[f64; 3]; 2]);
impl_deref_for!(DMat3x3, [[f64; 3]; 3]);
impl_deref_for!(DMat3x4, [[f64; 3]; 4]);
impl_deref_for!(DMat4x2, [[f64; 4]; 2]);
impl_deref_for!(DMat4x3, [[f64; 4]; 3]);
impl_deref_for!(DMat4x4, [[f64; 4]; 4]);
impl_deref_for!(DVec2, [f64; 2]);
impl_deref_for!(DVec3, [f64; 3]);
impl_deref_for!(DVec4, [f64; 4]);

impl_deref_for!(IMat2x2, [[i32; 2]; 2]);
impl_deref_for!(IMat2x3, [[i32; 2]; 3]);
impl_deref_for!(IMat2x4, [[i32; 2]; 4]);
impl_deref_for!(IMat3x2, [[i32; 3]; 2]);
impl_deref_for!(IMat3x3, [[i32; 3]; 3]);
impl_deref_for!(IMat3x4, [[i32; 3]; 4]);
impl_deref_for!(IMat4x2, [[i32; 4]; 2]);
impl_deref_for!(IMat4x3, [[i32; 4]; 3]);
impl_deref_for!(IMat4x4, [[i32; 4]; 4]);
impl_deref_for!(IVec2, [i32; 2]);
impl_deref_for!(IVec3, [i32; 3]);
impl_deref_for!(IVec4, [i32; 4]);
