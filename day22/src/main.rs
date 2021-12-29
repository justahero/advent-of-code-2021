use anyhow::anyhow;
use itertools::Itertools;

peg::parser! {
    grammar line_parser() for str {
        rule number() -> i32
            = n:$(['-']* ['0'..='9']+) { n.parse().unwrap() }

        rule ws()
            = " "

        rule comma()
            = ","

        rule state() -> State
            = s:$("on" / "off") { State::from(s) }

        rule range() -> (i32, i32)
            = l:number() ".." r:number() {
                (std::cmp::min(l, r), std::cmp::max(l, r))
            }

        pub(crate) rule instruction() -> Instruction
            = state:state() ws() "x=" x:range() comma() "y=" y:range() comma() "z=" z:range() {
                let cube = Cube {
                    x: Bounds::new(x.0, x.1),
                    y: Bounds::new(y.0, y.1),
                    z: Bounds::new(z.0, z.1),
                };
                Instruction {
                    state,
                    cube,
                }
            }
    }
}

#[derive(Debug, Clone)]
struct Bounds {
    pub min: i32,
    pub max: i32,
}

impl Bounds {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    On,
    Off,
}

impl From<&str> for State {
    fn from(val: &str) -> Self {
        match val {
            "on" => State::On,
            "off" => State::Off,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Cube {
    pub x: Bounds,
    pub y: Bounds,
    pub z: Bounds,
}

impl Cube {
    pub fn new(x: Bounds, y: Bounds, z: Bounds) -> Self {
        Self { x, y, z }
    }

    pub fn dim(dim: i32) -> Self {
        Self {
            x: Bounds::new(-dim, dim),
            y: Bounds::new(-dim, dim),
            z: Bounds::new(-dim, dim),
        }
    }

    /// Returns true if this cube overlaps with the other
    pub fn overlaps(&self, rhs: &Cube) -> bool {
        (self.x.min <= rhs.x.max && self.x.max >= rhs.x.min)
            && (self.y.min <= rhs.y.max && self.y.max >= rhs.y.min)
            && (self.z.min <= rhs.z.max && self.z.max >= rhs.z.min)
    }

    pub fn intersection(&mut self, rhs: &Cube) -> Vec<Cube> {
        let mut cubes = Vec::new();
        if !self.overlaps(rhs) {
            cubes.push(self.to_owned());
        } else {
            if self.x.min < rhs.x.min {
                let cube = Cube::new(
                    Bounds::new(self.x.min, rhs.x.min - 1),
                    Bounds::new(self.y.min, self.y.max),
                    Bounds::new(self.z.min, self.z.max),
                );
                cubes.push(cube);
                self.x.min = rhs.x.min;
            }
            if self.x.max > rhs.x.max {
                let cube = Cube::new(
                    Bounds::new(rhs.x.max + 1, self.x.max),
                    Bounds::new(self.y.min, self.y.max),
                    Bounds::new(self.z.min, self.z.max),
                );
                cubes.push(cube);
                self.x.max = rhs.x.max;
            }
            if self.y.min < rhs.y.min {
                let cube = Cube::new(
                    Bounds::new(self.x.min, self.x.max),
                    Bounds::new(self.y.min, rhs.y.min - 1),
                    Bounds::new(self.z.min, self.z.max),
                );
                cubes.push(cube);
                self.y.min = rhs.y.min;
            }
            if self.y.max > rhs.y.max {
                let cube = Cube::new(
                    Bounds::new(self.x.min, self.x.max),
                    Bounds::new(rhs.y.max + 1, self.y.max),
                    Bounds::new(self.z.min, self.z.max),
                );
                cubes.push(cube);
                self.y.max = rhs.y.max;
            }
            if self.z.min < rhs.z.min {
                let cube = Cube::new(
                    Bounds::new(self.x.min, self.x.max),
                    Bounds::new(self.y.min, self.y.max),
                    Bounds::new(self.z.min, rhs.z.min - 1),
                );
                cubes.push(cube);
                self.z.min = rhs.z.min;
            }
            if self.z.max > rhs.z.max {
                let cube = Cube::new(
                    Bounds::new(self.x.min, self.x.max),
                    Bounds::new(self.y.min, self.y.max),
                    Bounds::new(rhs.z.max + 1, self.z.max),
                );
                cubes.push(cube);
                self.z.max = rhs.z.max;
            }
        }
        cubes.into_iter().filter(|c| c.volume() > 0).collect_vec()
    }

    #[inline(always)]
    pub fn volume(&self) -> usize {
        let x = 0.max(self.x.max - self.x.min) as i64 + 1;
        let y = 0.max(self.y.max - self.y.min) as i64 + 1;
        let z = 0.max(self.z.max - self.z.min) as i64 + 1;
        (x * y * z) as usize
    }
}

#[derive(Debug)]
struct Instruction {
    pub state: State,
    pub cube: Cube,
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        line_parser::instruction(line).map_err(|e| anyhow!("Failed to parse line '{}'", e))
    }
}

#[derive(Debug)]
struct Reactor {
    pub instructions: Vec<Instruction>,
}

impl Reactor {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub fn part1(&self, dim: i32) -> usize {
        let dim = Cube::dim(dim);
        self.reboot()
            .iter()
            .filter(|&c| {
                c.x.min >= dim.x.min
                    && c.x.max <= dim.x.max
                    && c.y.min >= dim.y.min
                    && c.y.max <= dim.y.max
                    && c.z.min >= dim.z.min
                    && c.z.max <= dim.z.max
            })
            .map(|c| c.volume())
            .sum::<usize>()
    }

    pub fn part2(&self) -> usize {
        self.reboot()
            .into_iter()
            .map(|c| c.volume())
            .sum::<usize>()
    }

    pub fn reboot(&self) -> Vec<Cube> {
        let mut result: Vec<Cube> = Vec::new();
        for Instruction { cube, state } in self.instructions.iter() {
            let mut cubes = Vec::new();

            for index in 0..result.len() {
                cubes.extend(result[index].intersection(&cube));
            }

            if *state == State::On {
                cubes.push(cube.clone());
            }

            result = cubes;
        }

        result
    }
}

fn parse_input(input: &str) -> anyhow::Result<Reactor> {
    let instructions = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(Instruction::try_from)
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(Reactor::new(instructions))
}

fn main() -> anyhow::Result<()> {
    let reactor = parse_input(include_str!("input.txt"))?;

    dbg!(reactor.part1(50));
    dbg!(reactor.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::{parse_input, Bounds, Cube, Instruction, State};

    const INPUT: &str = r#"
        on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682
    "#;

    #[test]
    fn test_cubes_overlap() {
        let lhs = Cube::new(Bounds::new(0, 4), Bounds::new(0, 4), Bounds::new(1, 4));
        let rhs = Cube::new(Bounds::new(0, 3), Bounds::new(2, 3), Bounds::new(1, 4));
        assert!(lhs.overlaps(&rhs));
    }

    #[test]
    fn test_cube_does_not_overlap() {
        let lhs = Cube::new(Bounds::new(1, 3), Bounds::new(1, 3), Bounds::new(1, 2));
        let rhs = Cube::new(Bounds::new(4, 6), Bounds::new(4, 6), Bounds::new(1, 3));
        assert!(!lhs.overlaps(&rhs));
    }

    #[test]
    fn test_cube_volume() {
        let cube = Cube::new(Bounds::new(0, 4), Bounds::new(1, 5), Bounds::new(0, 4));
        assert_eq!(125, cube.volume());
    }

    #[test]
    fn test_parse_cube_line() -> anyhow::Result<()> {
        let instruction = Instruction::try_from("on x=-20..26,y=-36..17,z=-47..7")?;
        let cube = instruction.cube.borrow();

        assert_eq!((-20, 26), (cube.x.min, cube.x.max));
        assert_eq!((-36, 17), (cube.y.min, cube.y.max));
        assert_eq!((-47, 7), (cube.z.min, cube.z.max));
        assert_eq!(State::On, instruction.state);
        Ok(())
    }

    #[test]
    fn test_part1_example() {
        let reactor = parse_input(INPUT).expect("Failed to parse input.");
        assert_eq!(590784, reactor.part1(50));
    }

    #[test]
    fn test_part2_example() {
        let reactor = parse_input(include_str!("example.txt")).expect("Failed to parse input.");
        assert_eq!(2758514936282235, reactor.part2());
    }
}
