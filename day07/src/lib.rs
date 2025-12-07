extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::{Grid, GridCoordinate, GridPrintable, GridTraversable};

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Terrain {
    Blank,
    Splitter,
}

impl GridPrintable for Terrain {
    fn get_character(&self) -> char {
        return match self {
            Terrain::Blank => '.',
            Terrain::Splitter => '^',
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct GridLine {
    start: gridlib::GridCoordinate,
    end: gridlib::GridCoordinate,
}

fn parse_grid_and_start(string_list: &Vec<String>) -> (Grid<Terrain>, GridCoordinate) {
    let height = string_list.len();
    let width = string_list[0].len();
    let mut values = vec![];
    let mut start = GridCoordinate::new(0, 0);

    for (y, line) in string_list.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => values.push(Terrain::Blank),
                '^' => values.push(Terrain::Splitter),
                'S' => {
                    values.push(Terrain::Blank);
                    start = GridCoordinate::new(x, y);
                }
                _ => panic!("Unknown character {}", c),
            }
        }
    }

    return (Grid::new(width, height, values), start);
}

fn find_end(start: GridCoordinate, grid: &Grid<Terrain>) -> (GridLine, Vec<GridCoordinate>) {
    let mut next_starts = vec![];
    let mut end = start.clone();
    while let Some(next_coord) = grid.get_coordinate_by_direction(end, gridlib::Direction::SOUTH) {
        let v = grid.get_value(next_coord).expect("Should have a value");

        if v == Terrain::Splitter {
            // This is the end, don't update end.
            // Next starts: Get SOUTHEAST and Get SOUTHWEST from end of line and add to next_starts if possible
            // We could also do WEST and EAST of next_coord, its equivalent.
            if let Some(split_left) =
                grid.get_coordinate_by_direction(end, gridlib::Direction::SOUTHWEST)
            {
                next_starts.push(split_left);
            }
            if let Some(split_right) =
                grid.get_coordinate_by_direction(end, gridlib::Direction::SOUTHEAST)
            {
                next_starts.push(split_right);
            }
            break;
        }

        // End just follows next_coord consistently.
        end = next_coord;
    }

    return (
        GridLine {
            start: start,
            end: end,
        },
        next_starts,
    );
}

fn queue_lines_split_count(grid: &Grid<Terrain>, start: GridCoordinate) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(start);
    let mut seen_starts: HashSet<GridCoordinate> = HashSet::new();
    let mut seen_splits: HashSet<GridCoordinate> = HashSet::new();

    while let Some(cur) = queue.pop_back() {
        if seen_starts.contains(&cur) {
            continue;
        }
        seen_starts.insert(cur);

        let (z, next) = find_end(cur, &grid);
        if z.end.y != grid.get_height() - 1 {
            let splitter_loc = GridCoordinate::new(z.end.x, z.end.y + 1);
            seen_splits.insert(splitter_loc);
            info!("Split detected at {:?}", splitter_loc);
        }

        for n in next {
            queue.push_front(n);
        }
    }
    return seen_splits.len();
}

/// Count how many times this line was split.
/// ```
/// let vec1: Vec<String> = vec![
///     ".......S.......", "...............", ".......^.......",
///     "...............", "......^.^......", "...............",
///     ".....^.^.^.....", "...............", "....^.^...^....",
///     "...............", "...^.^...^.^...", "...............",
///     "..^...^.....^..", "...............", ".^.^.^.^.^...^.",
///     "..............."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day07::puzzle_a(&vec1), 21);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let (grid, start) = parse_grid_and_start(string_list);
    return queue_lines_split_count(&grid, start);
}

fn find_all_possible_paths(grid: &Grid<Terrain>, start: GridCoordinate) -> usize {
    let mut memo = HashMap::new();
    return dfs_recursive(grid, start, &mut memo);
}

fn dfs_recursive(
    grid: &Grid<Terrain>,
    current: GridCoordinate,
    memo: &mut HashMap<GridCoordinate, usize>,
) -> usize {
    // Memoization immediate return
    if let Some(&count) = memo.get(&current) {
        return count;
    }

    let (_, next_starts) = find_end(current, grid);
    if next_starts.is_empty() {
        // The line ended from here
        return 1;
    }

    let mut total_paths = 0;
    for next in next_starts {
        total_paths += dfs_recursive(grid, next, memo)
    }

    memo.insert(current, total_paths);
    return total_paths;
}

/// Count how many possible paths there are through this grid.
/// ```
/// let vec1: Vec<String> = vec![
///     ".......S.......", "...............", ".......^.......",
///     "...............", "......^.^......", "...............",
///     ".....^.^.^.....", "...............", "....^.^...^....",
///     "...............", "...^.^...^.^...", "...............",
///     "..^...^.....^..", "...............", ".^.^.^.^.^...^.",
///     "..............."
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day07::puzzle_b(&vec1), 40);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let (grid, start) = parse_grid_and_start(string_list);
    return find_all_possible_paths(&grid, start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_and_start() {
        let vec1: Vec<String> = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let (expected_grid, expected_start) = get_expected_start_and_grid();
        let (result_grid, result_start) = parse_grid_and_start(&vec1);
        assert_eq!(result_start, expected_start);
        assert_eq!(result_grid, expected_grid);
    }

    #[test]
    fn test_find_end() {
        let (grid, start) = get_expected_start_and_grid();
        let (resultline, result_next) = find_end(start, &grid);
        let expectedline = GridLine {
            start: GridCoordinate::new(7, 0),
            end: GridCoordinate::new(7, 1),
        };
        let expected_next = vec![GridCoordinate::new(6, 2), GridCoordinate::new(8, 2)];
        assert_eq!(resultline, expectedline);
        assert_eq!(result_next, expected_next);
    }

    #[test]
    fn test_a_simple() {
        let vec1: Vec<String> = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(puzzle_a(&vec1), 3);
    }

    #[test]
    fn test_a_merge_in() {
        let vec1: Vec<String> = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "........^......",
            "...............",
            ".......^.......",
            "...............",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(puzzle_a(&vec1), 3);
        /*

           ".......S.......",
           ".......|.......",
           "......|^|......",
           "......|.|......",
           "......||^|.....",
           "......||.|.....",
           "......|^||.....",
           "......|.||.....",
          7,0 -> 7,1
          6,2 -> 6,7
          8,2 -> 8,3
          7,4 -> 7,5
          9,4 -> 9,7
          8,6 -> 8,7
        */
    }

    fn get_expected_start_and_grid() -> (Grid<Terrain>, GridCoordinate) {
        let expected_start = GridCoordinate::new(7, 0);
        let expected_grid_values = vec![
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Splitter,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
            Terrain::Blank,
        ];
        return (Grid::new(15, 16, expected_grid_values), expected_start);
    }
}
