use std::ops::{Add, Div, Mul, Sub};

/// Linear Interpolation finds percentage T between two values
/// ```
/// assert_eq!(mathlib::lerp(3.2, 5.8, 0.32), 4.032);
/// ```
pub fn lerp<T>(start: T, end: T, percentage: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy,
{
    start + (end - start) * percentage
}

/// Remap takes a value within one range and linearly maps it to another range
/// ```
/// assert_eq!(mathlib::remap(5.4, 1.0, 20.0, -5.0, 5.0), -2.6842105263157894);
/// ```
pub fn remap<T>(value: T, low1: T, high1: T, low2: T, high2: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    low2 + (value - low1) * (high2 - low2) / (high1 - low1)
}
