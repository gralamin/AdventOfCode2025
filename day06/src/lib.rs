extern crate filelib;

pub use filelib::load_no_blanks;
#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

type Num = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Plus,
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    nums: Vec<Num>,
    op: Operation,
}

impl Problem {
    fn new(nums: Vec<Num>, op: Operation) -> Problem {
        return Problem { nums: nums, op: op };
    }

    fn solve(&self) -> Num {
        info!("Solving: {:?}", self);
        return match self.op {
            Operation::Plus => self.nums.iter().sum(),
            Operation::Multiply => self.nums.iter().product(),
        };
    }
}

fn parse_problems(string_list: &Vec<String>) -> Vec<Problem> {
    let mut result = vec![];
    let mut ops = vec![];
    let mut all_nums: Vec<Vec<Num>> = vec![];
    let mut first = true;
    for line in string_list.iter().rev() {
        let split_line: Vec<&str> = line.trim().split(" ").filter(|x| x.len() > 0).collect();
        info!("Reading {:?}", split_line);
        if first {
            first = false;

            // set up the all_nums
            for _ in 0..split_line.len() {
                all_nums.push(vec![]);
            }

            // This will be ops.
            for s in split_line {
                let op = match s {
                    "*" => Operation::Multiply,
                    "+" => Operation::Plus,
                    _ => panic!("Unknown character in last line \"{}\"", s),
                };
                ops.push(op);
            }
            continue;
        }
        for (i, s) in split_line.iter().enumerate() {
            let n = s.parse().expect("Number should be parsable.");
            all_nums[i].push(n);
        }
    }
    for (op, num) in ops.iter().zip(all_nums) {
        // correct the way since we reversed earlier
        let unrev = num.iter().map(|&x| x).rev().collect();
        result.push(Problem::new(unrev, *op));
    }
    return result;
}

/// Find the grand total by adding together the solution to each vertical problem.
/// ```
/// let vec1: Vec<String> = vec![
///     "123 328  51 64 ", " 45 64  387 23 ", "  6 98  215 314", "*   +   *   +  "
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day06::puzzle_a(&vec1), 4277556);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> Num {
    return parse_problems(string_list).iter().map(|x| x.solve()).sum();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day06::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let input = vec![" 45 123", "133   6", "*   +   "]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = vec![
            Problem::new(vec![45, 133], Operation::Multiply),
            Problem::new(vec![123, 6], Operation::Plus),
        ];
        assert_eq!(parse_problems(&input), expected);
    }

    #[test]
    fn test_solve_multiply() {
        let input = Problem::new(vec![45, 133], Operation::Multiply);
        let expected = 45 * 133;
        assert_eq!(input.solve(), expected);
    }

    #[test]
    fn test_solve_plus() {
        let input = Problem::new(vec![123, 6], Operation::Plus);
        let expected = 129;
        assert_eq!(input.solve(), expected);
    }
}
