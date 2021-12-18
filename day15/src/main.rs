use std::{cmp::min, collections::{HashMap, VecDeque}, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: u32,
    pub y: u32,
    pub value: u8,
}

impl Point {
    pub fn new(x: u32, y: u32, value: u8) -> Self {
        Self { x, y, value }
    }
}

#[derive(Debug)]
struct Grid {
    pub fields: Vec<Point>,
    pub width: usize,
    pub height: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for points in self.fields.chunks(self.width as usize) {
            let values = points.iter().map(|p| p.value).join("");
            writeln!(f, "{}", values)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn new(fields: Vec<Point>) -> Self {
        let width = fields.iter().max_by_key(|&p| p.x).unwrap().x as usize + 1;
        let height = fields.iter().max_by_key(|&p| p.y).unwrap().y as usize + 1;

        Self {
            width,
            height,
            fields,
        }
    }

    pub fn find_shortest_path(&self) -> usize {
        println!("find_shortest_path start");
        let initial_node = self.get(0, 0).expect("Failed to get initial node.");

        // not sure both hash maps are required
        let mut unvisted = self
            .fields
            .iter()
            .cloned()
            .map(|node| (node, u32::MAX))
            .collect::<HashMap<_, _>>();
        let mut visited: HashMap<Point, u32> = HashMap::new();

        // Set initial node to distance 0
        let mut points: VecDeque<&Point> = VecDeque::new();
        points.push_back(initial_node);

        while let Some(point) = points.pop_front() {
            println!("  point: {:?}", point);

            if let Some(current_distance) = unvisted.remove(point) {
                let neighbors = self.neighbors(point.x, point.y).collect_vec();
                for neighbor in neighbors.iter() {
                    if let Some(d) = unvisted.get_mut(neighbor) {
                        *d = min(current_distance + neighbor.value as u32, *d);
                    }
                }

                for neighbor in neighbors.iter() {
                    if !visited.contains_key(neighbor) {
                        points.push_back(neighbor);
                    }
                }

                visited.insert(*point, current_distance);
            }
        }

        0
    }

    /// Returns the `Point` at coordinates x,y
    fn get(&self, x: i32, y: i32) -> Option<&Point> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.fields
                .get((y as u32 * self.width as u32 + x as u32) as usize)
        } else {
            None
        }
    }

    fn neighbors(&self, x: u32, y: u32) -> impl Iterator<Item = &Point> + '_ {
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
                .map(move |(x, c)| Point::new(x as u32, y as u32, c.to_digit(10).unwrap() as u8))
        })
        .collect_vec();

    Grid::new(fields)
}

fn main() {
    let grid = parse_input(include_str!("input.txt"));
    let result = grid.find_shortest_path();
    dbg!(result);
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
