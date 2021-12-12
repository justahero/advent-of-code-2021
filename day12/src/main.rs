use std::{collections::HashMap, fmt::{Debug, Display}};

use itertools::Itertools;

/// A single node in the graph, can be shared by multiple edges
#[derive(Clone, Hash, PartialEq, Eq)]
struct Node(String);

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl Node {
    pub fn new(value: String) -> Self {
        Node(value)
    }

    pub fn small(&self) -> bool {
        self.0.chars().nth(0).unwrap().is_lowercase()
    }

    pub fn big(&self) -> bool {
        !self.small()
    }

    pub fn is_start(&self) -> bool {
        &self.0 == "start"
    }

    pub fn is_end(&self) -> bool {
        &self.0 == "end"
    }
}

/// Returns true if all small cave nodes in the given list are unique
fn is_unique(list: &[Node]) -> bool {
    let list = list.iter().filter(|node| node.small()).collect_vec();
    list.len() == list.iter().unique().count()
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

    pub fn count_all_paths(&self, visit_twice: bool) -> usize {
        Self::find_paths(vec![Node::new("start".to_string())], &self.map, visit_twice).len()
    }

    /// Traverse all paths via DFS, return the list of paths found
    pub fn find_paths(visited: Vec<Node>, edges: &HashMap<Node, Vec<Node>>, visit_twice: bool) -> Vec<Vec<Node>> {
        let last_node = visited.last().expect("No last node found");
        if last_node.is_end() {
            vec![visited]
        } else {
            let mut results = Vec::new();

            let can_visit_twice = visit_twice && is_unique(&visited);
            for next_node in edges.get(last_node).expect("No edges found for node") {
                if !visited.contains(next_node) || next_node.big() || can_visit_twice {
                    // copy current path for next step
                    let mut next_visited = visited.clone();
                    next_visited.push(next_node.clone());
                    results.append(&mut Self::find_paths(next_visited, edges, visit_twice));
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
        graph.add_edges(Node::new(left.to_string()), Node::new(right.to_string()));
        graph
    });

    graph
}

fn main() {
    let graph = parse_input(include_str!("input.txt"));
    let count = graph.count_all_paths(false);
    dbg!(count);

    let count = graph.count_all_paths(true);
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
        assert_eq!(10, graph.count_all_paths(false));
        assert_eq!(36, graph.count_all_paths(true));
    }

    #[test]
    fn traverses_and_counts_all_paths() {
        let graph = parse_input(INPUT);
        assert_eq!(19, graph.count_all_paths(false));
        assert_eq!(103, graph.count_all_paths(true));
    }
}
