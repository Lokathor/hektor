mod clamp01;
pub use clamp01::*;

pub use _f32::*;
mod _f32 {
  mod structures;
  pub use structures::*;

  mod as_methods;
  pub use as_methods::*;

  mod addition;
  pub use addition::*;

  mod subtraction;
  pub use subtraction::*;

  mod negation;
  pub use negation::*;

  mod scaling;
  pub use scaling::*;

  mod determinant;
  pub use determinant::*;

  mod identity;
  pub use identity::*;

  mod dot_product;
  pub use dot_product::*;

  mod free_constructors;
  pub use free_constructors::*;

  mod multiplication;
  pub use multiplication::*;

  mod transpose;
  pub use transpose::*;

  mod length;
  pub use length::*;

  mod reflect;
  pub use reflect::*;

  mod refract;
  pub use refract::*;

  mod mat4_constructors;
  pub use mat4_constructors::*;

  mod cross;
  pub use cross::*;
}

// TODO: quaternions? or rotors?

// TODO: curves? splines?
