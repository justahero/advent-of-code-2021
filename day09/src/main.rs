use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    pub x: u32,
    pub y: u32,
    pub depth: u8,
}

impl Point {
    pub fn new(x: u32, y: u32, depth: u8) -> Self {
        Self { x, y, depth }
    }
}

#[derive(Debug)]
struct HeightMap {
    pub width: u32,
    pub height: u32,
    pub points: Vec<Point>,
}

impl HeightMap {
    pub fn new(width: u32, height: u32, values: Vec<u8>) -> Self {
        let mut points = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;
                let depth = values[index];
                points.push(Point::new(x, y, depth));
            }
        }

        Self {
            width,
            height,
            points,
        }
    }

    /// Determine all basins in the heightmap.
    ///
    /// * for each low point, determine all other fields flowing into
    /// * a basin is surrounded by `9` (wall)
    ///
    pub fn find_basins(&self) {
        let low_points = self.find_low_points();

        
    }

    /// Return the number of fields that belong to the basin of the low point
    /// Use breadth search first for now, should be simple enough, maybe not fast
    pub fn find_basin(&self, x: u32, y: u32) -> usize {
        let mut basin: Vec<&Point> = Vec::new();
        let mut points: VecDeque<&Point> = VecDeque::new();
        points.push_back(self.get_point(x, y));

        while !points.is_empty() {
            break;
        }

        basin.len()
    }

    /// Find all low points in the height map
    /// These are points where all neighbors are higher than the current depth, a local minimum
    pub fn find_low_points(&self) -> Vec<Point> {
        let mut result = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let depth = self.get_depth(x as i32, y as i32);

                let neighbors = [
                    self.get_depth(x as i32 - 1, y as i32),
                    self.get_depth(x as i32 + 1, y as i32),
                    self.get_depth(x as i32, y as i32 - 1),
                    self.get_depth(x as i32, y as i32 + 1),
                ];

                if neighbors.iter().all(|&neighbor| neighbor > depth) {
                    result.push(self.get_point(x, y).clone());
                }
            }
        }

        result
    }

    /// Returns the depth at coordinates (x, y). If the coordinates are outside the heightmap return max value
    fn get_depth(&self, x: i32, y: i32) -> u8 {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            let index = y as u32 * self.width + x as u32;
            self.points[index as usize].depth
        } else {
            u8::MAX
        }
    }

    /// Returns the point at coordinates
    fn get_point(&self, x: u32, y: u32) -> &Point {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.points[(y * self.width + x) as usize]
    }

    /*
    fn neighbors(&self, x: u32, y: u32) -> impl Iterator<Item = Option<&Point>> + '_ {
        self.get(x as i32 - 1, y as i32);
        self.get(x as i32 + 1, y as i32);
        self.get(x as i32, y as i32 - 1);
        self.get(x as i32, y as i32 + 1);
    }
    */
}

fn parse_input(input: &str) -> HeightMap {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect_vec();

    let width = lines.iter().max_by_key(|&l| l.len()).expect("Failed to get width.").len();
    let height = lines.len();

    let values = lines
        .iter()
        .flat_map(|&line| line.chars())
        .map(|val| (val.to_string()).parse::<u8>().unwrap())
        .collect_vec();

    HeightMap::new(width as u32, height as u32, values)
}

fn main() {
    let height_map = parse_input(include_str!("input.txt"));
    let low_points = height_map.find_low_points();

    let risk_level = low_points.iter().map(|p| p.depth as u32 + 1).sum::<u32>();
    dbg!(risk_level);
}

#[cfg(test)]
mod tests {
    use crate::{Point, parse_input};

    const INPUT: &str = r#"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "#;
    
    #[test]
    fn check_find_lowest_points() {
        let height_map = parse_input(INPUT);
        assert_eq!(10, height_map.width);
        assert_eq!(5, height_map.height);
        assert_eq!(
            vec![
                Point::new(1, 0, 1),
                Point::new(9, 0, 0),
                Point::new(2, 2, 5),
                Point::new(6, 4, 5),
            ],
            height_map.find_low_points(),
        );
    }

    #[test]
    fn find_basin_of_low_point() {
        let height_map = parse_input(INPUT);
        assert_eq!(3, height_map.find_basin(1, 0));
        assert_eq!(9, height_map.find_basin(9, 0));
        assert_eq!(14, height_map.find_basin(2, 2));
        assert_eq!(9, height_map.find_basin(6, 4));
    }
}
