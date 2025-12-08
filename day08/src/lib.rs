extern crate filelib;

pub use filelib::load_no_blanks;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;
use std::{cmp::Ordering, collections::HashSet};

// Use i32 for parsing numbers, because we will be subtracting later, and it makes the math easier.
type Num = i32;
// Sqrt gives a float, so we store distance as a float.
type Dist = f64;
// We have same big numbers at the end, so this can be changed if its too small.
type Size = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: Num,
    y: Num,
    z: Num,
}

impl Coord {
    pub fn euclidean_distance(&self, other: &Coord) -> Dist {
        info!(
            "euclidean_distance multiplication happening {:?} {:?}",
            self, other
        );
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;

        return (dx * dx + dy * dy + dz * dz).sqrt();
    }
}

// A possible connection
#[derive(Debug, Copy, Clone, PartialEq)]
struct Edge {
    u: usize, // Index of node 1
    v: usize, // Index of node 2
    distance: Dist,
}

// Disjoint Set union helper, for Kruskal’s algorithm
// Parent stores "who is your parent"
// size stores the number of connected nodes
#[derive(Debug, Clone, PartialEq, Eq)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<Size>,
}

impl DSU {
    fn new(n: usize) -> Self {
        return DSU {
            parent: (0..n).collect(), // Everyone starts as their own parent
            size: vec![1; n],
        };
    }

    fn find_representative(&mut self, i: usize) -> usize {
        // this is to compress together paths.
        // Instead of having 0 -> 1 -> 2, just have 0 say its parent is 2.
        if self.parent[i] != i {
            self.parent[i] = self.find_representative(self.parent[i]);
        }
        return self.parent[i];
    }

    // Combine two sets, return true if they were different sets
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find_representative(i);
        let root_j = self.find_representative(j);

        if root_i == root_j {
            return false;
        }

        // Merge smaller tree into larger tree to minimize depth
        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }
        return true;
    }
}

fn parse_coords(string_list: &Vec<String>) -> Vec<Coord> {
    return string_list
        .iter()
        .map(|s| {
            let (x_str, yz) = s.split_once(",").expect("Should be three coords");
            let (y_str, z_str) = yz.split_once(",").expect("Should be two coords left");
            let x: Num = x_str.parse().expect("Should fit in Num");
            let y: Num = y_str.parse().expect("Should fit in Num");
            let z: Num = z_str.parse().expect("Should fit in Num");
            return Coord { x: x, y: y, z: z };
        })
        .collect();
}

fn generate_all_edges(coords: &Vec<Coord>) -> Vec<Edge> {
    let mut edges = vec![];
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let dist = coords[i].euclidean_distance(&coords[j]);
            edges.push(Edge {
                u: i,
                v: j,
                distance: dist,
            });
        }
    }

    return edges;
}

fn kruskals_algorithm(coords: &Vec<Coord>, edges: &Vec<Edge>, steps: usize) -> DSU {
    let mut dsu = DSU::new(coords.len());
    let mut connections_made = 0;

    for edge in edges {
        if connections_made >= steps {
            break;
        }

        // Try to connect. union returns true if they weren't already connected.
        // Turns out for part A at least, we don't care if they were already connected.
        dsu.union(edge.u, edge.v);
        info!(
            "Connected {} to {} (dist: {:.2})",
            edge.u, edge.v, edge.distance
        );
        connections_made += 1;
    }

    return dsu;
}

fn find_sizes(coords: &Vec<Coord>, dsu: &mut DSU) -> Vec<Size> {
    // Each unique root is a "tree" in the forest.
    let mut unique_roots = HashSet::new();
    for i in 0..coords.len() {
        unique_roots.insert(dsu.find_representative(i));
    }

    let mut component_sizes: Vec<Size> = unique_roots.iter().map(|&root| dsu.size[root]).collect();
    // set largest to be first
    component_sizes.sort_by(|a, b| b.cmp(a));

    return component_sizes;
}

/// Run Kruskal’s algorithm, up to `steps` steps. In the resulting forest, return the three largest graphs.
/// ```
/// let vec1: Vec<String> = vec![
///     "162,817,812", "57,618,57", "906,360,560", "592,479,940", "352,342,300", "466,668,158",
///     "542,29,236", "431,825,988", "739,650,466", "52,470,668", "216,146,977", "819,987,18",
///     "117,168,530", "805,96,715", "346,949,466", "970,615,88", "941,993,340", "862,61,35",
///     "984,92,344", "425,690,689"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_a(&vec1, 10), 40);
/// ```
pub fn puzzle_a(string_list: &Vec<String>, steps: usize) -> Size {
    info!("Parsing coords");
    let coords = parse_coords(string_list);
    info!("Generating edges");
    let mut edges = generate_all_edges(&coords);

    info!("Sorting edges");
    // f64 doesn't implement Ord, so we use partial_cmp
    edges.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(Ordering::Equal)
    });

    info!("Running Kruskal's");
    let mut dsu = kruskals_algorithm(&coords, &edges, steps);

    info!("Multiplying end values");
    let sizes = find_sizes(&coords, &mut dsu);
    return sizes.into_iter().take(3).product();
}

fn kruskals_algorithm_b(coords: &Vec<Coord>, edges: &Vec<Edge>) -> (Coord, Coord) {
    let mut dsu = DSU::new(coords.len());
    let mut from: Coord = Coord { x: 0, y: 0, z: 0 };
    let mut to: Coord = Coord { x: 0, y: 0, z: 0 };

    for edge in edges {
        // Try to connect. union returns true if they weren't already connected.
        // Turns out for part A at least, we don't care if they were already connected.
        if dsu.union(edge.u, edge.v) {
            info!(
                "Connected {} to {} (dist: {:.2})",
                edge.u, edge.v, edge.distance
            );
            from = coords[edge.u];
            to = coords[edge.v];
        }
    }

    return (from, to);
}

/// Run Kruskal until done, get the x coordinates of the last two connected boxes and provide those.
/// ```
/// let vec1: Vec<String> = vec![
///     "162,817,812", "57,618,57", "906,360,560", "592,479,940", "352,342,300", "466,668,158",
///     "542,29,236", "431,825,988", "739,650,466", "52,470,668", "216,146,977", "819,987,18",
///     "117,168,530", "805,96,715", "346,949,466", "970,615,88", "941,993,340", "862,61,35",
///     "984,92,344", "425,690,689"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day08::puzzle_b(&vec1), 25272);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u64 {
    info!("Parsing coords");
    let coords = parse_coords(string_list);
    info!("Generating edges");
    let mut edges = generate_all_edges(&coords);

    info!("Sorting edges");
    // f64 doesn't implement Ord, so we use partial_cmp
    edges.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(Ordering::Equal)
    });

    info!("Running Kruskal's");
    let (from, to) = kruskals_algorithm_b(&coords, &edges);
    let result: u64 = (from.x * to.x) as u64;
    return result;
}
