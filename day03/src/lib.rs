extern crate filelib;

pub use filelib::load_no_blanks;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

type Battery = u32;
type LargeBattery = u64;

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

fn get_large_joltage(bank: &Vec<Battery>, steps: u32) -> LargeBattery {
    // Recursive is easier to write, and since steps is 1->12, we will never stack overflow.
    if steps == 0 {
        // Should never happen, this is here just in case.
        return 0;
    }
    if steps == 1 {
        // just return the max of the remaining digits
        return (*bank.iter().max().expect("must exist"))
            .try_into()
            .unwrap();
    }

    let steps_usize: usize = steps.try_into().unwrap();

    // What do we need to search? Consider 12 and
    // 818181911112111
    // We know it needs 12 digits, so we can remove the last 11
    // 8181
    // Because its 11 not 12, we need to remember to add 1.
    let max_search = bank.len() - steps_usize + 1;
    let search_bank = &bank[0..max_search];
    info!("Searching {:?}", search_bank);

    // Now that we have what to search, find the max.
    let max = *search_bank.iter().max().unwrap();
    // And find the index again.
    let index = bank
        .iter()
        .position(|&r| r == max)
        .expect("It should exist");

    // Now we need to start creating the result. Its larger than a u32, so convert the type
    // The digit we are at it based on steps. Remember that if this was the 2nd last digit (steps 2) we would it to be 10 (10^1).
    let max_large: LargeBattery = max.try_into().unwrap();
    let base: LargeBattery = 10;
    let v: LargeBattery = base.pow(steps - 1) * max_large;

    // Now what do we want our sub check to look at?
    // Well it needs to ignore us, and then all the rest of the numbers are available.
    let sub_bank = bank[index + 1..].to_vec();
    info!("Found {} getting subdigits", v);
    return v + get_large_joltage(&sub_bank, steps - 1);
}

/// Same thing but now its twelve numbers instead of 2.
/// ```
/// let vec1: Vec<String> = vec![
///     "987654321111111", "811111111111119", "234234234234278", "818181911112111"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day03::puzzle_b(&vec1), 3121910778619);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> LargeBattery {
    let banks = parse_batteries(string_list);
    return banks.iter().map(|bank| get_large_joltage(bank, 12)).sum();
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

    #[test]
    fn test_large_joltage() {
        let bank = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        let expected = 888911112111;
        assert_eq!(get_large_joltage(&bank, 12), expected);
    }
}
