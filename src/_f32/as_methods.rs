use super::*;

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
