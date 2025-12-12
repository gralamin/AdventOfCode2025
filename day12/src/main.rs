use day12::load;
use day12::puzzle_a;
use day12::puzzle_b;
use day12::split_lines_by_blanks;

fn main() {
    colog::init();
    let filename = "input";
    let input = load(filename);
    let lines = split_lines_by_blanks(&input);

    let value = puzzle_a(&lines);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&lines);
    println!("Answer to 2nd question: {}", value_b);
}
