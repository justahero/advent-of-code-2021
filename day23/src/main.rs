use itertools::Itertools;

use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Amphipod {
    pub energy: u32,
}

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Self { energy: 1 },
            'B' => Self { energy: 10 },
            'C' => Self { energy: 100 },
            'D' => Self { energy: 1000 },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Corridor,
    Room,
    Wall,
    Occupied(Amphipod),
}

#[derive(Debug, Clone)]
struct Field {
    pub x: i32,
    pub y: i32,
    pub state: State,
}

impl Field {
    pub fn new(x: i32, y: i32, state: State) -> Self {
        Self { x, y, state }
    }
}

#[derive(Debug)]
struct Grid {
    pub fields: Vec<Field>,
}

impl Grid {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    /// Returns the positions of all amphipods
    pub fn amphipods(&self) -> Vec<Field> {
        self.fields
            .iter()
            .cloned()
            .filter(|f| {
                if let State::Occupied(_amphipod) = f.state {
                    true
                } else {
                    false
                }
            })
            .collect_vec()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Parses the input
fn parse_input(input: &str) -> Grid {
    let mut fields = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match c {
                '#' | ' ' => fields.push(Field::new(x, y, State::Wall)),
                '.' => fields.push(Field::new(x, y, State::Corridor)),
                'A' | 'B' | 'C' | 'D' => {
                    fields.push(Field::new(x, y, State::Occupied(Amphipod::from(c))));
                }
                _ => unreachable!(),
            }
        }
    }

    Grid::new(fields)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        assert_eq!(8, grid.amphipods().len());
    }
}
