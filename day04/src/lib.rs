extern crate filelib;
extern crate gridlib;

pub use filelib::load_no_blanks;
use gridlib::GridTraversable;
use log::info;

#[derive(Copy, Clone, PartialEq, Eq)]
enum GridType {
    Paper,
    Empty,
}

impl gridlib::GridPrintable for GridType {
    fn get_character(&self) -> char {
        return match self {
            GridType::Paper => '@',
            GridType::Empty => '.',
        };
    }
}

type ParsedGrid = gridlib::Grid<GridType>;

fn parse_grid(string_list: &Vec<String>) -> ParsedGrid {
    let height = string_list.len();
    let width = string_list[0].len();
    let mut values = vec![];

    for line in string_list {
        for char in line.chars() {
            let v: GridType = match char {
                '@' => GridType::Paper,
                '.' => GridType::Empty,
                _ => panic!("Unknown character {}", char),
            };
            values.push(v);
        }
    }
    return ParsedGrid::new(width, height, values);
}

// type for debugging:
#[derive(Copy, Clone, Debug)]
struct Overlay {
    coord: gridlib::GridCoordinate,
}

impl gridlib::GridPrintable for Overlay {
    fn get_character(&self) -> char {
        return 'X';
    }
}

impl gridlib::GridOverlay for Overlay {
    fn get_position(&self) -> gridlib::GridCoordinate {
        return self.coord;
    }
}

// Find the max neighbors
fn max_neighbors(grid: &ParsedGrid, max_neighbors: usize) -> Vec<Overlay> {
    let mut result = vec![];
    for coord in grid.coord_iter() {
        if grid.get_value(coord).unwrap() == GridType::Empty {
            continue;
        }
        let num_matching: usize = grid
            .get_all_adjacent_coordinates(coord)
            .iter()
            .map(|&x| {
                if grid.get_value(x).unwrap() == GridType::Paper {
                    1
                } else {
                    0
                }
            })
            .sum();
        if num_matching >= max_neighbors {
            continue;
        }
        result.push(Overlay { coord });
    }
    return result;
}

fn print_solution(grid: &ParsedGrid, overlay: &Vec<Overlay>) {
    info!("Grid solution {}:\n", overlay.len());
    for line in grid.grid_strings_with_overlay(overlay.clone()) {
        info!("{}", line);
    }
}

/// Find rolls of paper with 4 or more neighbors.
/// ```
/// let vec1: Vec<String> = vec![
///     "..@@.@@@@.", "@@@.@.@.@@", "@@@@@.@.@@", "@.@@@@..@.", "@@.@@@@.@@",
///     ".@@@@@@@.@", ".@.@.@.@@@", "@.@@@.@@@@", ".@@@@@@@@.", "@.@.@@@.@."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day04::puzzle_a(&vec1), 13);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let parsed = parse_grid(string_list);
    let solution = max_neighbors(&parsed, 4);
    print_solution(&parsed, &solution);
    return solution.len();
}

fn remove_from_grid(grid: &ParsedGrid, overlay: &Vec<Overlay>) -> ParsedGrid {
    let mut new_grid = grid.clone();
    for coordwrapper in overlay.iter() {
        new_grid.set_value(coordwrapper.coord, GridType::Empty);
    }
    return new_grid;
}

/// Iterate over the grid, removing paper until none can be removed.
/// ```
/// let vec1: Vec<String> = vec![
///     "..@@.@@@@.", "@@@.@.@.@@", "@@@@@.@.@@", "@.@@@@..@.", "@@.@@@@.@@",
///     ".@@@@@@@.@", ".@.@.@.@@@", "@.@@@.@@@@", ".@@@@@@@@.", "@.@.@@@.@."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day04::puzzle_b(&vec1), 43);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let mut parsed = parse_grid(string_list);
    let mut solution = max_neighbors(&parsed, 4);
    let mut total = 0;
    while solution.len() > 0 {
        print_solution(&parsed, &solution);

        total += solution.len();
        parsed = remove_from_grid(&parsed, &solution);
        solution = max_neighbors(&parsed, 4);
    }
    return total;
}
