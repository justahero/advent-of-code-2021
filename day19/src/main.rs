use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Sub},
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub const NUM_ALIGNMENTS: usize = 24;

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn rotate(&self, step: u8) -> Self {
        let &Self { x, y, z } = self;
        let (x, y, z) = match step {
            00 => (x, y, z),
            01 => (x, -y, -z),
            02 => (x, z, -y),
            03 => (x, -z, y),
            04 => (-x, y, -z),
            05 => (-x, -y, z),
            06 => (-x, z, y),
            07 => (-x, -z, -y),
            08 => (y, x, -z),
            09 => (y, -x, z),
            10 => (y, z, x),
            11 => (y, -z, -x),
            12 => (-y, x, z),
            13 => (-y, -x, -z),
            14 => (-y, z, -x),
            15 => (-y, -z, x),
            16 => (z, x, y),
            17 => (z, -x, -y),
            18 => (z, y, -x),
            19 => (z, -y, x),
            20 => (-z, x, -y),
            21 => (-z, -x, y),
            22 => (-z, y, x),
            23 => (-z, -y, -x),
            _ => unreachable!(),
        };
        Self { x, y, z }
    }

    pub fn manhattan(&self, rhs: &Point) -> usize {
        let Self { x, y, z } = self;
        ((rhs.x - x).abs() + (rhs.y - y).abs() + (rhs.z - z).abs()) as usize
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let Self { x, y, z } = self;
        Point::new(x - rhs.x, y - rhs.y, z - rhs.z)
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let Point { x, y, z } = self;
        Point::new(x - rhs.x, y - rhs.y, z - rhs.z)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let Self { x, y, z } = self;
        Point::new(x + rhs.x, y + rhs.y, z + rhs.z)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let &Point { x, y, z } = self;
        Point::new(x + rhs.x, y + rhs.y, z + rhs.z)
    }
}

impl From<&str> for Point {
    fn from(line: &str) -> Self {
        let (x, y, z) = line
            .splitn(3, ',')
            .into_iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect_tuple()
            .unwrap();
        Point::new(x, y, z)
    }
}

#[derive(Debug)]
struct Report {
    pub points: HashSet<Point>,
}

impl Report {
    /// Finds shared beacons.
    ///
    /// First find the best alignment of the report, if there
    /// are more than 12 shared beacons, update the given beacons list.
    pub fn find_beacons(&self, beacons: &HashSet<Point>) -> Option<(Point, HashSet<Point>)> {
        // try to find the alignment
        for alignment in 0..Point::NUM_ALIGNMENTS {
            // first rotate & flip all points
            let rotated_points = self
                .points
                .iter()
                .map(|p| p.rotate(alignment as u8))
                .collect::<Vec<_>>();

            // calculate distances to existing beacons
            let distances = beacons
                .iter()
                .cartesian_product(&rotated_points)
                .map(|(l, r)| l - r)
                .collect::<HashSet<_>>();

            for distance in &distances {
                let translated_points = rotated_points.iter().map(|p| p + distance);
                let count = translated_points
                    .clone()
                    .filter(|p| beacons.contains(p))
                    .count();

                // when there are at least 12 shared points, we consider both scanners in range of each other
                if count >= 12 {
                    return Some((distance.clone(), translated_points.collect::<HashSet<_>>()));
                }
            }
        }

        None
    }
}

impl From<&str> for Report {
    fn from(line: &str) -> Self {
        let points = line
            .lines()
            .skip(1)
            .map(Point::from)
            .collect::<HashSet<_>>();
        Self { points }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in &self.points {
            writeln!(f, "{}", p)?;
        }
        Ok(())
    }
}

/// Retruns the number of shared beacons
fn shared_beacons(mut reports: Vec<Report>) -> (Vec<Point>, HashSet<Point>) {
    // All reports are relative to the first
    let mut distances = Vec::with_capacity(reports.len() + 1);
    distances.push(Point::new(0, 0, 0));
    let mut beacons = reports[0].points.iter().cloned().collect::<HashSet<_>>();
    reports.remove(0);

    while !reports.is_empty() {
        for index in 0..reports.len() {
            if let Some((distance, transformed_beacons)) = reports[index].find_beacons(&beacons) {
                beacons.extend(transformed_beacons);
                distances.push(distance);
                reports.remove(index);
                break;
            }
        }
    }

    (distances, beacons)
}

/// Calculates the largest manhatten distance between scanners
fn manhattan_distance(distances: Vec<Point>) -> usize {
    distances
        .iter()
        .tuple_combinations()
        .map(|(l, r)| l.manhattan(r))
        .max()
        .unwrap()
}

/// Parses the list of scanner reports
fn parse_input(input: &str) -> Vec<Report> {
    input.split("\n\n").map(Report::from).collect_vec()
}

fn main() {
    let reports = parse_input(include_str!("input.txt"));
    let (distances, beacons) = shared_beacons(reports);

    dbg!(beacons.len());
    dbg!(manhattan_distance(distances));
}

#[cfg(test)]
mod tests {
    use crate::{manhattan_distance, parse_input, shared_beacons};

    const INPUT: &str = include_str!("example.txt");

    #[test]
    fn parse_scanner_input() {
        let reports = parse_input(INPUT);
        assert_eq!(5, reports.len());
    }

    #[test]
    fn test_shared_number_of_beacons() {
        let reports = parse_input(INPUT);
        let (_distances, beacons) = shared_beacons(reports);
        assert_eq!(79, beacons.len());
    }

    #[test]
    fn test_manhattan_distances() {
        let reports = parse_input(INPUT);
        let (distances, _beacons) = shared_beacons(reports);
        assert_eq!(3621, manhattan_distance(distances));
    }
}
