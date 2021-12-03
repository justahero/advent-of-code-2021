/// States the submarine can move
#[derive(Debug)]
pub enum Move {
    Forward(i64),
    Down(i64),
    Up(i64),
}

/// Moves the submarine with the given steps
pub fn do_move(moves: &[Move]) -> (i64, i64) {
    let (mut depth, mut horizontal) = (0, 0);
    for step in moves {
        match step {
            Move::Forward(x) => horizontal += x,
            Move::Down(x) => depth += x,
            Move::Up(x) => depth -= x,
        }
    }
    (depth, horizontal)
}

/// Aims the submarine rather than move with the given steps
pub fn do_aim(moves: &[Move]) -> (i64, i64) {
    let (mut depth, mut horizontal, mut aim) = (0, 0, 0);

    for step in moves {
        match step {
            Move::Forward(x) => {
                horizontal += x;
                depth += aim * x;
            }
            Move::Down(x) => aim += x,
            Move::Up(x) => aim -= x,
        }
    }

    (depth, horizontal)
}

/// Parse input
fn parse(instructions: &str) -> Vec<Move> {
    instructions
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(|line| {
            if let Some((movement, value)) = line.split_once(" ") {
                let step = value.parse::<i64>().expect("Failed to parse integer");
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
    let (depth, horizontal) = do_move(&instructions);
    dbg!(depth * horizontal);

    let (depth, horizontal) = do_aim(&instructions);
    dbg!(depth * horizontal);
}

#[cfg(test)]
mod tests {
    use crate::{do_aim, do_move, parse};

    const INSTRUCTIONS: &str = r#"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "#;

    #[test]
    fn test_do_move() {
        let instructions = parse(&INSTRUCTIONS);
        let (depth, horizontal) = do_move(&instructions);
        assert_eq!(15, horizontal);
        assert_eq!(10, depth);
    }

    #[test]
    fn test_do_aim() {
        let instructions = parse(&INSTRUCTIONS);
        let (depth, horizontal) = do_aim(&instructions);
        assert_eq!(15, horizontal);
        assert_eq!(60, depth);
    }
}
