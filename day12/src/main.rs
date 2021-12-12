use std::{cmp::Ordering, collections::{HashMap, VecDeque}};

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

    pub fn small(&self) -> bool {
        self.value.chars().nth(0).unwrap().is_lowercase()
    }

    pub fn big(&self) -> bool {
        !self.small()
    }

    pub fn is_start(&self) -> bool {
        self.value.cmp(&String::from("start")) == Ordering::Equal
    }

    pub fn is_end(&self) -> bool {
        self.value.cmp(&String::from("end")) == Ordering::Equal
    }
}

#[derive(Debug, Clone)]
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

    /// Traverse all paths, returns the number of paths found
    pub fn count_all_paths(&self) -> usize {
        let mut count = 0_usize;
        let mut visited: Vec<&Node> = Vec::new();
        let mut nodes: VecDeque<&Node> = VecDeque::new();

        let start_node = self
            .map
            .keys()
            .find(|&key| key.is_start()).expect("No start node");

        nodes.push_back(start_node);
        visited.push(start_node);

        while let Some(node) = nodes.pop_front() {
            // If at the end, finish the path
            if node.is_end() {
                count += 1;
                break;
            }

            // get all next edges
            let edges = &self.map[node];
        }

        count
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

        // ignore all end paths, we cannot move from there
        if !left.is_end() {
            graph.add_edge(left, right);
        }
        graph
    });

    graph
}

fn main() {
    let graph = parse_input(include_str!("input.txt"));
    let count = graph.count_all_paths();
    dbg!(count);
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    // dc, end, start, HN, kj, LN, sa
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
        assert_eq!(19, graph.count_all_paths());
    }
}
