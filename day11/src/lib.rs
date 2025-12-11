extern crate filelib;

pub use filelib::load_no_blanks;
use std::collections::HashMap;

#[cfg(not(test))]
use log::info;

#[cfg(test)]
use std::println as info;

type DeviceId = String;
type AdjacencyGraph = HashMap<DeviceId, Vec<DeviceId>>;

fn parse(string_list: &Vec<String>) -> AdjacencyGraph {
    let mut graph = AdjacencyGraph::new();
    let out = "out";
    graph.insert(out.to_string(), vec![]); // should always be empty

    for line in string_list {
        let (source_name, rest) = line.split_once(": ").expect("Line should have a :");
        let outputs = rest.split(" ").map(|s| s.to_string()).collect();
        graph.insert(source_name.to_string(), outputs);
    }

    return graph;
}

fn dfs_recursive(
    graph: &AdjacencyGraph,
    current: DeviceId,
    memo: &mut HashMap<DeviceId, usize>,
) -> usize {
    // Memoization immediate return
    if let Some(&count) = memo.get(&current) {
        return count;
    }

    let outputs = graph.get(&current).expect("all should be in there");
    if outputs.is_empty() {
        // The line ended from here
        return 1;
    }

    let mut total_paths = 0;
    for next in outputs {
        total_paths += dfs_recursive(graph, next.clone(), memo);
    }

    memo.insert(current, total_paths);
    return total_paths;
}

/// Find every path from you to out
/// ```
/// let vec1: Vec<String> = vec![
///   "aaa: you hhh", "you: bbb ccc", "bbb: ddd eee", "ccc: ddd eee fff",
///   "ddd: ggg", "eee: out", "fff: out", "ggg: out", "hhh: ccc fff iii", "iii: out"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day11::puzzle_a(&vec1), 5);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> usize {
    let graph = parse(string_list);
    let start = "you".to_string();
    let mut memo: HashMap<DeviceId, usize> = HashMap::new();
    return dfs_recursive(&graph, start, &mut memo);
}

fn dfs_b_recursive(
    graph: &AdjacencyGraph,
    current: DeviceId,
    memo: &mut HashMap<(DeviceId, usize), usize>,
    path: &Vec<DeviceId>,
    required: &Vec<DeviceId>,
) -> usize {
    let mut new_path = path.clone();
    new_path.push(current.clone());
    let required_satisfied = new_path.iter().filter(|x| required.contains(x)).count();
    // Memoization immediate return
    if let Some(&count) = memo.get(&(current.clone(), required_satisfied)) {
        return count;
    }

    let outputs = graph.get(&current).expect("all should be in there");
    if outputs.is_empty() {
        if required_satisfied == required.len() {
            info!("{:?} path found", new_path);
            return 1;
        }
        info!("{:?} path pruned", new_path);
        return 0;
    }

    let mut total_paths = 0;
    for next in outputs {
        total_paths += dfs_b_recursive(graph, next.clone(), memo, &new_path, required);
    }

    memo.insert((current, required_satisfied), total_paths);
    return total_paths;
}

/// Find paths from `svr` to `out` that contain `dac` and `fft`
/// ```
/// let vec1: Vec<String> = vec![
///     "svr: aaa bbb", "aaa: fft", "fft: ccc", "bbb: tty", "tty: ccc",
///     "ccc: ddd eee", "ddd: hub", "hub: fff", "eee: dac", "dac: fff",
///     "fff: ggg hhh", "ggg: out", "hhh: out"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day11::puzzle_b(&vec1), 2);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> usize {
    let graph = parse(string_list);
    let start = "svr".to_string();
    let mut memo = HashMap::new();
    let path = vec![];
    let required = vec!["dac".to_string(), "fft".to_string()];
    return dfs_b_recursive(&graph, start, &mut memo, &path, &required);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coords() {
        let vec1: Vec<String> = vec![
            "aaa: you hhh",
            "you: ccc",
            "ccc: hhh bbb",
            "bbb: out",
            "hhh: out",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let mut expected = HashMap::new();
        expected.insert(
            "aaa".to_string(),
            vec!["you", "hhh"].iter().map(|s| s.to_string()).collect(),
        );
        expected.insert(
            "you".to_string(),
            vec!["ccc"].iter().map(|s| s.to_string()).collect(),
        );
        expected.insert(
            "ccc".to_string(),
            vec!["hhh", "bbb"].iter().map(|s| s.to_string()).collect(),
        );
        expected.insert(
            "bbb".to_string(),
            vec!["out"].iter().map(|s| s.to_string()).collect(),
        );
        expected.insert(
            "hhh".to_string(),
            vec!["out"].iter().map(|s| s.to_string()).collect(),
        );
        expected.insert("out".to_string(), vec![]);
        assert_eq!(parse(&vec1), expected);
    }

    #[test]
    fn test_part_b() {
        let vec1: Vec<String> = vec![
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let expected = 2;
        assert_eq!(puzzle_b(&vec1), expected);
    }
}
