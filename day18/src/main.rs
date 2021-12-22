use anyhow::anyhow;
use itertools::Itertools;
use std::{fmt::Display, ops::Add};

// Simple grammar to parse snailfish pairs
peg::parser! {
    grammar line_parser() for str {
        rule literal() -> Node
            = l:$(['0'..='9']+) { Node::leaf(l.parse::<u8>().unwrap()) }

        rule comma()
            = ","

        rule open()
            = "["

        rule close()
            = "]"

        pub(crate) rule pair() -> Node
            = open() l:(literal() / pair()) comma() r:(literal() / pair()) close() {
                Node::branch(l, r)
            }
    }
}

/// A binary tree representation?
#[derive(Debug, Clone)]
enum Node {
    Leaf { value: u8 },
    Branch { left: Box<Node>, right: Box<Node> },
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
        line_parser::pair(line).map_err(|e| anyhow!("Failed to parse line '{}': {}", line, e))
    }
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        Node::Branch {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl Node {
    pub fn leaf(value: u8) -> Self {
        Node::Leaf { value }
    }

    pub fn branch(left: Node, right: Node) -> Self {
        Node::Branch {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Calculates the magnitude recursively
    pub fn magnitude(&self) -> u32 {
        match self {
            Node::Leaf { value } => *value as u32,
            Node::Branch { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    /// Returns `true` if this Node reduces further, otherwise `false`
    pub fn reduce(&mut self) -> bool {
        self.explode() || self.split().is_some()
    }

    /// Returns true when there is an exploding pair, updates the binary tree accordingly
    pub fn explode(&mut self) -> bool {
        self.do_explode(0).is_some()
    }

    /// Checks if a Node in this tree can explode.
    /// In order to explode one pair needs to be at least in a certain depth.
    /// In case it explodeds, the values of the pair are returned in an Option and merged up..
    fn do_explode(&mut self, depth: u32) -> Option<(u8, u8)> {
        if let Node::Branch { left, right } = self {
            // println!("do_explode left: {:?}, right: {:?} depth: {}", left, right, depth);
            if depth >= 4 {
                let a = match **left {
                    Node::Leaf { value, .. } => value,
                    _ => panic!("Not a leaf."),
                };
                let b = match **right {
                    Node::Leaf { value, .. } => value,
                    _ => panic!("Not a leaf."),
                };
                *self = Node::leaf(0);
                return Some((a, b));
            } else {
                if let Some((a, b)) = left.do_explode(depth + 1) {
                    right.merge(true, b);
                    return Some((a, 0));
                }
                if let Some((a, b)) = right.do_explode(depth + 1) {
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
            Node::Leaf { value } => {
                if *value >= 10 {
                    // split into a new Node
                    let left = Node::leaf((*value as f32 / 2.0).floor() as u8);
                    let right = Node::leaf((*value as f32 / 2.0).ceil() as u8);
                    *self = Node::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    return Some(());
                }
            }
            Node::Branch { left, right } => {
                if let Some(_) = left.split() {
                    return Some(());
                }
                if let Some(_) = right.split() {
                    return Some(());
                };
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
        self.pairs.iter().skip(1).fold(self.pairs[0].clone(), |result, next| {
            let mut result = Node::Branch {
                left: Box::new(result),
                right: Box::new(next.clone()),
            };
            while result.reduce() {}
            result
        })
    }

    /// 2nd half of the assignment
    pub fn largest_magnitude(&self) -> u32 {
        let max = self.pairs.iter().permutations(2).map(|pair| {
            let mut result = Node::branch(pair[0].clone(), pair[1].clone());
            while result.reduce() {}
            result.magnitude()
        }).max().expect("No max value found.");
        max
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

    let sum = pairs.sum();
    dbg!(sum.magnitude());

    let highest = pairs.largest_magnitude();
    dbg!(highest);

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
    fn calculate_sum_example() {
        let input = r#"
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]
        "#;
        let table = parse_input(input).expect("Failed to parse input.");
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            table.sum().to_string()
        );
    }

    #[test]
    fn calculate_sum_2nd_example() {
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
        assert_eq!(4140, table.sum().magnitude());
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
            assert!(node.explode());
            assert_eq!(expected, &node.to_string());
        }
        Ok(())
    }

    #[test]
    fn test_add_pairs() -> anyhow::Result<()> {
        let lhs = Node::try_from("[1,2]")?;
        let rhs = Node::try_from("[[3,4],5]")?;

        let result = lhs + rhs;
        assert_eq!("[[1,2],[[3,4],5]]", result.to_string());
        Ok(())
    }

    #[test]
    fn calculate_magnitudes() -> anyhow::Result<()> {
        assert_eq!(143, Node::try_from("[[1,2],[[3,4],5]]")?.magnitude());
        assert_eq!(1384, Node::try_from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?.magnitude());
        assert_eq!(445, Node::try_from("[[[[1,1],[2,2]],[3,3]],[4,4]]")?.magnitude());
        assert_eq!(791, Node::try_from("[[[[3,0],[5,3]],[4,4]],[5,5]]")?.magnitude());
        assert_eq!(1137, Node::try_from("[[[[5,0],[7,4]],[5,5]],[6,6]]")?.magnitude());
        assert_eq!(3488, Node::try_from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?.magnitude());

        Ok(())
    }

    #[test]
    fn find_largest_magnitude_pair() {
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
        assert_eq!(3993, table.largest_magnitude());
    }
}
