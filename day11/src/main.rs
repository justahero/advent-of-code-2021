use itertools::Itertools;

#[derive(Debug, Clone)]
struct Grid {
    pub width: usize,
    pub height: usize,
    pub fields: Vec<u8>,
}

impl Grid {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            fields: Vec::with_capacity((width * height) as usize),
        }
    }

    pub fn new(width: usize, height: usize, fields: Vec<u8>) -> Self {
        Self {
            width,
            height,
            fields,
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let lines = input.lines().map(str::trim).filter(|line| !line.is_empty());

    let fields = lines
        .map(|line| line.chars().map(|val| val.to_digit(10).unwrap() as u8).collect_vec())
        .collect::<Vec<_>>();

    let width = fields[0].len();
    let height = fields.len();

    Grid::new(10, 10, Vec::new())
}

fn main() {
    let input = r#"
        8577245547
        1654333653
        5365633785
        1333243226
        4272385165
        5688328432
        3175634254
        6775142227
        6152721415
        2678227325
    "#;
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "#;

    #[test]
    fn parses_grid() {
        let grid = parse_input(INPUT);
        assert_eq!(10, grid.width);
        assert_eq!(10, grid.height);
    }

    #[test]
    fn check_grid_after_steps() {
        let grid = parse_input(INPUT);
    }
}
