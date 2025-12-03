extern crate filelib;

pub use filelib::load_no_blanks;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

type Battery = u32;

fn parse_batteries(lines: &Vec<String>) -> Vec<Vec<Battery>> {
    return lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Input should be a number from 0-9"))
                .collect()
        })
        .collect();
}

fn get_joltage(bank: &Vec<Battery>) -> Battery {
    // because these must be sequential, a few things here
    // highest number gets multiplied by 10, so it will always be the highest number in the bank with the end sliced off.
    let tens = *bank[..bank.len() - 1]
        .iter()
        .max()
        .expect("Maximum value should be found");
    // Next find the first occurance of highest. The ones will be the max value of the splice after that.
    let index = bank
        .iter()
        .position(|&r| r == tens)
        .expect("It should exist");
    let ones = *bank[index + 1..]
        .iter()
        .max()
        .expect("At least one value should exist so should max");
    info!("Returning {} and {}", tens, ones);
    return tens * 10 + ones;
}

/// On each line, find two highest numbers, create a number from them, and add them up.
/// ```
/// let vec1: Vec<String> = vec![
///     "987654321111111", "811111111111119", "234234234234278", "818181911112111"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_a(&vec1), 357);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> Battery {
    let banks = parse_batteries(string_list);
    return banks.iter().map(|bank| get_joltage(bank)).sum();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage() {
        let mut bank: Vec<Battery> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        let mut expected = 98;
        assert_eq!(get_joltage(&bank), expected);

        bank = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        expected = 89;
        assert_eq!(get_joltage(&bank), expected);

        bank = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        expected = 78;
        assert_eq!(get_joltage(&bank), expected);

        bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        expected = 92;
        assert_eq!(get_joltage(&bank), expected);
    }
}
