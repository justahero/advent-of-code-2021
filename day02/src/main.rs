#[derive(Debug, Clone)]
struct Submarine {
    pub depth: i32,
    pub horizontal: i32,
}

impl Submarine {
    pub fn new(depth: i32, horizontal: i32) -> Self {
        Self { depth, horizontal }
    }

    /// Moves the submarine with the given steps
    pub fn do_move(&self, moves: &[Move]) -> (i32, i32) {
        let Self { mut depth, mut horizontal } = &self;
        for step in moves {
            match step {
                Move::Forward(v) => horizontal += v,
                Move::Down(v) => depth += v,
                Move::Up(v) => depth -= v,
            }
        }
        (depth, horizontal)
    }
}

/// States the submarine can move
#[derive(Debug)]
enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

/// Parse input
fn parse(instructions: &str) -> Vec<Move> {
    instructions
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(|line| {
            if let Some((movement, value)) = line.split_once(" ") {
                let step = value.parse::<i32>().expect("Failed to parse integer");
                match movement {
                    "forward" => Move::Forward(step),
                    "down" => Move::Down(step),
                    "up" => Move::Up(step),
                    _ => panic!("Unsupported move '{}' found", movement),
                }
            } else {
                panic!("Failed to parse line '{}'", line);
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    let instructions = parse(include_str!("input.txt"));
    let submarine = Submarine::new(0, 0);
    let (depth, horizontal) = submarine.do_move(&instructions);

    dbg!(depth * horizontal);
}

#[cfg(test)]
mod tests {
    use crate::{parse, Submarine};

    #[test]
    fn follows_movement() {
        let instructions = r#"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "#;
        let instructions = parse(&instructions);
        let submarine = Submarine::new(0, 0);
        let (depth, horizontal) = submarine.do_move(&instructions);
        assert_eq!(15, horizontal);
        assert_eq!(10, depth);
    }
}
