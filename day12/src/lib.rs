extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;

use std::collections::HashSet;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PolygonVariant {
    points: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PuzzlePolygon {
    id: usize,
    variations: Vec<PolygonVariant>,
}

impl PuzzlePolygon {
    fn new(id: usize, width: usize, height: usize, points: Vec<(usize, usize)>) -> Self {
        let base = PolygonVariant {
            points,
            height,
            width,
        };
        let mut variations = Self::generate_variations(base);
        variations.sort_by(|a, b| a.points.cmp(&b.points));
        return Self { id, variations };
    }

    fn generate_variations(base: PolygonVariant) -> Vec<PolygonVariant> {
        let mut unique_variants = HashSet::new();
        let mut current = base;

        // Try all 4 rotations
        for _ in 0..4 {
            // Add current
            unique_variants.insert(Self::normalize(&current));

            // Add flipped version of current
            let flipped = Self::flip(&current);
            unique_variants.insert(Self::normalize(&flipped));

            // Rotate for next iteration
            current = Self::rotate(&current);
        }

        return unique_variants.into_iter().collect();
    }

    // Rotate 90 degrees clockwise
    fn rotate(v: &PolygonVariant) -> PolygonVariant {
        let new_points: Vec<(usize, usize)> = v
            .points
            .iter()
            .map(|&(x, y)| (v.height - 1 - y, x)) // (x, y) -> (H-1-y, x)
            .collect();
        return PolygonVariant {
            points: new_points,
            width: v.height,
            height: v.width,
        };
    }

    // Flip horizontally
    fn flip(v: &PolygonVariant) -> PolygonVariant {
        let new_points: Vec<(usize, usize)> = v
            .points
            .iter()
            .map(|&(x, y)| (v.width - 1 - x, y))
            .collect();
        return PolygonVariant {
            points: new_points,
            width: v.width,
            height: v.height,
        };
    }

    // Shifts points so the top-left-most occupied point is near (0,0)
    // and sorts them to ensure HashSet handles duplicates correctly
    fn normalize(v: &PolygonVariant) -> PolygonVariant {
        if v.points.is_empty() {
            return v.clone();
        }
        let min_r = v.points.iter().map(|p| p.0).min().unwrap();
        let min_c = v.points.iter().map(|p| p.1).min().unwrap();

        let mut new_points: Vec<(usize, usize)> = v
            .points
            .iter()
            .map(|&(r, c)| (r - min_r, c - min_c))
            .collect();
        new_points.sort(); // Sorting is crucial for the HashSet dedup logic

        return PolygonVariant {
            points: new_points,
            width: v.width,
            height: v.height,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PuzzleRegion {
    width: usize,
    height: usize,
    to_fit: Vec<usize>,
}

impl PuzzleRegion {
    fn can_fit(&self, polygons: &Vec<PuzzlePolygon>) -> bool {
        // We map the IDs in `to_fit` to the actual shape definitions.
        // We assume the `all_polygons` ID == index.
        info!(
            "Trying to solve {:?}, with {} polygons",
            self.to_fit,
            polygons.len()
        );

        let mut shapes_to_place: Vec<&PuzzlePolygon> = Vec::new();

        for (id, &count) in self.to_fit.iter().enumerate() {
            if count > 0 {
                // Find the actual polygon data.
                // Assuming all_polygons is sorted by ID such that index == ID.
                // If not, use: all_polygons.iter().find(|p| p.id == id)
                let polygon = &polygons[id];

                // Add 'count' copies of this polygon to our list
                for _ in 0..count {
                    shapes_to_place.push(polygon);
                }
            }
        }

        // OPTIMIZATION: Sort largest shapes first.
        // It's much harder to place a big shape than a small one.
        shapes_to_place.sort_by(|a, b| {
            let area_a = a.variations[0].points.len();
            let area_b = b.variations[0].points.len();
            area_b.cmp(&area_a) // Descending
        });

        let area = self.width * self.height;
        let to_place_area: usize = shapes_to_place
            .iter()
            .map(|x| x.variations[0].points.len())
            .sum();
        if to_place_area > area {
            // Larger than available area, quit instantly
            info!(
                "Trying to place {} area in {} available",
                to_place_area, area
            );
            return false;
        }

        // false = empty, true = occupied
        let mut grid = vec![false; self.width * self.height];

        return self.solve_recursive(&mut grid, &shapes_to_place);
    }

    fn can_place(&self, x: usize, y: usize, variant: &PolygonVariant) -> bool {
        // Check if the shape's bounding box goes off the edge
        if x + variant.width > self.width || y + variant.height > self.height {
            return false;
        }
        return true;
    }

    /// Internal recursive solver
    fn solve_recursive(&self, grid: &mut Vec<bool>, remaining_shapes: &[&PuzzlePolygon]) -> bool {
        // Base Case: All shapes placed successfully
        if remaining_shapes.is_empty() {
            return true;
        }

        let current_shape = remaining_shapes[0];
        let next_shapes = &remaining_shapes[1..];

        // Try to place the current shape at every possible coordinate
        for y in 0..self.height {
            for x in 0..self.width {
                for variant in &current_shape.variations {
                    if self.can_place(x, y, variant) {
                        if !self.check_collision(grid, x, y, variant) {
                            self.toggle_shape(grid, x, y, variant, true);
                            if self.solve_recursive(grid, next_shapes) {
                                return true;
                            }
                            // Didn't find a solution this path, backtrack.
                            self.toggle_shape(grid, x, y, variant, false);
                        }
                    }
                }
            }
        }
        return false;
    }

    fn check_collision(
        &self,
        grid: &Vec<bool>,
        x: usize,
        y: usize,
        variant: &PolygonVariant,
    ) -> bool {
        for &(px, py) in &variant.points {
            let grid_idx = (y + py) * self.width + (x + px);
            if grid[grid_idx] {
                return true; // Collision detected
            }
        }
        return false;
    }

    // Helper to mark/unmark cells
    fn toggle_shape(
        &self,
        grid: &mut Vec<bool>,
        x: usize,
        y: usize,
        variant: &PolygonVariant,
        state: bool,
    ) {
        for &(px, py) in &variant.points {
            let grid_idx = (y + py) * self.width + (x + px);
            grid[grid_idx] = state;
        }
    }
}

fn parse_puzzle_polygon(strings: &Vec<String>) -> PuzzlePolygon {
    info!("Parsing {:?}", strings);
    let (id_str, _) = strings[0].split_once(":").unwrap();
    let id: usize = id_str.parse().unwrap();
    let rest = strings[1..].to_vec();
    let height = rest.len();
    let width = rest[0].len();
    let mut points = vec![];
    for (y, charline) in rest.iter().enumerate() {
        for (x, c) in charline.chars().enumerate() {
            match c {
                '#' => {
                    // this is a point!
                    points.push((x, y));
                }
                '.' => {
                    // Empty do nothing
                    continue;
                }
                _ => panic!("Unknown character"),
            }
        }
    }
    info!("Found {}, {}, {}, {:?}", id, width, height, points);
    return PuzzlePolygon::new(id, width, height, points);
}

fn parse_puzzle_regions(strings: &Vec<String>) -> Vec<PuzzleRegion> {
    let mut regions = vec![];
    for line in strings {
        info!("Parsing {:?}", line);
        let (dimensions, indexes) = line.split_once(": ").unwrap();
        let (width_s, height_s) = dimensions.split_once("x").unwrap();
        let width: usize = width_s.parse().unwrap();
        let height: usize = height_s.parse().unwrap();
        let to_fit: Vec<usize> = indexes.split(" ").map(|x| x.parse().unwrap()).collect();
        let region = PuzzleRegion {
            width,
            height,
            to_fit,
        };
        regions.push(region);
    }

    return regions;
}

fn parse(string_list: &Vec<Vec<String>>) -> (Vec<PuzzleRegion>, Vec<PuzzlePolygon>) {
    let regions_str = string_list.last().unwrap();
    let polygons_str = &string_list[..string_list.len() - 1];
    let mut polygons: Vec<PuzzlePolygon> = polygons_str
        .iter()
        .map(|x| parse_puzzle_polygon(x))
        .collect();
    polygons.sort_by(|a, b| a.id.cmp(&b.id));
    let regions = parse_puzzle_regions(regions_str);
    return (regions, polygons);
}

/// Find how many regions can fit the given present shapes.
/// ```
/// let vec1: Vec<Vec<String>> = vec![
///     vec![
///       "0:", "###", "##.", "##."
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "1:", "###", "##.", ".##"
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "2:", ".##", "###", "##.",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "3:", "##.", "###", "##.",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "4:", "###", "#..", "###",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "5:", "###", ".#.", "###",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "4x4: 0 0 0 0 2 0", "12x5: 1 0 1 0 2 2", "12x5: 1 0 1 0 3 2",
///     ].iter().map(|s| s.to_string()).collect()
/// ];
/// assert_eq!(day12::puzzle_a(&vec1), 2);
/// ```
pub fn puzzle_a(string_list: &Vec<Vec<String>>) -> usize {
    let (regions, polygons) = parse(string_list);
    return regions.iter().filter(|x| x.can_fit(&polygons)).count();
}

/// Foo
/// ```
/// let vec1: Vec<Vec<String>> = vec![
///     vec![
///       "0:", "###", "##.", "##."
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "1:", "###", "##.", ".##"
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "2:", ".##", "###", "##.",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "3:", "##.", "###", "##.",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "4:", "###", "#..", "###",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "5:", "###", ".#.", "###",
///     ].iter().map(|s| s.to_string()).collect(),
///     vec![
///       "4x4: 0 0 0 0 2 0", "12x5: 1 0 1 0 2 2", "12x5: 1 0 1 0 3 2",
///     ].iter().map(|s| s.to_string()).collect()
/// ];
/// assert_eq!(day12::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<Vec<String>>) -> u32 {
    let (regions, polygons) = parse(string_list);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let vec1: Vec<Vec<String>> = vec![
            vec!["0:", "###", "##.", "##."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["1:", "###", "##.", ".##"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["2:", ".##", "###", "##."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["3:", "##.", "###", "##."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["4:", "###", "#..", "###"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["5:", "###", ".#.", "###"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["4x4: 0 0 0 0 2 0", "12x5: 1 0 1 0 2 2", "12x5: 1 0 1 0 3 2"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ];

        let expected_regions = vec![
            PuzzleRegion {
                width: 4,
                height: 4,
                to_fit: vec![0, 0, 0, 0, 2, 0],
            },
            PuzzleRegion {
                width: 12,
                height: 5,
                to_fit: vec![1, 0, 1, 0, 2, 2],
            },
            PuzzleRegion {
                width: 12,
                height: 5,
                to_fit: vec![1, 0, 1, 0, 3, 2],
            },
        ];
        let expected_polygons = vec![
            PuzzlePolygon::new(
                0,
                3,
                3,
                vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
            ),
            PuzzlePolygon::new(
                1,
                3,
                3,
                vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            ),
            PuzzlePolygon::new(
                2,
                3,
                3,
                vec![(1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2)],
            ),
            PuzzlePolygon::new(
                3,
                3,
                3,
                vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2)],
            ),
            PuzzlePolygon::new(
                4,
                3,
                3,
                vec![(0, 0), (1, 0), (2, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            ),
            PuzzlePolygon::new(
                5,
                3,
                3,
                vec![(0, 0), (1, 0), (2, 0), (1, 1), (0, 2), (1, 2), (2, 2)],
            ),
        ];

        let result = parse(&vec1);
        assert_eq!(result.0, expected_regions, "Mismatch region");
        for (polygon, expected) in result.1.iter().zip(expected_polygons.iter()) {
            assert_eq!(polygon.id, expected.id);
            assert_eq!(polygon.variations[0], expected.variations[0]);
        }
    }

    #[test]
    fn test_puzzle_a_debug() {
        let vec1: Vec<Vec<String>> = vec![
            vec!["0:", "###", "##.", "##."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["1:", "###", "##.", ".##"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["2:", ".##", "###", "##."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["3:", "##.", "###", "##."]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["4:", "###", "#..", "###"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["5:", "###", ".#.", "###"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["4x4: 0 0 0 0 2 0", "12x5: 1 0 1 0 2 2", "12x5: 1 0 1 0 3 2"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ];
        assert_eq!(puzzle_a(&vec1), 2);
    }
}
