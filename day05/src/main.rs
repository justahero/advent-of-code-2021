use anyhow::anyhow;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
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

fn parse_input(input: &str) -> anyhow::Result<Vec<(Point, Point)>> {
    let points = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(" -> ")
                .map(str::trim)
                .map(|v| Point::try_from(v))
                .collect::<Result<Vec<Point>, anyhow::Error>>()
        })
        .collect::<Result<Vec<Vec<Point>>, anyhow::Error>>()?;

    let points = points
        .iter()
        .map(|p| (p[0].clone(), p[1].clone()))
        .collect_vec();

    Ok(points)
}

fn main() -> anyhow::Result<()> {
    let _points = parse_input(include_str!("input.txt"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, Point};

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
    fn parses_input() {
        let points = parse_input(INPUT).expect("Failed to parse input");
        assert_eq!(
            vec![
                (Point::new(0, 9), Point::new(5, 9)),
                (Point::new(8, 0), Point::new(0, 8)),
                (Point::new(9, 4), Point::new(3, 4)),
                (Point::new(2, 2), Point::new(2, 1)),
                (Point::new(7, 0), Point::new(7, 4)),
                (Point::new(6, 4), Point::new(2, 0)),
                (Point::new(0, 9), Point::new(2, 9)),
                (Point::new(3, 4), Point::new(1, 4)),
                (Point::new(0, 0), Point::new(8, 8)),
                (Point::new(5, 5), Point::new(8, 2)),
            ],
            points,
        )
    }
}
