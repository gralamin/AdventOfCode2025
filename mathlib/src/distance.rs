extern crate num_traits;

use num_traits::{Signed, ToPrimitive};

/// Provide manhattan distance for most number types
/// ```
/// assert_eq!(mathlib::manhattan_distance(3, 4, 7, 1), 7);
/// ```
pub fn manhattan_distance<T: Signed>(x1: T, y1: T, x2: T, y2: T) -> T {
    return (x1 - x2).abs() + (y1 - y2).abs();
}

/// Provide euclidean distance squared for most number types
/// ```
/// assert_eq!(mathlib::euclidean_distance_squared(3.0, 4.0, 7.0, 1.0), 25.0);
/// ```
pub fn euclidean_distance_squared<T: Signed + Copy>(x1: T, y1: T, x2: T, y2: T) -> T {
    let a = x2 - x1;
    let b = y2 - y1;
    return a * a + b * b;
}

/// Provide true euclidean distance, by applying a squareroot for each size, stored in an f64.
/// ```
/// assert_eq!(mathlib::euclidean_distance(3.0, 4.0, 7.0, 1.0), 5.0);
/// ```
pub fn euclidean_distance<T: Signed + Copy + ToPrimitive>(x1: T, y1: T, x2: T, y2: T) -> f64 {
    let squared = euclidean_distance_squared(x1, y1, x2, y2);
    return f64::sqrt(squared.to_f64().unwrap());
}
