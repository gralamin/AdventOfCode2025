extern crate filelib;

pub use filelib::load_no_blanks;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

use mathlib::modulus;

type PuzzleInt = i32;

// 0 through 99 in order
// Right = addition
// Left = subtraction
// % 100 the value
// count how many times its at 0

fn convert_ints(string_list: &Vec<String>) -> Vec<PuzzleInt> {
    return string_list
        .iter()
        .map(|s: &String| {
            if s.starts_with("L") {
                -1 * s[1..].parse::<PuzzleInt>().unwrap()
            } else {
                s[1..].parse::<PuzzleInt>().unwrap()
            }
        })
        .collect();
}

fn move_int(value: PuzzleInt, rotation: PuzzleInt) -> PuzzleInt {
    return modulus(value + rotation, 100);
}

/// Count how many times we arrive at 0.
/// ```
/// let vec1: Vec<String> = vec![
///     "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_a(&vec1), 3);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let input = convert_ints(string_list);
    let mut v = 50;
    let mut count: usize = 0;
    for i in input {
        v = move_int(v, i);
        if v == 0 {
            count += 1;
        }
    }
    return count;
}

fn move_counting_0s(value: PuzzleInt, rotation: PuzzleInt) -> (PuzzleInt, usize) {
    let mut count = usize::try_from(rotation.abs() / 100).unwrap();
    let remainder_0 = rotation % 100;

    info!(
        "After dividing, have count {}, and remainder {}",
        count, remainder_0
    );

    // next check if modulus is different from the addition
    let addition = value + remainder_0;
    let modded = modulus(addition, 100);
    if value != 0 && modded != addition || addition == 0 {
        count += 1;
        info!("Detected past 0, count is {}", count);
    }
    return (modded, count);
}

/// Count how many times we pass by 0.
/// ```
/// let vec1: Vec<String> = vec![
///     "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_b(&vec1), 6);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let input = convert_ints(string_list);
    let mut v = 50;
    let mut count: usize = 0;
    for i in input {
        info!("v {}, i {} ", v, i);
        let count_up;
        (v, count_up) = move_counting_0s(v, i);
        count += count_up;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let vec1: Vec<String> = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let expected = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(convert_ints(&vec1), expected);
    }

    #[test]
    fn test_move_above_99() {
        let i = 99;
        let r = 20;
        let expected = 19;
        assert_eq!(move_int(i, r), expected);
    }

    #[test]
    fn test_move_below_0() {
        let i = 5;
        let r = -10;
        let expected = 95;
        assert_eq!(move_int(i, r), expected);
    }

    #[test]
    fn test_count_passing_0_neg() {
        let i = 5;
        let r = -10;
        let expected = (95, 1);
        assert_eq!(move_counting_0s(i, r), expected);
    }

    #[test]
    fn test_count_passing_hit_0_neg() {
        let i = 5;
        let r = -5;
        let expected = (0, 1);
        assert_eq!(move_counting_0s(i, r), expected);
    }

    #[test]
    fn test_count_passing_0_pos() {
        let i = 95;
        let r = 10;
        let expected = (5, 1);
        assert_eq!(move_counting_0s(i, r), expected);
    }

    #[test]
    fn test_count_passing_hit_0_pos() {
        let i = 95;
        let r = 5;
        let expected = (0, 1);
        assert_eq!(move_counting_0s(i, r), expected);
    }

    #[test]
    fn test_count_passing_multiple_times_pos() {
        let i = 95;
        let r = 5000;
        let expected = (95, 50);
        assert_eq!(move_counting_0s(i, r), expected);
    }

    #[test]
    fn test_count_passing_multiple_times_neg() {
        let i = 5;
        let r = -5000;
        let expected = (5, 50);
        assert_eq!(move_counting_0s(i, r), expected);
    }

    #[test]
    fn test_count_passing_0_neg_start_0() {
        let i = 0;
        let r = -5;
        let expected = (95, 0);
        assert_eq!(move_counting_0s(i, r), expected);
    }

    #[test]
    fn test_correct_remainder() {
        let i = 98;
        let r = -651;
        let expected = (47, 6);
        assert_eq!(move_counting_0s(i, r), expected);
    }
}
