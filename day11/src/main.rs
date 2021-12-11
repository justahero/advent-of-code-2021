use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
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
            fields: vec![0; (width * height) as usize],
        }
    }

    pub fn new(width: usize, height: usize, fields: Vec<u8>) -> Self {
        Self {
            width,
            height,
            fields,
        }
    }

    /// Advance the grid by a single step
    pub fn single_step(&self) -> Self {
        let mut result = Grid::empty(self.width, self.height);

        result
    }

    /// Advances the grid by a number of steps
    pub fn steps(&self, count: u32) -> Self {
        self.clone()
    }
}

fn parse_input(input: &str) -> Grid {
    let lines = input.lines().map(str::trim).filter(|line| !line.is_empty());

    let fields = lines
        .map(|line| {
            line.chars()
                .map(|val| val.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect::<Vec<_>>();

    let width = fields[0].len();
    let height = fields.len();

    let fields = fields.iter().flatten().cloned().collect_vec();

    Grid::new(width, height, fields)
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
    let grid = parse_input(input);
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
        let expected_grid = parse_input(r#"
            6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637
        "#);
        let grid = parse_input(INPUT);
        assert_eq!(expected_grid, grid.single_step());
    }

    #[test]
    fn check_grid_after_10_steps() {
        let expected_grid = parse_input(r#"
            0481112976
            0031112009
            0041112504
            0081111406
            0099111306
            0093511233
            0442361130
            5532252350
            0532250600
            0032240000
        "#);
        let grid = parse_input(INPUT);
        assert_eq!(expected_grid, grid.steps(10));
    }
}
