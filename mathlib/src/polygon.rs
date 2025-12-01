extern crate num_traits;

use num_traits::{Num, Signed};

// Came up in Advent of code 2023 day10
/// Provide the area enclosed by a polygon.
/// ```
/// let points = [(1.0, 6.0), (3.0, 1.0), (7.0, 2.0), (4.0, 4.0), (8.0, 5.0)];
/// assert_eq!(mathlib::shoelace_area(&points), 16.5);
/// ```
pub fn shoelace_area<T: Num + Copy + Signed + std::iter::Sum>(points: &[(T, T)]) -> T {
    if points.len() < 3 {
        return T::zero();
    }

    let sum: T = points
        .iter()
        // Zip the list with itself, offset by 1, wrapping around forever
        .zip(points.iter().cycle().skip(1))
        // Stop after we've processed every edge once
        .take(points.len())
        .map(|((x1, y1), (x2, y2))| (*x1 * *y2) - (*x2 * *y1))
        .sum();

    return (sum / (T::one() + T::one())).abs();
}
