extern crate filelib;

use good_lp::{IntoAffineExpression, Solution, SolverModel, highs, variable, variables};
#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;
use std::{
    collections::{HashSet, VecDeque},
    usize,
};

pub use filelib::load_no_blanks;

type JoltNum = u32;
type Joltages = Vec<JoltNum>;
type Lights = Vec<bool>;
type Button = Vec<usize>;

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    light_goal: Lights,
    cur_light: Lights,
    buttons: Vec<Button>,
    cur_joltage: Joltages,
    joltage_goal: Joltages,
}

impl Machine {
    fn new(light_goal: Lights, buttons: Vec<Button>, joltages: Joltages) -> Self {
        let cur_light = vec![false; light_goal.len()];
        let cur_joltage = vec![0; joltages.len()];
        return Machine {
            light_goal: light_goal,
            cur_light: cur_light,
            buttons: buttons,
            cur_joltage: cur_joltage,
            joltage_goal: joltages,
        };
    }

    fn push_button(&mut self, index: usize) {
        for idx in self.buttons[index].clone() {
            self.cur_light[idx] = !self.cur_light[idx];
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<Machine> {
    let mut machines = vec![];
    for line in lines {
        let (inds, rest) = line
            .split_once("] ")
            .expect("Line should have indicator lights");
        let lights = parse_indicator_light(&inds[1..]);

        let (buttons_str, jolts) = rest.split_once(" {").expect("Line should have joltages");
        let button_str_split = buttons_str.split(" ").collect();
        let buttons = parse_buttons(&button_str_split);
        let joltages = parse_joltage(&jolts[..jolts.len() - 1]);

        let machine = Machine::new(lights, buttons, joltages);
        machines.push(machine);
    }
    return machines;
}

fn parse_indicator_light(s: &str) -> Lights {
    info!("Parsing light: {}", s);
    let mut result = Lights::new();

    for c in s.chars() {
        result.push(match c {
            '.' => false,
            '#' => true,
            _ => panic!("Unknown char {}", c),
        });
    }

    return result;
}

fn parse_buttons(buttons: &Vec<&str>) -> Vec<Button> {
    let mut result = vec![];
    for button in buttons {
        info!("Parsing button: {}", button);
        let button = button[1..button.len() - 1]
            .split(",")
            .map(|x| x.parse::<usize>().expect("number should be parsable"))
            .collect();
        result.push(button);
    }

    return result;
}

fn parse_joltage(s: &str) -> Joltages {
    info!("Parsing joltage: {}", s);
    return s
        .split(",")
        .map(|x| x.parse::<JoltNum>().expect("Should be parsable"))
        .collect();
}

fn find_fewest_presses(m: &Machine) -> usize {
    // Essentially BFS the buttons, with a "seen state" tracker
    // If we have seen a state before, there is a quicker way to get here, so we can safely discard it
    // or there is an equivalent path, in which case we can discard it.
    let mut seen_states: HashSet<(usize, Lights)> = HashSet::new();
    let mut queue = VecDeque::new();
    for index in 0..m.buttons.len() {
        queue.push_back((index, m.clone(), vec![]));
    }

    while let Some((cur_button, mut cur_machine, path)) = queue.pop_front() {
        cur_machine.push_button(cur_button);
        let mut new_path = path.clone();
        new_path.push(cur_button);
        info!(
            "Checking state: {}, {:?}, {:?}",
            cur_button, cur_machine.cur_light, path
        );
        if seen_states.contains(&(cur_button, cur_machine.cur_light.clone())) {
            info!("Discarding seen");
            continue;
        }
        seen_states.insert((cur_button, cur_machine.cur_light.clone()));

        if cur_machine.cur_light == cur_machine.light_goal {
            info!("Found solution {:?}", new_path);
            return new_path.len();
        }

        // Not found the solution yet, try pressing every button.
        for index in 0..m.buttons.len() {
            queue.push_back((index, cur_machine.clone(), new_path.clone()));
        }
    }

    info!("Could not find solution, returning usize max");
    return usize::MAX;
}

/// Find the fewest presses required for the lights
/// ```
/// let vec1: Vec<String> = vec![
///     "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day10::puzzle_a(&vec1), 7);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let machines = parse_lines(string_list);
    return machines.iter().map(|m| find_fewest_presses(m)).sum();
}

fn find_fewest_presses_for_jolt(m: &Machine) -> usize {
    // Run good_lp against it, to solve the linear program.
    let mut problem_vars = variables!();

    // Create a variable for each button representing how many times it is pressed.
    // We constrain this to be a positive integer.
    let button_press_vars: Vec<_> = (0..m.buttons.len())
        .map(|i| problem_vars.add(variable().min(0).integer().name(format!("btn_{}", i))))
        .collect();

    // Define the Objective: Minimize the total number of button presses
    let total_presses = button_press_vars
        .iter()
        .fold(0.into_expression(), |acc, &v| acc + v);
    // Fold because I always forgethow it works: Its a running total, accumulating each value into the expression.
    // Its different from reduce, because it lets you change the type.
    let mut problem = highs(problem_vars.minimise(total_presses));

    // Build Constraints:
    // For every specific joltage index, the sum of contributions from all buttons must equal the goal for that component.

    // Initialize an expression for every joltage component (e.g., component 0, component 1...)
    let mut component_equations = vec![0.into_expression(); m.joltage_goal.len()];

    // Iterate over every button to see which components it affects
    for (button_idx, affected_indices) in m.buttons.iter().enumerate() {
        let press_count_var = button_press_vars[button_idx];

        // Add this button's variable to the equation of every component it affects
        for &component_idx in affected_indices {
            component_equations[component_idx] += press_count_var;
        }
    }

    // Apply the equality constraint: Calculated Sum == Goal Value
    for (equation, &target_value) in component_equations.into_iter().zip(&m.joltage_goal) {
        problem.add_constraint(equation.eq(target_value));
    }

    let solution = problem
        .solve()
        .expect("Linear program failed to find a solution");

    // Aggregate Result
    // round to handle floating point epsilon errors (e.g. 2.99999 -> 3)
    return button_press_vars
        .iter()
        .map(|&v| solution.value(v))
        .map(|f| f.round() as usize)
        .sum();
}

/// Find the fewest button presses for the joltage requirements
/// ```
/// let vec1: Vec<String> = vec![
///     "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day10::puzzle_b(&vec1), 33);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let machines = parse_lines(string_list);
    return machines
        .iter()
        .map(|m| find_fewest_presses_for_jolt(m))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()];
        let light_goal = vec![false, true, true, false];
        let buttons = vec![
            vec![3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![0, 2],
            vec![0, 1],
        ];
        let joltage = vec![3, 5, 4, 7];
        let expected = Machine::new(light_goal, buttons, joltage);
        let result = parse_lines(&input)[0].clone();
        assert_eq!(result.light_goal, expected.light_goal);
        assert_eq!(result.joltage_goal, expected.joltage_goal);
        assert_eq!(result.buttons, expected.buttons);
    }

    #[test]
    fn test_part_a_debug() {
        // get better debug info from unit tests then doc tests
        let vec1: Vec<String> = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(puzzle_a(&vec1), 7);
    }

    #[test]
    fn test_part_b_debug() {
        // get better debug info from unit tests then doc tests
        let vec1: Vec<String> = vec![
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(puzzle_b(&vec1), 33);
    }
}
