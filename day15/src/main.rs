use itertools::Itertools;

#[derive(Debug)]
struct Point {
    pub x: u16,
    pub y: u16,
    pub value: u8,
}

impl Point {
    pub fn new(x: u16, y: u16, value: u8) -> Self {
        Self { x, y, value }
    }
}

#[derive(Debug)]
struct Grid {
    pub fields: Vec<Point>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(fields: Vec<Point>) -> Self {
        let width  = fields.iter().max_by_key(|&p| p.x).unwrap().x as usize + 1;
        let height = fields.iter().max_by_key(|&p| p.y).unwrap().y as usize + 1;
    
        Self { width, height, fields }
    }
}

fn parse_input(input: &str) -> Grid {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    let fields = lines
        .iter()
        .enumerate()
        .flat_map(|(y, &line)| {
            line.chars().enumerate().map(move |(x, c)| Point::new(x as u16, y as u16, c as u8))
        })
        .collect_vec();

    Grid::new(fields)
}

fn main() {
    let grid = parse_input(include_str!("input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "#;

    #[test]
    fn parses_input_grid() {
        let grid = parse_input(INPUT);
        assert_eq!(10, grid.width);
        assert_eq!(10, grid.height);
        assert_eq!(100, grid.fields.len());
    }
}
