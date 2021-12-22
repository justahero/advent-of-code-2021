use std::{collections::HashSet, ops::{Add, Sub}};

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
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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
    /// Finds and possibly returns shared beacons
    ///
    /// First bring find the best alignment of the 2nd list of points, if there
    /// are more than 12 shared beacons, return this list.
    pub fn shared_beacons(&self, beacons: &mut HashSet<Point>) -> Option<Point> {
        // try to find the alignment
        for alignment in 0..Point::NUM_ALIGNMENTS {
            // first rotate & flip all points
            let rotated_points = self.points
                .iter()
                .map(|p| p.rotate(alignment as u8))
                .collect::<Vec<_>>();

            // calculate distances to existing beacons
            let distances = beacons
                .iter()
                .cartesian_product(&rotated_points)
                .map(|(l, r)| l - r)
                .collect_vec();

            for distance in &distances {
                let translated_points = rotated_points.iter().map(|p| p + distance);
                let count = translated_points.clone().filter(|p| self.points.contains(p)).count();

                // when there are at least 12 shared points, we consider both scanners in range of each other
                if count >= 12 {
                    beacons.extend(translated_points);
                    return Some(distance.clone());
                }
            }
        }

        None
    }
}

impl From<&str> for Report {
    fn from(line: &str) -> Self {
        let points = line.lines().skip(1).map(Point::from).collect::<HashSet<_>>();
        Self { points }
    }
}

/// Retruns the number of shared beacons
fn shared_beacons(reports: Vec<Report>) -> usize {
    let mut distances = Vec::new();
    let mut beacons = reports[0].points.iter().cloned().collect::<HashSet<_>>();
    
    distances.push(Point::new(0, 0, 0));
    for report in reports.iter().skip(1) {
        if let Some(distance) = report.shared_beacons(&mut beacons) {
            distances.push(distance);
        }
    }

    beacons.len()
}

/// Parses the list of scanner reports
fn parse_input(input: &str) -> Vec<Report> {
    input.split("\n\n").map(Report::from).collect_vec()
}

fn main() {
    let reports = parse_input(include_str!("input.txt"));

    // get first solution
    dbg!(shared_beacons(reports));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, shared_beacons};

    const INPUT: &str = include_str!("example.txt");

    #[test]
    fn parse_scanner_input() {
        let reports = parse_input(INPUT);
        assert_eq!(5, reports.len());
    }

    #[test]
    fn test_shared_number_of_beacons() {
        let reports = parse_input(INPUT);
        assert_eq!(79, shared_beacons(reports));
    }
}
