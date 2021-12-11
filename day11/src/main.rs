use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    pub width: u32,
    pub height: u32,
    pub fields: Vec<u8>,
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

impl Grid {
    pub fn new(width: u32, height: u32, fields: Vec<u8>) -> Self {
        Self {
            width,
            height,
            fields,
        }
    }

    /// Returns true when all fields are zero
    pub fn is_synched(&self) -> bool {
        self.flashes() == self.width as usize * self.height as usize
    }

    /// Returns the number of flashes
    pub fn flashes(&self) -> usize {
        self.fields.iter().filter(|&&val| val == 0).count()
    }

    /// Get the energy level of a field if available
    pub fn get(&self, x: u32, y: u32) -> u8 {
        assert!(x < self.width);
        assert!(y < self.height);
        self.fields[(y * self.width + x) as usize]
    }

    pub fn inc(&mut self, x: u32, y: u32, allow: bool) {
        assert!(x < self.width);
        assert!(y < self.height);
        let value = &mut self.fields[(y * self.width + x) as usize];
        if *value > 0 || allow {
            *value += 1;
        }
    }

    /// Reset the field after a flash back to energy level 0
    pub fn reset(&mut self, x: u32, y: u32) {
        assert!(x < self.width);
        assert!(y < self.height);
        *(&mut self.fields[(y * self.width + x) as usize]) = 0;
    }

    /// Advance the grid by a single step, returns the new grid and the number of flashes
    pub fn single_step(&mut self) {
        // Increase all fields by one
        for y in 0..self.height {
            for x in 0..self.width {
                self.inc(x, y, true);
            }
        }

        loop {
            let mut flash_happened = false;
            for y in 0..self.height {
                for x in 0..self.width {
                    let value = self.get(x, y);
                    if value > 9 {
                        self.reset(x, y);
                        flash_happened = true;

                        // check all neighbors
                        for &(nx, ny) in NEIGHBORS.iter() {
                            let nx = nx + x as i32;
                            let ny = ny + y as i32;
                            if 0 <= nx
                                && nx < self.width as i32
                                && 0 <= ny
                                && ny < self.height as i32
                            {
                                self.inc(nx as u32, ny as u32, false);
                            }
                        }
                    }
                }
            }

            // return if no flash happened
            if !flash_happened {
                break;
            }
        }
    }

    /// Advances the grid by a number of steps, returns the resulting grid & number of observed flashes
    pub fn steps(&self, count: u32) -> (Grid, u32) {
        (0..count).fold((self.clone(), 0), |(mut grid, flashes), _| {
            grid.single_step();
            let next_flashes = grid.flashes() as u32;
            (grid, flashes + next_flashes)
        })
    }

    /// Determines when all octopuses are in sync, returns the step when this first occurs.
    pub fn find_synched_step(&self) -> u32 {
        let mut grid = self.clone();
        let mut step = 0;
        loop {
            grid.single_step();
            step += 1;
            if grid.is_synched() {
                break;
            }
        }
        step
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.fields.chunks(self.width as usize) {
            writeln!(f, "{:?}", chunk)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Grid {
    let lines = input.lines().map(str::trim).filter(|line| !line.is_empty());

    let fields = lines
        .map(|line| {
            line.chars()
                .filter_map(|val| format!("{}", val).parse::<u8>().ok())
                .collect_vec()
        })
        .collect::<Vec<_>>();

    let width = fields[0].len();
    let height = fields.len();

    let fields = fields.iter().flatten().cloned().collect_vec();

    Grid::new(width as u32, height as u32, fields)
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

    let (_, flashes) = grid.steps(100);
    dbg!(flashes);

    let step = grid.find_synched_step();
    dbg!(step);
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
    fn check_grid_after_single_step() {
        let expected_grid = parse_input(
            r#"
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
        "#,
        );
        let mut grid = parse_input(INPUT);
        grid.single_step();
        assert_eq!(expected_grid, grid);
        assert_eq!(0, grid.flashes());
    }

    #[test]
    fn check_grid_after_flashes() {
        let mut grid = parse_input(
            r#"
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
        "#,
        );
        let expected = parse_input(
            r#"
            8807476555
            5089087054
            8597889608
            8485769600
            8700908800
            6600088989
            6800005943
            0000007456
            9000000876
            8700006848
        "#,
        );
        grid.single_step();
        assert_eq!(expected, grid);
        assert_eq!(35, grid.flashes());
    }

    #[test]
    fn check_grid_after_10_steps() {
        let expected_grid = parse_input(
            r#"
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
        "#,
        );
        let grid = parse_input(INPUT);
        assert_eq!((expected_grid, 204), grid.steps(10));
    }

    #[test]
    fn check_grid_after_100_steps() {
        let expected_grid = parse_input(
            r#"
            0397666866
            0749766918
            0053976933
            0004297822
            0004229892
            0053222877
            0532222966
            9322228966
            7922286866
            6789998766
        "#,
        );
        let grid = parse_input(INPUT);
        assert_eq!((expected_grid, 1656), grid.steps(100));
    }

    #[test]
    fn test_find_synched_step() {
        let grid = parse_input(INPUT);
        assert_eq!(195, grid.find_synched_step());
    }
}
