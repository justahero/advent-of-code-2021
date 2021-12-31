use itertools::Itertools;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
struct Amphipod {
    pub name: String,
    pub energy: u32,
}

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Self { name: c.to_string(), energy: 1 },
            'B' => Self { name: c.to_string(), energy: 10 },
            'C' => Self { name: c.to_string(), energy: 100 },
            'D' => Self { name: c.to_string(), energy: 1000 },
            _ => unreachable!(),
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Corridor,
    Room,
    Wall,
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

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match &self.state {
            State::Corridor => ".".to_string(),
            State::Room => "_".to_string(),
            State::Wall => "#".to_string(),
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug)]
struct Grid {
    pub fields: Vec<Field>,
    pub amphipods: Vec<(Field, Amphipod)>,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    pub fn new(fields: Vec<Field>, amphipods: Vec<(Field, Amphipod)>) -> Self {
        let width = fields.iter().map(|f| f.x).max().unwrap() as u32 + 1;
        let height = fields.iter().map(|f| f.y).max().unwrap() as u32 + 1;

        Self { fields, amphipods, width, height }
    }

    /// Moves all amphiods into their rooms, calculates minimum possible total entry
    pub fn organize(&self) -> usize {
        // let visited = self.amphipods().filter(|f|)
        0
    }

    fn neighbors(&self, x: u32, y: u32) -> impl Iterator<Item = Option<&Field>> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |&(nx, ny)| self.get(x as i32 + nx, y as i32 + ny))
    }

    fn get(&self, x: i32, y: i32) -> Option<&Field> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.fields.get((y as u32 * self.width + x as u32) as usize)
        } else {
            None
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(field) = self.get(x as i32, y as i32) {
                    write!(f, "{}", field)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Parses the input
fn parse_input(input: &str) -> Grid {
    let mut fields = Vec::new();
    let mut amphipods = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match c {
                '#' | ' ' => fields.push(Field::new(x, y, State::Wall)),
                '.' => fields.push(Field::new(x, y, State::Corridor)),
                'A' | 'B' | 'C' | 'D' => {
                    let field = Field::new(x, y, State::Room);
                    fields.push(field.clone());
                    amphipods.push((field, Amphipod::from(c)))
                }
                _ => unreachable!(),
            }
        }
    }

    Grid::new(fields, amphipods)
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
  #########  "#;

    #[test]
    fn test_parse_input() {
        let grid = parse_input(INPUT);
        assert_eq!(8, grid.amphipods.len());
    }

    #[test]
    fn test_organize_amphipods() {
        let grid = parse_input(INPUT);
        println!("GRID:\n{}", grid);
        assert_eq!(12521, grid.organize());
    }
}
