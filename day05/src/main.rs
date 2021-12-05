use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
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

    /// Returns an iterator if segment is a supported line, either horizontal / vertical
    /// or additionally diagonal, otherwise returns an Iterator that returns `None` immediately
    pub fn iter(&self, kind: LineDirection) -> impl Iterator<Item = Point> {
        match kind {
            LineDirection::Straight if self.is_straight() => LineIterator::new(self, false),
            LineDirection::Full if self.is_straight() || self.is_diagonal() => LineIterator::new(self, false),
            _ => LineIterator::new(self, true),
        }
    }

    fn is_straight(&self) -> bool {
        self.start.y == self.end.y || self.start.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs()
    }
}

/// A straigh line iterator
#[derive(Debug)]
struct LineIterator {
    pub segment: LineSegment,
    pub stepx: i32,
    pub stepy: i32,
    pub done: bool,
}

impl LineIterator {
    /// Creates a new iterator over given line segment, `done` marks the iterator already consumed.
    pub fn new(segment: &LineSegment, done: bool) -> Self {
        let stepx = (segment.end.x - segment.start.x).signum();
        let stepy = (segment.end.y - segment.start.y).signum();

        Self {
            segment: segment.clone(),
            stepx,
            stepy,
            done,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            if self.segment.start == self.segment.end {
                self.done = true;
            }

            // get current start
            let start = self.segment.start.clone();

            // calculate next start point and set in line segment
            let LineSegment { start: s, .. } = &self.segment;
            let next_start = Point::new(s.x + self.stepx, s.y + self.stepy);
            self.segment.start = next_start;

            Some(start)
        } else {
            None
        }
    }
}

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

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values = value
            .split(',')
            .map(str::trim)
            .map(|val| {
                val.parse::<i32>()
                    .map_err(|_| anyhow!("Failed to parse value."))
            })
            .collect::<Result<Vec<i32>, Self::Error>>()?;
        if values.len() != 2 {
            return Err(anyhow!("Failed to parse tuple."));
        }
        Ok(Point::new(values[0], values[1]))
    }
}

struct DepthMap {
    pub depths: Vec<Point>,
}

impl DepthMap {
    pub fn with_lines(segments: &[LineSegment], kind: LineDirection) -> Self {
        let mut depths = Vec::new();

        for segment in segments.iter() {
            for p in segment.iter(kind).collect::<Vec<_>>() {
                depths.push(p);
            }
        }

        Self { depths }
    }

    /// Returns all points where the depth is at least 2
    pub fn find_depths(&self) -> Vec<Point> {
        let mut map = HashMap::new();
        for p in self.depths.iter() {
            *map.entry(p).or_insert(0) += 1;
        }

        map.iter()
            .filter(|(_, &count)| count >= 2)
            .map(|(&p, _)| p.clone())
            .sorted()
            .collect_vec()
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<LineSegment>> {
    let points = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(" -> ")
                .map(str::trim)
                .map(Point::try_from)
                .collect::<Result<Vec<Point>, anyhow::Error>>()
        })
        .collect::<Result<Vec<Vec<Point>>, anyhow::Error>>()?;

    let segments = points
        .iter()
        .map(|p| LineSegment::new(p[0].clone(), p[1].clone()))
        .collect_vec();

    Ok(segments)
}

fn main() -> anyhow::Result<()> {
    let points = parse_input(include_str!("input.txt"))?;
    let depth_map = DepthMap::with_lines(&points, LineDirection::Straight);
    let depths = depth_map.find_depths();
    dbg!(depths.len());

    let depth_map = DepthMap::with_lines(&points, LineDirection::Full);
    let depths = depth_map.find_depths();
    dbg!(depths.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, DepthMap, LineSegment, Point, LineDirection};

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
            horizontal.iter(LineDirection::Straight).collect::<Vec<Point>>(),
        );

        let diagonal = LineSegment::new(Point::new(4, 2), Point::new(2, 4));
        assert_eq!(
            Vec::<Point>::new(),
            diagonal.iter(LineDirection::Straight).collect::<Vec<Point>>(),
        );
    }

    #[test]
    fn test_diagonal_line() {
        let iter = LineSegment::new(Point::new(4, 2), Point::new(2, 4)).iter(LineDirection::Full);
        assert_eq!(
            vec![Point::new(4, 2), Point::new(3, 3), Point::new(2, 4)],
            iter.collect::<Vec<Point>>(),
        );
    }

    #[test]
    fn parses_input() {
        let points = parse_input(INPUT).expect("Failed to parse input");
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
        let points = parse_input(INPUT).expect("Failed to parse input");
        let depth_map = DepthMap::with_lines(&points, LineDirection::Straight);

        assert_eq!(26, depth_map.depths.len());
        assert_eq!(
            vec![
                Point::new(0, 9),
                Point::new(1, 9),
                Point::new(2, 9),
                Point::new(3, 4),
                Point::new(7, 4),
            ],
            depth_map.find_depths(),
        );
    }

    /// 2nd part
    #[test]
    fn find_depths_with_all_lines() {
        let points = parse_input(INPUT).expect("Failed to parse input");
        let depth_map = DepthMap::with_lines(&points, LineDirection::Full);

        assert_eq!(12, depth_map.find_depths().len());
    }
}
