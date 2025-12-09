extern crate filelib;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

pub use filelib::load_no_blanks;
use gridlib::GridCoordinateInf;

type Num = i64;
type Coord = GridCoordinateInf<Num>;

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

fn calc_area(a: Coord, b: Coord) -> Num {
    let dx = ((a.x - b.x).abs() + 1) as Num; // need to add 1 as the border is counted
    let dy = ((a.y - b.y).abs() + 1) as Num; // need to add 1 as the border is counted
    return (dx * dy).try_into().expect("Should fit in Num");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PotentialRect {
    u: usize,
    v: usize,
    area: Num,
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
pub fn puzzle_a(string_list: &Vec<String>) -> Num {
    let redtiles = parse_coordinates(string_list);
    let mut all_sizes = find_all_rects(redtiles);
    all_sizes.sort_by(|a, &b| b.area.cmp(&a.area));
    return all_sizes.first().unwrap().area;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub coord: Coord,
    pub width: Num,
    pub height: Num,
    pub area: Num,
}

/// Helper struct for internal logic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    p1: Coord,
    p2: Coord,
}

// Raycast point is in polygon, need to use floats here.
fn is_point_in_polygon(p: (f64, f64), edges: &Vec<Edge>) -> bool {
    let (px, py) = p;
    let mut inside = false;

    for edge in edges {
        let (p1x, p1y) = (edge.p1.x as f64, edge.p1.y as f64);
        let (p2x, p2y) = (edge.p2.x as f64, edge.p2.y as f64);

        // Check if ray crosses the edge's Y-range
        if (p1y > py) != (p2y > py) {
            // Calculate X intersection of the edge with the horizontal ray at py
            let intersect_x = p1x + (py - p1y) * (p2x - p1x) / (p2y - p1y);
            if px < intersect_x {
                inside = !inside;
            }
        }
    }
    return inside;
}

fn edge_crosses_rect_interior(
    edge: &Edge,
    r_min_x: Num,
    r_max_x: Num,
    r_min_y: Num,
    r_max_y: Num,
) -> bool {
    // Normalize Coordinates
    let e_min_x = edge.p1.x.min(edge.p2.x);
    let e_max_x = edge.p1.x.max(edge.p2.x);
    let e_min_y = edge.p1.y.min(edge.p2.y);
    let e_max_y = edge.p1.y.max(edge.p2.y);

    // If the edge bounding box doesn't overlap the rect, safe.
    if e_max_x <= r_min_x || e_min_x >= r_max_x || e_max_y <= r_min_y || e_min_y >= r_max_y {
        return false;
    }

    // An edge needs to be trictly inside the bounds for us to have an issue.
    // Since this is a Rectilinear polygon, edges are either Vertical or Horizontal.
    if edge.p1.x == edge.p2.x {
        // Vertical, x must be between left and right
        if edge.p1.x > r_min_x && edge.p1.x < r_max_x {
            // AND its Y range must partially overlap the Rect's Y range strictly
            let overlap_min = e_min_y.max(r_min_y);
            let overlap_max = e_max_y.min(r_max_y);
            if overlap_min < overlap_max {
                return true;
            }
        }
    } else {
        // Horizontal, y must be between top and bottom
        if edge.p1.y > r_min_y && edge.p1.y < r_max_y {
            // AND its X range must partially overlap the Rect's X range strictly
            let overlap_min = e_min_x.max(r_min_x);
            let overlap_max = e_max_x.min(r_max_x);
            if overlap_min < overlap_max {
                return true;
            }
        }
    }
    return false;
}

// Generate the edge objects we will need
fn make_edges(vertices: &Vec<Coord>) -> Vec<Edge> {
    let mut edges = vec![];
    for i in 0..vertices.len() {
        edges.push(Edge {
            p1: vertices[i],
            p2: vertices[(i + 1) % vertices.len()],
        });
    }
    return edges;
}

// brute force check each pair.
fn check_each_vertex_pair(vertices: &Vec<Coord>, edges: &Vec<Edge>) -> Rect {
    let mut best_rect = Rect {
        coord: Coord::new(0, 0),
        width: 0,
        height: 0,
        area: 0,
    };

    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[j];

            // Calculate Bounds
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            let width = (max_x - min_x) + 1;
            let height = (max_y - min_y) + 1;
            let area = width * height;

            if area <= best_rect.area {
                continue;
            }

            // Check if any polygon edge cuts through the rectangle
            let mut crossed = false;
            for edge in edges {
                if edge_crosses_rect_interior(edge, min_x, max_x, min_y, max_y) {
                    crossed = true;
                    break;
                }
            }
            if crossed {
                continue;
            }

            // Check if the rectangle is actually inside (vs floating in a hole)
            // Currently checking center, but I honestly forgot why I did this because its 12:30 AM
            // I feel like its because we haven't crossed lines it a valid choice but I would have to draw it out.
            let center_x = (min_x as f64 + max_x as f64) / 2.0;
            let center_y = (min_y as f64 + max_y as f64) / 2.0;

            if !is_point_in_polygon((center_x, center_y), &edges) {
                continue;
            }

            // If we are here, it's valid!
            best_rect = Rect {
                coord: Coord::new(min_x, min_y),
                width,
                height,
                area,
            };
        }
    }
    return best_rect;
}

fn find_largest_inscribed_rect(vertices: &Vec<Coord>) -> Rect {
    if vertices.len() < 3 {
        panic!("Need at least 3 vertices to be a polygon");
    }

    let edges = make_edges(vertices);
    return check_each_vertex_pair(vertices, &edges);
}

/// Find the largest rectangle within a polygon.
/// By definition it should be convex.
/// Notably in data, we never have both x and y switch at once. This avoids diagonal issues.
/// Technical term for this Rectilinear polygon
/// ```
/// let vec1: Vec<String> = vec![
///     "7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day09::puzzle_b(&vec1), 24);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> Num {
    let vertices = parse_coordinates(string_list);
    let rect = find_largest_inscribed_rect(&vertices);
    return rect.area;
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
    fn test_find_largest_inscribed_rect() {
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
        let expected = Rect {
            coord: Coord::new(2, 3),
            width: 8,
            height: 3,
            area: 24,
        };
        assert_eq!(find_largest_inscribed_rect(&input), expected);
    }

    #[test]
    fn test_find_largest_inscribed_rect_c_shape() {
        /*
             0123456789
            0.#.....#.. 1 2
            1..........
            2....#..#.. 4 3
            3..........
            4....#..#.. 5 6
            5..........
            6.#.....#.. 8 7
        */
        let input: Vec<Coord> = vec![
            Coord::new(1, 0),
            Coord::new(7, 0),
            Coord::new(7, 2),
            Coord::new(4, 2),
            Coord::new(4, 4),
            Coord::new(7, 4),
            Coord::new(7, 6),
            Coord::new(1, 6),
        ];
        let expected = Rect {
            coord: Coord::new(1, 0),
            width: 7,
            height: 3,
            area: 21,
        };
        assert_eq!(find_largest_inscribed_rect(&input), expected);
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
