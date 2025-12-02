extern crate filelib;

pub use filelib::load_no_blanks;
use log::info;

type RangeType = usize;
type ResultType = usize;

fn get_ranges(input: &String) -> Vec<(RangeType, RangeType)> {
    let mut output = vec![];
    for range_string in input.split(",") {
        let (first_str, last_str) = range_string.split_once("-").expect("Should contain a dash");
        let first = first_str.parse().expect("Valid number string");
        let last = last_str.parse().expect("Valid number string");
        output.push((first, last));
    }
    return output;
}

fn find_invalid_ids(first: RangeType, last: RangeType) -> Vec<ResultType> {
    // Naive way, iterate over, returning ever invalid id
    let mut output = vec![];
    for i in first..=last {
        if is_repeated_twice(i) {
            info!("Found repeat: {}", i);
            output.push(i.try_into().expect("u32 should fit in u64"));
        }
    }
    return output;
}

fn is_repeated_twice(number: RangeType) -> bool {
    // Must have 2+ digits, must be even number of digits
    if number < 10 {
        return false;
    }

    // log10 is the number of digits.
    let digit_count = (number as f64).log10().floor() as RangeType + 1;
    if digit_count % 2 != 0 {
        return false;
    }

    // Midpoint divisor to check first vs second half
    let base: RangeType = 10;
    let divisor = base.pow(
        (digit_count / 2)
            .try_into()
            .expect("Anything larger than a u32 here naturally cannot be used as an exponent"),
    );
    let first_half = number / divisor;
    let second_half = number % divisor;
    return first_half == second_half;
}

/// Find all strings repeated twice, add them together
/// ```
/// let vec1: Vec<String> = vec![
///     "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_a(&vec1), 1227775554);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> ResultType {
    let ranges = get_ranges(string_list.first().expect("At least one line of input"));
    return ranges
        .iter()
        .map(|(first, last)| {
            find_invalid_ids(*first, *last)
                .into_iter()
                .sum::<ResultType>()
        })
        .sum::<ResultType>();
}

fn find_invalid_ids_b(first: RangeType, last: RangeType) -> Vec<ResultType> {
    // Naive way, iterate over, returning every invalid id
    let mut output = vec![];
    for i in first..=last {
        if is_repeated_any_number_of_times(i) {
            info!("Found repeat: {}", i);
            output.push(i.try_into().expect("u32 should fit in u64"));
        }
    }
    return output;
}

fn is_repeated_any_number_of_times(number: RangeType) -> bool {
    // Maybe accidentally solved problem 2 ahead of time, lol.
    let s = number.to_string();
    let doubled = format!("{}{}", s, s);
    let search_area = &doubled[1..doubled.len() - 1];
    // If the original sring exists in the shaved double, its periodic.
    // This is called a rotation trick.
    return search_area.contains(&s);
}

/// Find all strings repeated any number of times, return them together.
/// ```
/// let vec1: Vec<String> = vec![
///     "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day02::puzzle_b(&vec1), 4174379265);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> ResultType {
    let ranges = get_ranges(string_list.first().expect("At least one line of input"));
    return ranges
        .iter()
        .map(|(first, last)| {
            find_invalid_ids_b(*first, *last)
                .into_iter()
                .sum::<ResultType>()
        })
        .sum::<ResultType>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ranges() {
        let input = "11-22,2121212118-2121212124";
        let expected = vec![(11, 22), (2121212118, 2121212124)];
        assert_eq!(get_ranges(&input.to_string()), expected);
    }

    #[test]
    fn test_find_invalid_ids() {
        let result = find_invalid_ids(11, 22);
        let expected = vec![11, 22];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_puzzle_1_ids() {
        let mut result = find_invalid_ids(99, 115);
        let mut expected = vec![99];
        assert_eq!(result, expected);

        result = find_invalid_ids(998, 1012);
        expected = vec![1010];
        assert_eq!(result, expected);

        result = find_invalid_ids(1188511880, 1188511890);
        expected = vec![1188511885];
        assert_eq!(result, expected);

        result = find_invalid_ids(222220, 222224);
        expected = vec![222222];
        assert_eq!(result, expected);

        result = find_invalid_ids(1698522, 1698528);
        expected = vec![];
        assert_eq!(result, expected);

        result = find_invalid_ids(446443, 446449);
        expected = vec![446446];
        assert_eq!(result, expected);

        result = find_invalid_ids(38593856, 38593862);
        expected = vec![38593859];
        assert_eq!(result, expected);

        result = find_invalid_ids(565653, 565659);
        expected = vec![];
        assert_eq!(result, expected);

        result = find_invalid_ids(824824821, 824824827);
        expected = vec![];
        assert_eq!(result, expected);

        result = find_invalid_ids(2121212118, 2121212124);
        expected = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_invalid_ids_n_times() {
        let result = find_invalid_ids_b(11110, 11112);
        let expected = vec![11111];
        assert_eq!(result, expected);
    }
}
