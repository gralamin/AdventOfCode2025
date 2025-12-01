extern crate filelib;

pub use filelib::load_no_blanks;
use log::info;
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

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day01::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

/// Delete this after starting on puzzle_a.
/// ```
/// let vec1: Vec<u32> = vec![];
/// let vec2: Vec<u32> = vec![1];
/// assert_eq!(day01::coverage_workaround(&vec1), 1);
/// assert_eq!(day01::coverage_workaround(&vec2), 2);
/// ```
pub fn coverage_workaround(a: &Vec<u32>) -> u32 {
    if a.len() == 0 {
        info!("Example logging of {:?}", a);
        return 1;
    } else {
        return 2;
    }
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
}
