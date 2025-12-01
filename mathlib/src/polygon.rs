extern crate num_traits;

use crate::distance::euclidean_distance;
use num_traits::{FromPrimitive, Num, Signed, ToPrimitive};

// Came up in Advent of code 2023 day10
/// Provide the area enclosed by a polygon.
/// ```
/// let points = [(1.0, 6.0), (3.0, 1.0), (7.0, 2.0), (4.0, 4.0), (8.0, 5.0)];
/// let not_enough = [(1.0, 6.0)];
/// assert_eq!(mathlib::shoelace_area(&points), 16.5);
/// assert_eq!(mathlib::shoelace_area(&not_enough), 0.0);
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

/// Get integer points within polygon, you need the area first from Shoelace
///
/// A = interior_points + (integer_boundary / 2) - 1.
/// A + 1 = interior_points + integer_boundary / 2
/// A + 1 - integer_boundary / 2 = interior_points;
/// ```
/// assert_eq!(mathlib::picks_theorem_i(10, 8), 7);
/// ```
pub fn picks_theorem_i<T: Num + Copy + Signed + std::iter::Sum>(
    area: T,
    integer_points_boundary: T,
) -> T {
    return (area + T::one()) - integer_points_boundary / (T::one() + T::one());
}

/// Oneshot shoelace area and picks_theroem_i with no work from the points.
///
/// This avoids errors by combining the formulas to make all division happen late in the process
/// ```
/// let points = [(1.0, 6.0), (3.0, 1.0), (7.0, 2.0), (4.0, 4.0), (8.0, 5.0)];
/// let not_enough = [(1.0, 6.0)];
/// assert_eq!(mathlib::shoepick(&points), 5.346002427150355);
/// assert_eq!(mathlib::shoepick_intlengths(&points), 6.0);
/// assert_eq!(mathlib::shoepick(&not_enough), 0.0);
/// ```
pub fn shoepick<T: Num + Copy + Signed + std::ops::AddAssign + ToPrimitive + FromPrimitive>(
    points: &[(T, T)],
) -> T {
    return inner_shoepick(points, false);
}

pub fn shoepick_intlengths<
    T: Num + Copy + Signed + std::ops::AddAssign + ToPrimitive + FromPrimitive,
>(
    points: &[(T, T)],
) -> T {
    return inner_shoepick(points, true);
}

fn inner_shoepick<T: Num + Copy + Signed + std::ops::AddAssign + ToPrimitive + FromPrimitive>(
    points: &[(T, T)],
    integer_lengths: bool,
) -> T {
    let n = points.len();
    if n < 3 {
        return T::zero();
    }
    let mut shoelace_sum: T = T::zero();
    let mut boundary_len: T = T::zero();
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n]; // Wrap around to start
                                            // Shoelace sum is the cross product
        shoelace_sum += (x1 * y2) - (x2 * y1);
        if integer_lengths {
            boundary_len += T::from_i64(euclidean_distance(x1, y1, x2, y2) as i64).unwrap();
        } else {
            boundary_len += T::from_f64(euclidean_distance(x1, y1, x2, y2)).unwrap();
        }
    }
    return (shoelace_sum.abs() - boundary_len) / (T::one() + T::one()) + T::one();
}
