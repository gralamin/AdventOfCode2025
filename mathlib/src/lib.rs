mod modulus;
pub use crate::modulus::modulus;

mod interpolation;
pub use crate::interpolation::{lerp, remap};

mod multiples;
pub use crate::multiples::{gcd, lcm};

mod polygon;
pub use crate::polygon::{picks_theorem_i, shoelace_area, shoepick, shoepick_intlengths};

mod distance;
pub use crate::distance::{euclidean_distance, euclidean_distance_squared, manhattan_distance};

mod lines;
pub use crate::lines::{determinant, line_intersect};
