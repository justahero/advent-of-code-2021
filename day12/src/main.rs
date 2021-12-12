use std::{cmp::Ordering, rc::Rc};

use itertools::Itertools;

/// A single node in the graph, can be shared by multiple edges
#[derive(Debug, Clone, Hash, PartialEq)]
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
struct Edge {
    pub nodes: [Rc<Node>; 2],
}

impl Edge {
    pub fn new(left: Rc<Node>, right: Rc<Node>) -> Self {
        Self { nodes: [left, right] }
    }
}

struct Graph {
    // pub start_nodes: Vec<u32>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }
}

fn parse_input(input: &str) -> Graph {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    // find all nodes / edges, then link them in the graph
    let edges = lines
        .iter()
        .filter_map(|&line| line.split_once('-'))
        .map(|(left, right)| {
            (Rc::new(Node::new(left.to_string())), Rc::new(Node::new(right.to_string())))
        })
        .map(|(left, right)| Edge::new(left, right))
        .collect_vec();

    Graph::new(edges)
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
        assert_eq!(10, graph.edges.len());
    }
}
