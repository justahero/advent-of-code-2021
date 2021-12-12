use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

/// A single node in the graph, can be shared by multiple edges
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    pub value: String,
}

impl Node {
    pub fn new(value: String) -> Self {
        Node { value }
    }

    pub fn is_start(&self) -> bool {
        self.value.cmp(&String::from("start")) == Ordering::Equal
    }

    pub fn is_end(&self) -> bool {
        self.value.cmp(&String::from("end")) == Ordering::Equal
    }
}

#[derive(Debug)]
struct Graph {
    pub map: HashMap<Node, Vec<Node>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Adds a new edge to the graph
    pub fn add_edge(&mut self, left: Node, right: Node) {
        self.map
            .entry(left.clone())
            .or_insert(Vec::new())
            .push(right.clone());
        self.map
            .entry(right.clone())
            .or_insert(Vec::new())
            .push(left.clone());
    }

    /// Count all edges
    pub fn count_edges(&self) -> usize {
        self.map
            .iter()
            .fold(0_usize, |count, (_key, edges)| count + edges.len())
    }
}

fn parse_input(input: &str) -> Graph {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    // parse all nodes
    let graph = lines.iter().fold(Graph::new(), |mut graph, &line| {
        let (left, right) = line.split_once('-').expect("Failed to split");
        let left = Node::new(left.to_string());
        let right = Node::new(right.to_string());
        graph.add_edge(left, right);
        graph
    });

    graph
}

fn main() {
    let graph = parse_input(include_str!("input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "#;

    #[test]
    fn build_graph() {
        let graph = parse_input(INPUT);
        assert_eq!(20, graph.count_edges());
    }

    #[test]
    fn traverses_and_counts_all_paths() {
        let graph = parse_input(INPUT);
    }
}
