use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Folds one half of sheet onto the other half
    ///
    /// This maps the numbers from one axis back to the first half:
    ///
    /// 0  1  2  3  4  5  6 [7] 8  9 10 11 12 13 14
    /// 0  1  2  3  4  5  6  7  6  5  4  3  2  1  0
    ///
    pub fn fold(&self) -> Self {
        fn flip(val: u16, line: u16) -> u16 {
            ((line as i32 + 1) - ((line as i32 + 1) - val as i32).abs()) as u16
        }

        let max = match self.folds[0] {
            Fold::Horizontal(y) => Point::new(self.max.x, y - 1),
            Fold::Vertical(x) => Point::new(x - 1, self.max.y),
        };

        let points = self.points
            .iter()
            .map(|p| Point::new(flip(p.x, max.x), flip(p.y, max.y)))
            .unique()
            .collect_vec();

        Self {
            points,
            folds: self.folds[1..].iter().cloned().collect_vec(),
            max,
        }
    }
}

fn parse_input(input: &str) -> Sheet {
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
    dbg!(sheet.points.len());
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
        assert_eq!(18, sheet.points.len());
        assert_eq!(vec![Fold::Horizontal(7), Fold::Vertical(5),], sheet.folds);
        assert_eq!(Point::new(10, 14), sheet.max);
    }

    #[test]
    fn fold_once() {
        let sheet = parse_input(INPUT);
        let sheet = sheet.fold();
        assert_eq!(Point::new(10, 6), sheet.max);
        assert_eq!(17, sheet.points.len());
        assert_eq!(vec![Fold::Vertical(5)], sheet.folds);
    }
}
