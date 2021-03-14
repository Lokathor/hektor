// Note(Lokathor): The macro_rules declarations have very stupid pathing rules
// where they live in their own namespace. However, after each macro is defined
// we can write `pub(crate) use macro_name;` and drag it from the special
// macro_rules namespace into the "normal" namespace. This `use` statement must
// be exactly `pub(crate)` (not `pub`) and it must be exactly `use macro_name;`
// (not `use self::macro_name;`). Also, each `use` must appear textually after
// the macro has been defined.

// Note(Yandros): To illustrate:
// ```
// pub(crate) use example;
// ```
// is actually
// ```txt
// pub(crate) use /* prelude_here:: */ example as /* self:: */ example;
// ```

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
          core::slice::from_raw_parts(self as *const $m as *const $element, $col_count * $row_count)
        }
      }
      /// All the (col-major) matrix data as a mutable slice
      pub fn as_slice_mut(&mut self) -> &mut [$element] {
        assert_eq!(core::mem::size_of::<$m>(), core::mem::size_of::<$element>() * $col_count * $row_count);
        unsafe {
          core::slice::from_raw_parts_mut(self as *mut $m as *mut $element, $col_count * $row_count)
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
pub(crate) use impl_deref_for;

macro_rules! impl_add_for {
  ($m:ident { $($f:ident),+ }) => {
    impl core::ops::Add for $m {
      type Output = Self;
      fn add(self, rhs: Self) -> Self {
        $m {
          $( $f: self.$f + rhs.$f ),+
        }
      }
    }
    impl core::ops::Add<&$m> for $m {
      type Output = Self;
      fn add(self, rhs: &Self) -> Self {
        self + *rhs
      }
    }
    //
    impl core::ops::AddAssign for $m {
      fn add_assign(&mut self, rhs: Self) {
        $( self.$f += rhs.$f; )+
      }
    }
    impl core::ops::AddAssign<&$m> for $m {
      fn add_assign(&mut self, rhs: &Self) {
        $( self.$f += rhs.$f; )+
      }
    }
  }
}
pub(crate) use impl_add_for;

macro_rules! impl_sub_for {
  ($m:ident { $($f:ident),+ }) => {
    impl core::ops::Sub for $m {
      type Output = Self;
      fn sub(self, rhs: Self) -> Self {
        $m {
          $( $f: self.$f - rhs.$f ),+
        }
      }
    }
    impl core::ops::Sub<&$m> for $m {
      type Output = Self;
      fn sub(self, rhs: &Self) -> Self {
        self - *rhs
      }
    }
    //
    impl core::ops::SubAssign for $m {
      fn sub_assign(&mut self, rhs: Self) {
        $( self.$f -= rhs.$f; )+
      }
    }
    impl core::ops::SubAssign<&$m> for $m {
      fn sub_assign(&mut self, rhs: &Self) {
        $( self.$f -= rhs.$f; )+
      }
    }
  }
}
pub(crate) use impl_sub_for;

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
pub(crate) use impl_neg_for;

macro_rules! impl_scale_for {
  ($m:ident<$elem_t:ty> { $($field:ident),+ }) => {
    // MatMxN * Element
    impl core::ops::Mul<$elem_t> for $m {
      type Output = Self;
      fn mul(self, rhs: $elem_t) -> Self {
        $m {
          $( $field: self.$field * rhs ),+
        }
      }
    }
    // Element * MatMxN
    impl core::ops::Mul<$m> for $elem_t {
      type Output = $m;
      fn mul(self, rhs: $m) -> $m {
        $m {
          $( $field: self * rhs.$field ),+
        }
      }
    }
  }
}
pub(crate) use impl_scale_for;
