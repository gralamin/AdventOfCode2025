extern crate num_traits;

use num_traits::{Num, Signed};

/// Find the greatest common divisor between two numbers.
/// ```
/// assert_eq!(mathlib::gcd(54, 24), 6);
/// ```
pub fn gcd<T: Num + Copy>(a: T, b: T) -> T {
    let mut cur_b = b;
    let mut cur_a = a;
    while cur_b != T::zero() {
        let temp = cur_b;
        cur_b = cur_a % cur_b;
        cur_a = temp
    }
    return cur_a;
}

/// Least Common Multiple of two numbers
/// ```
/// assert_eq!(mathlib::lcm(4, 6), 12);
/// ```
pub fn lcm<T: Num + Copy + Signed>(a: T, b: T) -> T {
    if a == T::zero() || b == T::zero() {
        return T::zero();
    }
    // Divide first to avoid overflow!
    return (a.abs() / gcd(a, b)) * b.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_neg() {
        let result = lcm(-2, -4);
        assert_eq!(result, -4);
    }

    #[test]
    fn lcm_left_zero() {
        let result = lcm(0, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn lcm_right_zero() {
        let result = lcm(2, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn gcd_zero() {
        let result = gcd(1, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn gcd_zero_right() {
        let result = gcd(0, 1);
        assert_eq!(result, 1);
    }
}
