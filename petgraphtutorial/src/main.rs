use std::fmt;

// Uses Adjaceny list, no type restrictions on edge / node type
use petgraph::graph::Graph;
// Like Graph, but indexes remain same when nodes/edges deleted.
//use petgraph::stable_graph::StableGraph;

// Backed with a map, using Nodes as keys, requiring Copy, Eq, Ord, and Hash
use petgraph::graphmap::UnGraphMap;

// CSR is a Compressed Sparse Row (A sparse matrix), more restricted API but faster.
use petgraph::csr::Csr;

// Provide a DFS interface
use petgraph::visit::Dfs;

// Provide a BFS interface
use petgraph::visit::Bfs;

// For visualiztion purposes
use petgraph::dot::Dot;

// For algorithms
use petgraph::algo;

// GraphMap example Node
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct City {
    population: u32,
    cars: u32,
}

// For visualizing Graph node, needs Fmt

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the population to the output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.population)
    }
}

fn basic_graph() {
    let mut graph = Graph::<(), ()>::new(); // directed and unlabeled
    graph.extend_with_edges(&[(0, 1)]); // 2 nodes, 1 node between them
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}

fn labeled_graph() {
    // Add some labels
    let mut graph = Graph::new(); // directed and unlabeled
    let origin = graph.add_node("Denver");
    let dest_1 = graph.add_node("San Diego");
    let dest_2 = graph.add_node("New york");
    let cost_1 = graph.add_edge(origin, dest_1, 250);
    let cost_2 = graph.add_edge(origin, dest_2, 1099);

    assert_eq!(graph.node_weight(origin).unwrap(), &"Denver");
    assert_eq!(graph[dest_1], "San Diego");
    assert_eq!(graph.edge_weight(cost_1).unwrap(), &250);
    assert_eq!(graph.edge_weight(cost_2).unwrap(), &1099);
}

fn undirected_graph() {
    let mut graph = Graph::new_undirected();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");
    let cost_1 = graph.add_edge(origin, destination_1, 250);
    let cost_2 = graph.add_edge(origin, destination_2, 1099);

    assert_eq!(graph.edge_weight(cost_1).unwrap(), &250);
    assert_eq!(graph.edge_weight(cost_2).unwrap(), &1099);
}

fn visualize_graph() {
    let mut graph = Graph::<&str, u32>::new();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");

    graph.extend_with_edges(&[(origin, destination_1, 250), (origin, destination_2, 1099)]);

    println!("Copy this into https://viz-js.com/");
    println!("{}", Dot::new(&graph));
}

fn graph_map() {
    let mut graph = UnGraphMap::<_, u32>::new();
    let bedford_falls = City {
        population: 1023,
        cars: 24,
    };
    let tinsel_town = City {
        population: 102479,
        cars: 1231441,
    };

    graph.add_node(&bedford_falls);
    graph.add_node(&tinsel_town);
    graph.add_edge(&bedford_falls, &tinsel_town, 200);

    assert!(graph.contains_node(&bedford_falls));
    assert!(graph.contains_node(&tinsel_town));
    assert!(graph.contains_edge(&bedford_falls, &tinsel_town));
    assert!(graph.contains_edge(&tinsel_town, &bedford_falls));

    println!("Copy this into https://viz-js.com/");
    println!("{}", Dot::new(&graph));
}

fn csr_graph() {
    let mut graph = Csr::<_, u32>::new(); // directed
    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let _ = graph.add_edge(a, b, 20);

    assert!(graph.contains_edge(a, b));
    assert!(!graph.contains_edge(b, a));
    println!("Copy this into https://viz-js.com/");
    println!("{}", Dot::new(&graph));
}

fn dfs_example() {
    println!("DFS _example");
    let mut graph = Graph::<(), (), petgraph::Undirected>::new_undirected();

    // 0(1)(2)3
    graph.extend_with_edges(&[(0, 1), (0, 2), (0, 3)]);

    // Run one DFS from every node
    for start in graph.node_indices() {
        // This is pre-ordered (So we visit A node before its neighbors)
        // There is also DfsPostOrder if you want to visit a node's neighbors before it.
        let mut dfs = Dfs::new(&graph, start);

        print!("[cur: {}] ", start.index());

        print!(" next:");
        while let Some(visited) = dfs.next(&graph) {
            print!(" {}", visited.index());
        }

        println!();
    }
}

fn bfs_example() {
    println!("BFS _example");
    let mut graph = Graph::<(), (), petgraph::Undirected>::new_undirected();

    // 0(1)(2)34
    graph.extend_with_edges(&[(0, 1), (0, 2), (0, 3), (3, 4)]);

    // Run one BFS from every node
    for start in graph.node_indices() {
        let mut bfs = Bfs::new(&graph, start);

        print!("[cur: {}] ", start.index());

        print!(" next:");
        while let Some(visited) = bfs.next(&graph) {
            print!(" {}", visited.index());
        }

        println!();
    }
}

fn isomorphic_example() {
    let mut g1 = Graph::<(), (), petgraph::Undirected>::new_undirected();
    let mut g2 = Graph::<(), (), petgraph::Undirected>::new_undirected();

    g1.extend_with_edges(&[(0, 1), (0, 2), (0, 3)]);

    g2.extend_with_edges(&[(0, 1), (0, 2), (0, 3)]);

    assert_eq!(algo::is_isomorphic(&g1, &g2), true);

    g1.extend_with_edges(&[(3, 4)]);

    assert_eq!(algo::is_isomorphic(&g1, &g2), false);

    // is_isomorphic_matching available if you need to check node and edge conditions.
}

fn dijkstra_example() {
    println!("\nDijkstra _example");
    let mut graph = Graph::<(), ()>::new();

    graph.extend_with_edges(&[(0, 1), (0, 2), (0, 3), (3, 4)]);

    for start in graph.node_indices() {
        println!("--- {:?} ---", start.index());
        println!("{:?}", algo::dijkstra(&graph, start, None, |_| 1));
    }
}

fn main() {
    basic_graph();
    labeled_graph();
    undirected_graph();
    visualize_graph();

    // Alternative graph types
    println!("\n----\n");
    graph_map();
    println!("\n----\n");
    csr_graph();

    // Algorithms
    println!("\n----\n");
    dfs_example();
    bfs_example();
    isomorphic_example();
    dijkstra_example();

    // Other algos available:
    // astar - Dijkstra + heuristics, a pathfinding algorithm
    // bellman_ford - Dijkstra with negative edge weights

    // There are more, but I haven't ever done those before, so I should try those myself!
}
