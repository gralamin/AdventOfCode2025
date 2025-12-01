extern crate num_traits;

use num_traits::Num;

/// Two by two matrix determinant, useful when you have two vectors.
/// ```
/// assert_eq!(mathlib::determinant(1, 3, 4, 5), -7);
/// ```
pub fn determinant<T: Num + Copy>(x1: T, y1: T, x2: T, y2: T) -> T {
    return x1 * y2 - x2 * y1;
}

/// Check if two lines intersect, and if so, where
/// ```
/// assert_eq!(mathlib::line_intersect(1, 2, 3, 4, 1, 4, 3, 2).unwrap(), (2, 3));
/// assert_eq!(mathlib::line_intersect(0, 2, 0, 1, 1, 2, 1, 1), None);
/// ```
pub fn line_intersect<T: Num + Copy>(
    ax: T,
    ay: T,
    bx: T,
    by: T,
    cx: T,
    cy: T,
    dx: T,
    dy: T,
) -> Option<(T, T)> {
    let vector_ab_x = ax - bx;
    let vector_ab_y = ay - by;
    let vector_cd_x = cx - dx;
    let vector_cd_y = cy - dy;
    let denominator = determinant(vector_ab_x, vector_ab_y, vector_cd_x, vector_cd_y);

    if denominator.is_zero() {
        return None;
    }

    let det_ab = determinant(ax, ay, bx, by);
    let det_cd = determinant(cx, cy, dx, dy);
    let x = (det_ab * (cx - dx) - (ax - bx) * det_cd) / denominator;
    let y = (det_ab * (cy - dy) - (ay - by) * det_cd) / denominator;

    return Some((x, y));
}
