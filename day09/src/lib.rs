extern crate filelib;

pub use filelib::load_no_blanks;
use gridlib::GridCoordinateInf;
use log::info;

type Area = u64;
type Coord = GridCoordinateInf<i32>;

fn parse_coordinates(string_list: &Vec<String>) -> Vec<Coord> {
    let mut parsed = vec![];
    for l in string_list {
        let (xstr, ystr) = l.split_once(",").expect("Should split");
        let x = xstr.parse().expect("Should fit");
        let y = ystr.parse().expect("should fit");
        parsed.push(Coord::new(x, y));
    }
    return parsed;
}

fn calc_area(a: Coord, b: Coord) -> Area {
    let dx = ((a.x - b.x).abs() + 1) as Area; // need to add 1 as the border is counted
    let dy = ((a.y - b.y).abs() + 1) as Area; // need to add 1 as the border is counted
    return (dx * dy).try_into().expect("Should fit in Area");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PotentialRect {
    u: usize,
    v: usize,
    area: Area,
}

fn find_all_rects(coords: Vec<Coord>) -> Vec<PotentialRect> {
    let mut results = vec![];
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let area = calc_area(coords[i], coords[j]);
            info!("{:?} to {:?} - area {}", coords[i], coords[j], area);
            results.push(PotentialRect {
                u: i,
                v: j,
                area: area,
            });
        }
    }
    return results;
}

/// Find the largest red tile rectangle area
/// ```
/// let vec1: Vec<String> = vec![
///     "7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_a(&vec1), 50);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> Area {
    let redtiles = parse_coordinates(string_list);
    let mut all_sizes = find_all_rects(redtiles);
    all_sizes.sort_by(|a, &b| b.area.cmp(&a.area));
    return all_sizes.first().unwrap().area;
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coords() {
        let vec1: Vec<String> = vec!["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected: Vec<Coord> = vec![
            Coord::new(7, 1),
            Coord::new(11, 1),
            Coord::new(11, 7),
            Coord::new(9, 7),
            Coord::new(9, 5),
            Coord::new(2, 5),
            Coord::new(2, 3),
            Coord::new(7, 3),
        ];
        assert_eq!(parse_coordinates(&vec1), expected);
    }

    #[test]
    fn test_calc_area() {
        let a = Coord::new(2, 5);
        let b = Coord::new(11, 1);
        assert_eq!(calc_area(a, b), 50);
    }

    #[test]
    fn test_find_all_rects() {
        let input: Vec<Coord> = vec![
            Coord::new(7, 1),
            Coord::new(11, 1),
            Coord::new(11, 7),
            Coord::new(9, 7),
            Coord::new(9, 5),
            Coord::new(2, 5),
            Coord::new(2, 3),
            Coord::new(7, 3),
        ];
        let expected = vec![
            PotentialRect {
                u: 0,
                v: 1,
                area: 5,
            },
            PotentialRect {
                u: 0,
                v: 2,
                area: 35,
            },
            PotentialRect {
                u: 0,
                v: 3,
                area: 21,
            },
            PotentialRect {
                u: 0,
                v: 4,
                area: 15,
            },
            PotentialRect {
                u: 0,
                v: 5,
                area: 30,
            },
            PotentialRect {
                u: 0,
                v: 6,
                area: 18,
            },
            PotentialRect {
                u: 0,
                v: 7,
                area: 3,
            },
            PotentialRect {
                u: 1,
                v: 2,
                area: 7,
            },
            PotentialRect {
                u: 1,
                v: 3,
                area: 21,
            },
            PotentialRect {
                u: 1,
                v: 4,
                area: 15,
            },
            PotentialRect {
                u: 1,
                v: 5,
                area: 50,
            },
            PotentialRect {
                u: 1,
                v: 6,
                area: 30,
            },
            PotentialRect {
                u: 1,
                v: 7,
                area: 15,
            },
            PotentialRect {
                u: 2,
                v: 3,
                area: 3,
            },
            PotentialRect {
                u: 2,
                v: 4,
                area: 9,
            },
            PotentialRect {
                u: 2,
                v: 5,
                area: 30,
            },
            PotentialRect {
                u: 2,
                v: 6,
                area: 50,
            },
            PotentialRect {
                u: 2,
                v: 7,
                area: 25,
            },
            PotentialRect {
                u: 3,
                v: 4,
                area: 3,
            },
            PotentialRect {
                u: 3,
                v: 5,
                area: 24,
            },
            PotentialRect {
                u: 3,
                v: 6,
                area: 40,
            },
            PotentialRect {
                u: 3,
                v: 7,
                area: 15,
            },
            PotentialRect {
                u: 4,
                v: 5,
                area: 8,
            },
            PotentialRect {
                u: 4,
                v: 6,
                area: 24,
            },
            PotentialRect {
                u: 4,
                v: 7,
                area: 9,
            },
            PotentialRect {
                u: 5,
                v: 6,
                area: 3,
            },
            PotentialRect {
                u: 5,
                v: 7,
                area: 18,
            },
            PotentialRect {
                u: 6,
                v: 7,
                area: 6,
            },
        ];
        assert_eq!(find_all_rects(input), expected);
    }
}
