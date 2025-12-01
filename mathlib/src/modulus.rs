use std::ops::{Add, Rem};

/// Provide modulus instead of default remainder.
/// ```
/// assert_eq!(-50 % 100, -50);
/// assert_eq!(mathlib::modulus(-50, 100), 50);
/// ```
pub fn modulus<T>(a: T, b: T) -> T
where
    // Need Rem trait (Remainder)
    // Need Add trait (for implementation)
    // Need Copy (to make the result)
    T: Rem<Output = T> + Add<Output = T> + Copy,
{
    // % is actually the remainder function, not the modulus function
    // This is the workaround way to "fix" this.
    return ((a % b) + b) % b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulus_one_neg() {
        let result = modulus(-2, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn modulus_both_neg() {
        let result = modulus(-2, -3);
        assert_eq!(result, -2);
    }

    #[test]
    fn modulus_both_pos() {
        let result = modulus(120i64, 3i64);
        assert_eq!(result, 0);
    }
}
