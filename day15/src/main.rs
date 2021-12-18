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
        let (initial_node, _) = self.fields[0];

        let mut best = self
            .fields
            .iter()
            .cloned()
            .map(|(point, _)| (point, u32::MAX))
            .collect::<HashMap<_, _>>();

        let mut points: VecDeque<(Point, u32)> = VecDeque::new();
        points.push_back((initial_node, 0));

        while let Some((current, cost)) = points.pop_front() {
            if cost < best[&current] {
                best.insert(current, cost);

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let x = current.x as i32 + dx;
                    let y = current.y as i32 + dy;
                    if 0 <= y && y < self.height as i32 && 0 <= x && x < self.width as i32 {
                        let (neighbor, value) = self.fields[(y * self.width as i32 + x) as usize];
                        points.push_back((neighbor, cost + value as u32));
                    }
                }
            }
        }

        best[&Point::new(self.width - 1, self.height - 1)]
    }
}

fn parse_input(input: &str) -> Grid {
    parse_input_multiple(input, 1, 1)
}

fn parse_input_multiple(input: &str, repeat_x: u32, repeat_y: u32) -> Grid {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    let tile_height = lines.len() as u32;
    let tile_width = lines[0].len() as u32;

    let mut fields = Vec::new();
    for ry in 0..repeat_x {
        for (y, &line) in lines.iter().enumerate() {
            for rx in 0..repeat_y {
                for (x, c) in line.chars().enumerate() {
                    let px = rx * tile_width + x as u32;
                    let py = ry * tile_height + y as u32;

                    let digit = c.to_digit(10).unwrap() + rx + ry;
                    let digit = 1 + ((digit as u8 - 1) % 9);
                    fields.push((Point::new(px, py), digit));
                }
            }
        }
    }

    Grid::new(fields)
}

fn main() {
    let grid = parse_input(include_str!("input.txt"));
    let result = grid.find_shortest_path();
    dbg!(result);

    let grid = parse_input_multiple(include_str!("input.txt"), 5, 5);
    let result = grid.find_shortest_path();
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, parse_input_multiple};

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

    #[test]
    fn check_repeat_axes() {
        let grid = parse_input_multiple("8", 5, 5);
        let expected = r#"
            89123
            91234
            12345
            23456
            34567
        "#;
        let expected = parse_input(expected);
        assert_eq!(expected.fields, grid.fields);
    }

    #[test]
    fn find_shortest_path_2nd() {
        let grid = parse_input_multiple(INPUT, 5, 5);
        println!("GRID:\n{}", grid);
        assert_eq!(315, grid.find_shortest_path());
    }
}
