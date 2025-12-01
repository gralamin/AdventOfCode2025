mod modulus;
pub use crate::modulus::modulus;

mod interpolation;
pub use crate::interpolation::{lerp, remap};

mod multiples;
pub use crate::multiples::{gcd, lcm};