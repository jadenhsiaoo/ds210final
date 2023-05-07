use petgraph::Graph;
use petgraph::visit::{Bfs};
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::prelude::*;
use petgraph::Undirected;
use std::collections::{HashMap};
use rand::seq::SliceRandom;

//Function to read data file and creating graph
pub fn read_data(filename: &str) -> Graph<u32, (), Undirected> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut graph = Graph::<u32, (), Undirected>::new_undirected();
    let mut nodes = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let ids: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let a = *nodes.entry(ids[0]).or_insert_with(|| graph.add_node(ids[0]));
        let b = *nodes.entry(ids[1]).or_insert_with(|| graph.add_node(ids[1]));

        graph.add_edge(a, b, ());
    }

    graph
}

//Function calculate average shortest path length
pub fn calculate_average_shortest_path_length(graph: &Graph<u32, (), Undirected>, num_samples: usize) -> f64 {
    let mut total_distance = 0;
    let mut total_pairs = 0;
    let mut rng = rand::thread_rng();
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();

    for _ in 0..num_samples {
        let nodes_pair = node_indices.choose_multiple(&mut rng, 2).cloned().collect::<Vec<_>>();
        let (node_a, node_b) = (nodes_pair[0], nodes_pair[1]);
        let mut bfs = Bfs::new(&graph, node_a);

        let mut depth_map = HashMap::new();
        depth_map.insert(node_a, 0);

        while let Some(node) = bfs.next(&graph) {
            let depth = depth_map[&node] + 1;
            for neighbor in graph.neighbors(node) {
                if !depth_map.contains_key(&neighbor) {
                    depth_map.insert(neighbor, depth);
                }
            }

            if node == node_b {
                total_distance += depth_map[&node_b];
                total_pairs += 1;
                break;
            }
        }
    }

    if total_pairs > 0 {
        total_distance as f64 / total_pairs as f64
    } else {
        0.0 
    }
}
#[cfg(test)]
pub mod tests {
    use super::super::functions::*;

    #[test]
    //Testing the graph
    fn test_graph_construction() {
        let graph = read_data("facebook_combined.txt");
        assert_eq!(graph.node_count(), 4039);
        assert_eq!(graph.edge_count(), 88234);
    }

    #[test]
    //Testing the function to calculate average shortest length
    fn test_average_shortest_path_length() {
        let graph = Graph::<u32, (), Undirected>::from_edges(&[
            (1, 2, ()),
            (2, 3, ())
        ]);

        let average_distance = calculate_average_shortest_path_length(&graph, 10);
        assert!(
            (average_distance - 1.5).abs() < 0.3,
            "The average shortest path length should be close to 1.5, but was {}",
            average_distance
        );
    }
}