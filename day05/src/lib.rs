extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;
#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

type ID = u64;

fn parse_fresh_id_ranges(string_list: &Vec<String>) -> Vec<(ID, ID)> {
    let mut result = vec![];

    for s in string_list {
        info!("Processing {}", s);
        let (start_str, end_str) = s.split_once("-").expect("Should have a dash");
        let start: ID = start_str.parse().unwrap();
        let end: ID = end_str.parse().unwrap();
        result.push((start, end));
    }

    return result;
}

fn parse_available_ids(string_list: &Vec<String>) -> Vec<ID> {
    return string_list.iter().map(|s| s.parse().unwrap()).collect();
}

fn is_fresh(ranges: &Vec<(ID, ID)>, ingreident: ID) -> bool {
    for (start, end) in ranges {
        if *start <= ingreident && *end >= ingreident {
            info!("Is fresh {}", ingreident);
            return true;
        }
    }
    return false;
}

/// Count number of fresh ingredients.
/// ```
/// let vec1: Vec<Vec<String>> = vec![
///     vec!["3-5", "10-14", "16-20", "12-18"].iter().map(|s| s.to_string()).collect(),
///     vec!["1", "5", "8", "11", "17", "32"].iter().map(|s| s.to_string()).collect(),
/// ];
/// assert_eq!(day05::puzzle_a(&vec1), 3);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> usize {
    let mut ranges = parse_fresh_id_ranges(&string_list[0]);
    ranges.sort_by(|&a, &b| a.0.cmp(&b.0));
    let available = parse_available_ids(&string_list[1]);
    return available
        .iter()
        .filter(|&&ing| is_fresh(&ranges, ing))
        .count();
}

fn merge_ranges(ranges: &Vec<(ID, ID)>) -> Vec<(ID, ID)> {
    let mut merged = vec![];
    let mut current_merged_range = ranges[0];
    let mut first = true;
    for r in ranges {
        if first {
            info!("Expanding {} {}", r.0, r.1);
            first = false;
            current_merged_range = r.clone();
            continue;
        }
        // if start of this range is less than the end of the previous, merge them
        // This is valid because of the earlier sort.
        if r.0 <= current_merged_range.1 {
            if current_merged_range.1 < r.1 {
                current_merged_range.1 = r.1;
            }
        } else {
            // new range
            info!(
                "Merged range: {} {}",
                current_merged_range.0, current_merged_range.1
            );
            merged.push(current_merged_range);

            info!("Expanding {} {}", r.0, r.1);
            current_merged_range = r.clone();
        }
    }
    info!(
        "Merged range: {} {}",
        current_merged_range.0, current_merged_range.1
    );
    merged.push(current_merged_range);
    return merged;
}

fn count_range(start: ID, end: ID) -> ID {
    return end - start + 1;
}

/// Ignore available, expand ranges to a set and count it.
/// ```
/// let vec1: Vec<Vec<String>> = vec![
///     vec!["3-5", "10-14", "16-20", "12-18"].iter().map(|s| s.to_string()).collect(),
///     vec!["1", "5", "8", "11", "17", "32"].iter().map(|s| s.to_string()).collect(),
/// ];
/// assert_eq!(day05::puzzle_b(&vec1), 14);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> ID {
    let mut ranges = parse_fresh_id_ranges(&string_list[0]);
    ranges.sort_by(|&a, &b| a.0.cmp(&b.0));
    let merged = merge_ranges(&ranges);
    return merged.iter().map(|(s, e)| count_range(*s, *e)).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ranges() {
        let vec1: Vec<String> = vec!["1-5", "2-6"].iter().map(|s| s.to_string()).collect();
        let expected = vec![(1, 5), (2, 6)];
        assert_eq!(parse_fresh_id_ranges(&vec1), expected);
    }

    #[test]
    fn test_merge_ranges() {
        let vec1 = vec![(3, 5), (10, 14), (12, 18), (16, 20)];
        let expected = vec![(3, 5), (10, 20)];
        assert_eq!(merge_ranges(&vec1), expected);
    }
}
