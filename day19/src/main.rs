use itertools::Itertools;

#[derive(Debug)]
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
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
}

impl From<&str> for Report {
    fn from(line: &str) -> Self {
        // skip first line
        let points = line.lines().skip(1).map(Point::from).collect_vec();
        Self { points }
    }
}

/// Parses the list of scanner reports
fn parse_input(input: &str) -> Vec<Report> {
    input
        .split("\n\n")
        .map(Report::from)
        .collect_vec()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = include_str!("example.txt");

    #[test]
    fn parse_scanner_input() {
        let reports = parse_input(INPUT);
        assert_eq!(5, reports.len());
    }
}
