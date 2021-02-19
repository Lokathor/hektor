use super::*;

macro_rules! impl_deref_for {
  ($m:ident, [[$element:ty; $row_count:literal]; $col_count:literal]) => {
    impl core::ops::Deref for $m {
      type Target = [[$element; $row_count]; $col_count];
      /// Since these are col-major, you deref into `col` arrays of `row` each.
      fn deref(&self) -> &Self::Target {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<Self::Target>());
        unsafe { &*(self as *const $m as *const Self::Target) }
      }
    }
    impl core::ops::DerefMut for $m {
      fn deref_mut(&mut self) -> &mut Self::Target {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<Self::Target>());
        unsafe { &mut *(self as *mut $m as *mut Self::Target) }
      }
    }
    impl $m {
      /// All the (col-major) matrix data as a slice
      pub fn as_slice(&self) -> &[$element] {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<$element>() * $col_count * $row_count);
        unsafe {
          core::slice::from_raw_parts(self.as_ptr().cast(), core::mem::size_of::<$element>() * $col_count * $row_count)
        }
      }
      /// All the (col-major) matrix data as a mutable slice
      pub fn as_slice_mut(&mut self) -> &mut [$element] {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<$element>() * $col_count * $row_count);
        unsafe {
          core::slice::from_raw_parts_mut(self.as_mut_ptr().cast(), core::mem::size_of::<$element>() * $col_count * $row_count)
        }
      }
    }
  };
  ($v:ident, [$element:ty; $e_count:literal]) => {
    impl core::ops::Deref for $v {
      type Target = [$element; $e_count];
      fn deref(&self) -> &Self::Target {
        assert_eq!(core::mem::size_of::<$v>(), core::mem::size_of::<Self::Target>());
        unsafe { &*(self as *const $v as *const Self::Target) }
      }
    }
    impl core::ops::DerefMut for $v {
      fn deref_mut(&mut self) -> &mut Self::Target {
        assert_eq!(core::mem::size_of::<$v>(), core::mem::size_of::<Self::Target>());
        unsafe { &mut *(self as *mut $v as *mut Self::Target) }
      }
    }
    // Note(Lokathor): vectors don't need an as_slice method. they deref into a
    // 1d array, which will further deref into a slice as necessary.
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
