use anyhow::anyhow;
use std::fmt::Display;

// Simple grammar to parse snailfish pairs
peg::parser! {
    grammar line_parser() for str {
        rule literal() -> Node
            = l:$(['0'..='9']+) { Node::Leaf(l.parse::<u8>().unwrap()) }

        rule comma()
            = ","

        rule open()
            = "["

        rule close()
            = "]"

        rule pair() -> Node
            = open() l:(literal() / pair()) comma() r:(literal() / pair()) close() { Node::branch(l, r) }

        pub(crate) rule parse() -> Node
            = open() l:(literal() / pair()) comma() r:(literal() / pair()) close() { Node::branch(l, r) }
    }
}

/// A binary tree representation
#[derive(Debug)]
enum Node {
    Leaf(u8),
    Branch { left: Box<Node>, right: Box<Node> },
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Node::Leaf(v) => format!("{}", v),
            Node::Branch { left, right} => format!("[{},{}]", left.to_string(), right.to_string()),
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Node {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        line_parser::parse(line).map_err(|e| anyhow!("Failed to parse line '{}': {}", line, e))
    }
}

impl Node {
    pub fn branch(left: Node, right: Node) -> Self {
        Node::Branch { left: Box::new(left), right: Box::new(right) }
    }

    /// If this node can explode, update the binary tree & return `true`, otherwise return `false`
    pub fn explode(&mut self) -> bool {
        if let Some(pair) = &mut self.can_explode(0) {
            println!("FOUND EXPLODING PAIR: {:?}", pair);
        }
        false
    }

    fn can_explode(&self, depth: u32) -> Option<&Node> {
        if let Node::Branch { left, right } = self {
            if depth >= 4 {
                if let (Node::Leaf(_), Node::Leaf(_)) = (left.as_ref(), right.as_ref()) {
                    return Some(&self);
                }
            }
            return left.can_explode(depth + 1).or(right.can_explode(depth + 1));
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
        Node::Leaf(1)
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
        assert!(Node::try_from("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").is_ok());
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
        assert!(Node::try_from("[[[[[9,8],1],2],3],4]")?.explode());
        assert!(Node::try_from("[7,[6,[5,[4,[3,2]]]]]")?.explode());
        assert!(Node::try_from("[7,[6,[5,[4,[3,2]]]]]")?.explode());
        Ok(())
    }
}