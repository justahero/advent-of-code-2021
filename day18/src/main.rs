use anyhow::anyhow;
use std::fmt::Display;

// Simple grammar to parse snailfish pairs
peg::parser! {
    grammar line_parser() for str {
        rule literal(depth: u32) -> Node
            = l:$(['0'..='9']+) { Node::leaf(l.parse::<u8>().unwrap(), depth) }

        rule comma()
            = ","

        rule open()
            = "["

        rule close()
            = "]"

        pub(crate) rule pair(depth: u32) -> Node
            = open() l:(literal((depth + 1)) / pair((depth + 1))) comma() r:(literal((depth + 1)) / pair((depth + 1))) close() {
                Node::branch(l, r, depth)
            }
    }
}

/// A binary tree representation?
#[derive(Debug)]
enum Node {
    Leaf {
        value: u8,
        depth: u32,
    },
    Branch {
        left: Box<Node>,
        right: Box<Node>,
        depth: u32,
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Node::Leaf { value, .. } => format!("{}", value),
            Node::Branch { left, right, .. } => {
                format!("[{},{}]", left.to_string(), right.to_string())
            }
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Node {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        line_parser::pair(line, 0).map_err(|e| anyhow!("Failed to parse line '{}': {}", line, e))
    }
}

impl Node {
    pub fn leaf(value: u8, depth: u32) -> Self {
        Node::Leaf {
            value,
            depth,
        }
    }

    pub fn branch(left: Node, right: Node, depth: u32) -> Self {
        Node::Branch {
            left: Box::new(left),
            right: Box::new(right),
            depth,
        }
    }

    /// Checks if a Node in this tree can explodes.
    /// In order to explode one pair needs to be at least in a certain depth.
    /// In case it explodeds, the values of the pair are returned in an Option and the pair is replaced.
    pub fn explode(&mut self) -> Option<(u8, u8)> {
        if let Node::Branch { left, right, depth } = self {
            if *depth >= 4 {
                let a = match **left {
                    Node::Leaf { value, ..} => value,
                    _ => panic!("Not a leaf."),
                };
                let b = match **right {
                    Node::Leaf { value, ..} => value,
                    _ => panic!("Not a leaf."),
                };
                *self = Node::leaf(0, *depth);
                return Some((a, b));
            } else {
                if let Some((a, b)) = left.explode() {
                    right.merge(true, b);
                    return Some((a, 0));
                }
                if let Some((a, b)) = right.explode() {
                    left.merge(false, a);
                    return Some((0, b));
                }
            }
        }

        None
    }

    /// Merges the exploded inner pair into the current Node or left / right node
    fn merge(&mut self, from_left: bool, value: u8) {
        match self {
            Node::Leaf { value: current, .. } => *current += value,
            Node::Branch { left, right, .. } => match from_left {
                true => left.merge(from_left, value),
                false => right.merge(from_left, value),
            },
        }
    }

    /// Checks if a Node needs to be split
    pub fn split(&mut self) -> Option<()> {
        match self {
            Node::Leaf { value, depth } => {
                if *value >= 10 {
                    // split into a new Node
                    let left = Node::leaf((*value as f32 / 2.0).floor() as u8, *depth + 1);
                    let right = Node::leaf((*value as f32 / 2.0).ceil() as u8, * depth + 1);
                    *self = Node::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                        depth: *depth,
                    };
                    return Some(());
                }
            }
            Node::Branch { left, right, depth } => {
                left.split()?;
                right.split()?;
            }
        }
        None
    }
}

struct Table {
    pub pairs: Vec<Node>,
}

impl Table {
    pub fn new(pairs: Vec<Node>) -> Self {
        Self { pairs }
    }

    pub fn sum(&self) -> Node {
        Node::leaf(1, 0)
    }
}

fn parse_input(input: &str) -> anyhow::Result<Table> {
    let pairs = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(Node::try_from)
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(Table::new(pairs))
}

fn main() -> anyhow::Result<()> {
    let pairs = parse_input(include_str!("input.txt"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, Node};

    #[test]
    fn can_parse_pairs() -> anyhow::Result<()> {
        assert!(Node::try_from("[1,2]").is_ok());
        assert!(Node::try_from("[[1,2],3]").is_ok());
        assert!(Node::try_from("[9,[8,7]]").is_ok());
        assert!(Node::try_from("[[1,9],[8,5]]").is_ok());
        assert!(Node::try_from("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").is_ok());
        assert!(Node::try_from("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]").is_ok());
        assert!(
            Node::try_from("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").is_ok()
        );
        Ok(())
    }

    #[test]
    fn calculate_example_sum() {
        let input = r#"
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        "#;
        let table = parse_input(input).expect("Failed to parse input.");
        assert_eq!(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            table.sum().to_string()
        );
    }

    #[test]
    fn test_exploding_examples() -> anyhow::Result<()> {
        let examples = vec![
            "[[[[[9,8],1],2],3],4]",
            "[7,[6,[5,[4,[3,2]]]]]",
            "[[6,[5,[4,[3,2]]]],1]",
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ];
        let solutions = vec![
            "[[[[0,9],2],3],4]",
            "[7,[6,[5,[7,0]]]]",
            "[[6,[5,[7,0]]],3]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ];

        for (&input, &expected) in examples.iter().zip(solutions.iter()) {
            let mut node = Node::try_from(input)?;
            assert!(node.explode().is_some());
            assert_eq!(expected, &node.to_string());
        }
        Ok(())
    }
}
