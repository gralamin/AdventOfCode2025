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

fn parse_problems_rtl_col(string_list: &Vec<String>) -> Vec<Problem> {
    let mut result = vec![];

    let lines: Vec<Vec<char>> = string_list.iter().map(|l| l.chars().collect()).collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut cur_op = Operation::Multiply;
    let mut cur_col: Vec<Num> = vec![];

    // Since we need to find these columns need to iterate by width instead
    // Remember to read from right to left.
    for rev_x in 0..max_width {
        let x = max_width - rev_x - 1;
        let mut col_chars = String::new();

        for line in &lines {
            if let Some(&c) = line.get(x) {
                match c {
                    // This works because of how the columns are formed.
                    '+' => {
                        cur_op = Operation::Plus;
                        // place a space because we can easily remove it when parsing later
                        // But we aren't empty to make it clear this isn't a seperator.
                        col_chars.push(' ');
                    }
                    '*' => {
                        cur_op = Operation::Multiply;
                        // place a space because we can easily remove it when parsing later
                        // But we aren't empty to make it clear this isn't a seperator.
                        col_chars.push(' ');
                    }
                    _ => {
                        if !c.is_whitespace() {
                            col_chars.push(c);
                        }
                    }
                }
            }
        }

        info!("Reading column: {}", col_chars);

        // Process column
        if col_chars.is_empty() {
            // seperator between columns
            result.push(Problem::new(cur_col, cur_op));
            cur_col = vec![];
            continue;
        }
        let num = col_chars.trim().parse().expect("Should be parsable");
        cur_col.push(num);
    }
    // Final column handling
    result.push(Problem::new(cur_col, cur_op));

    return result;
}

/// Much harder right to left column parsing, to solve. Rest still works as it did before.
/// ```
/// let vec1: Vec<String> = vec![
///     "123 328  51 64 ", " 45 64  387 23 ", "  6 98  215 314", "*   +   *   +  "
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day06::puzzle_b(&vec1), 3263827);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> Num {
    return parse_problems_rtl_col(string_list)
        .iter()
        .map(|x| x.solve())
        .sum();
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

    #[test]
    fn test_parse_b() {
        let input: Vec<String> = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let expected = vec![
            Problem::new(vec![4, 431, 623], Operation::Plus),
            Problem::new(vec![175, 581, 32], Operation::Multiply),
            Problem::new(vec![8, 248, 369], Operation::Plus),
            Problem::new(vec![356, 24, 1], Operation::Multiply),
        ];
        assert_eq!(parse_problems_rtl_col(&input), expected);
    }

    #[test]
    #[should_panic(expected = "Unknown character")]
    fn test_parse_panic_invalid_op() {
        // Covers Line 56: Panic on unknown operator in last line
        let input = vec!["10 20", "20 30", "+  Z "]
            .iter()
            .map(|s| s.to_string())
            .collect();

        parse_problems(&input);
    }
}
