use std::{collections::{HashMap, VecDeque}, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    pub fields: Vec<(Point, u8)>,
    pub width: u32,
    pub height: u32,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for points in self.fields.chunks(self.width as usize) {
            let values = points.iter().map(|(_p, value)| value).join("");
            writeln!(f, "{}", values)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(fields: Vec<(Point, u8)>) -> Self {
        let width = fields.iter().max_by_key(|&(p, _)| p.x).unwrap().0.x as u32 + 1;
        let height = fields.iter().max_by_key(|&(p, _)| p.y).unwrap().0.y as u32 + 1;

        Self {
            width,
            height,
            fields,
        }
    }

    pub fn find_shortest_path(&self) -> u32 {
        let (initial_node, _) = self.get(0, 0).expect("Failed to get initial node.");

        let mut best = self
            .fields
            .iter()
            .cloned()
            .map(|(point, _)| (point, u32::MAX))
            .collect::<HashMap<_, _>>();

        let mut points: VecDeque<(Point, u32)> = VecDeque::new();
        points.push_back((*initial_node, 0));

        while let Some((current, cost)) = points.pop_front() {
            if cost < best[&current] {
                best.insert(current, cost);
                for &(neighbor, distance) in self.neighbors(current.x, current.y) {
                    points.push_back((neighbor, cost + distance as u32));
                }
            }
        }

        best[&Point::new(self.width - 1, self.height - 1)]
    }

    /// Returns the `Point` at coordinates x,y
    fn get(&self, x: i32, y: i32) -> Option<&(Point, u8)> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.fields
                .get((y * self.width as i32 + x) as usize)
        } else {
            None
        }
    }

    fn neighbors(&self, x: u32, y: u32) -> impl Iterator<Item = &(Point, u8)> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(move |&(nx, ny)| self.get(x as i32 + nx, y as i32 + ny))
    }
}

fn parse_input(input: &str) -> Grid {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    let fields = lines
        .iter()
        .enumerate()
        .flat_map(|(y, &line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point::new(x as u32, y as u32), c.to_digit(10).unwrap() as u8))
        })
        .collect_vec();

    Grid::new(fields)
}

fn main() {
    let grid = parse_input(include_str!("input.txt"));

    let result = grid.find_shortest_path();
    dbg!(result);

    // result 467 too high
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "#;

    #[test]
    fn parses_input_grid() {
        let grid = parse_input(INPUT);
        assert_eq!(10, grid.width);
        assert_eq!(10, grid.height);
        assert_eq!(100, grid.fields.len());
    }

    #[test]
    fn find_shortest_path() {
        let grid = parse_input(INPUT);
        println!("GRID:\n{}", grid);
        assert_eq!(40, grid.find_shortest_path());
    }
}
