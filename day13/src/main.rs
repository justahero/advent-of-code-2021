use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Point {
    fn from(line: &str) -> Self {
        let (x, y) = line.split_once(',').expect("Failed to parse point.");
        let (x, y) = (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap());
        Self::new(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Fold {
    Horizontal(u16),
    Vertical(u16),
}

impl From<&str> for Fold {
    fn from(line: &str) -> Self {
        let (_, instruction) = line.rsplit_once(" ").expect("Failed to split line.");
        let (axis, value) = instruction.split_once("=").expect("Failed to split fold");
        let value = value.parse::<u16>().unwrap();

        if axis == "y" {
            Fold::Horizontal(value)
        } else {
            Fold::Vertical(value)
        }
    }
}

#[derive(Debug)]
struct Sheet {
    pub points: Vec<Point>,
    pub folds: Vec<Fold>,
    pub max: Point,
}

impl Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max.y {
            let line = (0..=self.max.x)
                .into_iter()
                .map(|x| {
                    if self.points.contains(&Point::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .join("");

            writeln!(f, "{}", line)?;
        }
        write!(f, "")
    }
}

impl Sheet {
    pub fn new(points: Vec<Point>, folds: Vec<Fold>) -> Self {
        // let (min_x, max_x) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();

        Self {
            points,
            folds,
            max: Point::new(max_x, max_y),
        }
    }

    pub fn fold(&self) -> Self {
        let fold = self.folds.first().unwrap();
        let points = Vec::new();

        match fold {
            Fold::Horizontal(y) => {}
            Fold::Vertical(x) => {}
        }

        Self {
            points,
            folds: self.folds[1..].iter().cloned().collect_vec(),
            max: Point::new(0, 0),
        }
    }
}

fn parse_input(input: &str) -> Sheet {
    // parse all dots
    let mut points = Vec::new();
    let mut folds = Vec::new();

    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    for line in lines {
        if line.starts_with("fold along") {
            folds.push(Fold::from(line));
        } else {
            points.push(Point::from(line));
        }
    }

    Sheet::new(points, folds)
}

fn main() {
    let sheet = parse_input(include_str!("input.txt"));
    let sheet = sheet.fold();
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, Fold, Point};

    const INPUT: &str = r#"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "#;

    #[test]
    fn check_parse_input() {
        let sheet = parse_input(INPUT);
        println!("SHEET:\n{}", sheet);
        assert_eq!(18, sheet.points.len());
        assert_eq!(vec![Fold::Horizontal(7), Fold::Vertical(5),], sheet.folds);
        assert_eq!(Point::new(10, 14), sheet.max);
    }
}
