use day05::puzzle_a;
use day05::puzzle_b;
use day05::split_lines_by_blanks;
use day05::load;

fn main() {
    colog::init();
    let filename = "input";
    let lines = split_lines_by_blanks(&load(filename));

    let value = puzzle_a(&lines);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&lines);
    println!("Answer to 2nd question: {}", value_b);
}
