use std::{cmp::Ordering, collections::HashMap, fmt::{Debug, Display}};

use itertools::Itertools;

/// A single node in the graph, can be shared by multiple edges
#[derive(Clone, Hash, PartialEq, Eq)]
struct Node {
    pub value: String,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.value)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.value)
    }
}

impl Node {
    pub fn new(value: &str) -> Self {
        Node { value: value.to_string() }
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
    pub fn add_edges(&mut self, left: Node, right: Node) {
        for (x, y) in [(&left, &right), (&right, &left)] {
            // ignore paths that start with "end" or end with "start"
            if !x.is_end() && !y.is_start() {
                self.map
                    .entry(x.clone())
                    .or_insert(Vec::new())
                    .push(y.clone());
            }
        }
    }

    pub fn count_all_paths(&self) -> usize {
        Self::find_paths(vec![Node::new("start")], &self.map).len()
    }

    /// Traverse all paths via DFS, return the list of paths found
    pub fn find_paths(visited: Vec<Node>, edges: &HashMap<Node, Vec<Node>>) -> Vec<Vec<Node>> {
        let last_node = visited.last().expect("No last node found");
        if last_node.is_end() {
            vec![visited]
        } else {
            let mut results: Vec<Vec<Node>> = Vec::new();

            for next_node in edges.get(last_node).unwrap() {
                if !visited.contains(next_node) || next_node.big() {
                    // copy current path for next step
                    let mut next_visited = visited.clone();
                    next_visited.push(next_node.clone());
                    results.append(&mut Self::find_paths(next_visited, edges));
                }
            }

            results
        }
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
        let left = Node::new(left);
        let right = Node::new(right);

        // ignore all end paths, we cannot move from there
        graph.add_edges(left, right);
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
    fn traverse_minimal_graph() {
        let input = r#"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "#;
        let graph = parse_input(input);
        assert_eq!(10, graph.count_all_paths());
    }

    #[test]
    fn traverses_and_counts_all_paths() {
        let graph = parse_input(INPUT);
        assert_eq!(19, graph.count_all_paths());
    }
}
