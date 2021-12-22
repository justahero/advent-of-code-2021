use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
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
    pub points: Vec<Point>,
}

impl Report {
    /// Finds and possibly returns shared beacons
    ///
    /// First bring find the best alignment of the 2nd list of points, if there
    /// are more than 12 shared beacons, return this list.
    pub fn shared_beacons(&self, other: &Vec<Point>) -> Option<HashSet<Point>> {
        // TODO
        // get all combinations of alignments?

        Some(HashSet::new())
    }
}

impl From<&str> for Report {
    fn from(line: &str) -> Self {
        let points = line.lines().skip(1).map(Point::from).collect_vec();
        Self { points }
    }
}

/// Retruns the number of shared beacons
fn shared_beacons(reports: Vec<Report>) -> usize {
    let beacons = reports
        .iter()
        .tuple_combinations()
        .fold(HashSet::new(), |mut result, (left, right)| {
            if let Some(shared) = left.shared_beacons(&right.points) {
                result.extend(shared);
            }
            result
        });

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
