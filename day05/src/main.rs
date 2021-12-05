use std::{cmp::max, collections::HashMap};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value
            .split(',')
            .map(str::trim)
            .filter_map(|val| val.parse::<i32>().ok())
            .collect_tuple::<(i32, i32)>()
            .expect("Failed to parse Point.");
        Self::new(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LineDirection {
    Straight,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn points(&self, kind: LineDirection) -> Vec<Point> {
        if self.is_straight() || (kind == LineDirection::Full && self.is_diagonal()) {
            let LineSegment { start, end } = self;
            let stepx = (end.x - start.x).signum();
            let stepy = (end.y - start.y).signum();

            let count = max((start.x - end.x).abs(), (start.y - end.y).abs());
            (0..=count)
                .into_iter()
                .map(|n| Point::new(start.x + n * stepx, start.y + n * stepy))
                .collect_vec()
        } else {
            Vec::new()
        }
    }

    fn is_straight(&self) -> bool {
        self.start.y == self.end.y || self.start.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs()
    }
}

fn find_depths(segments: &[LineSegment], kind: LineDirection) -> Vec<Point> {
    let mut map = HashMap::new();

    for segment in segments.iter() {
        for p in segment.points(kind) {
            *map.entry(p).or_insert(0) += 1;
        }
    }

    map.iter()
        .filter(|(_, &count)| count >= 2)
        .map(|(p, _)| p.clone())
        .sorted()
        .collect_vec()
}

fn parse_input(input: &str) -> Vec<LineSegment> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(" -> ")
                .map(|part| part.trim())
                .map(|f| Point::from(f))
                .collect_tuple::<(Point, Point)>()
                .unwrap()
        })
        .map(|(start, end)| LineSegment::new(start, end))
        .collect_vec()
}

fn main() {
    let points = parse_input(include_str!("input.txt"));
    let depths = find_depths(&points, LineDirection::Straight);
    dbg!(depths.len());

    let depths = find_depths(&points, LineDirection::Full);
    dbg!(depths.len());
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, LineDirection, LineSegment, Point, find_depths};

    const INPUT: &str = r#"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "#;

    #[test]
    fn test_straight_lines() {
        let horizontal = LineSegment::new(Point::new(3, 0), Point::new(1, 0));
        assert_eq!(
            vec![Point::new(3, 0), Point::new(2, 0), Point::new(1, 0)],
            horizontal.points(LineDirection::Straight),
        );

        let diagonal = LineSegment::new(Point::new(4, 2), Point::new(2, 4));
        assert_eq!(
            Vec::<Point>::new(),
            diagonal.points(LineDirection::Straight),
        );
    }

    #[test]
    fn test_diagonal_line() {
        let points =
            LineSegment::new(Point::new(4, 2), Point::new(2, 4)).points(LineDirection::Full);

        assert_eq!(
            vec![Point::new(4, 2), Point::new(3, 3), Point::new(2, 4)],
            points,
        );
    }

    #[test]
    fn parses_input() {
        let points = parse_input(INPUT);
        assert_eq!(
            vec![
                LineSegment::new(Point::new(0, 9), Point::new(5, 9)),
                LineSegment::new(Point::new(8, 0), Point::new(0, 8)),
                LineSegment::new(Point::new(9, 4), Point::new(3, 4)),
                LineSegment::new(Point::new(2, 2), Point::new(2, 1)),
                LineSegment::new(Point::new(7, 0), Point::new(7, 4)),
                LineSegment::new(Point::new(6, 4), Point::new(2, 0)),
                LineSegment::new(Point::new(0, 9), Point::new(2, 9)),
                LineSegment::new(Point::new(3, 4), Point::new(1, 4)),
                LineSegment::new(Point::new(0, 0), Point::new(8, 8)),
                LineSegment::new(Point::new(5, 5), Point::new(8, 2)),
            ],
            points,
        )
    }

    #[test]
    fn find_depths_with_straight_lines() {
        let points = parse_input(INPUT);
        let depths = find_depths(&points, LineDirection::Straight);

        assert_eq!(
            vec![
                Point::new(0, 9),
                Point::new(1, 9),
                Point::new(2, 9),
                Point::new(3, 4),
                Point::new(7, 4),
            ],
            depths,
        );
    }

    /// 2nd part
    #[test]
    fn find_depths_with_all_lines() {
        let points = parse_input(INPUT);
        let depths = find_depths(&points, LineDirection::Full);

        assert_eq!(12, depths.len());
    }
}
