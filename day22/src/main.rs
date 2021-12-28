use anyhow::anyhow;

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

        pub(crate) rule cube() -> Cube
            = state:state() ws() "x=" x:range() comma() "y=" y:range() comma() "z=" z:range() {
                Cube {
                    start: Vec3::new(x.0, y.0, z.0),
                    end: Vec3::new(x.1, y.1, z.1),
                    state,
                    children: Vec::new(),
                }
            }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Cube {
    pub start: Vec3,
    pub end: Vec3,
    pub state: State,
    pub children: Vec<Cube>,
}

impl Cube {
    pub fn dim(dim: i32, state: State) -> Self {
        Self {
            start: Vec3::new(-dim, -dim, -dim),
            end: Vec3::new(dim, dim, dim),
            state,
            children: Vec::new(),
        }
    }

    pub fn switch(&mut self, cuboid: &Cube) {
        // TODO
    }

    pub fn count(&self, state: State) -> usize {
        0
    }
}

impl TryFrom<&str> for Cube {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        line_parser::cube(line).map_err(|e| anyhow!("Failed to parse line '{}'", e))
    }
}

#[derive(Debug)]
struct Reactor {
    pub instructions: Vec<Cube>,
}

impl Reactor {
    pub fn new(instructions: Vec<Cube>) -> Self {
        Self { instructions }
    }

    /// Reboots the reactor inside the given cuboid dimension
    pub fn reboot(&self, dim: i32) -> usize {
        let mut cuboid = Cube::dim(dim, State::Off);
        for instruction in self.instructions.iter() {
            cuboid.switch(instruction);
        }
        cuboid.count(State::On)
    }
}

fn parse_input(input: &str) -> anyhow::Result<Reactor> {
    let instructions = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(Cube::try_from)
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(Reactor::new(instructions))
}

fn main() -> anyhow::Result<()> {
    let reactor = parse_input(include_str!("input.txt"))?;

    dbg!(reactor.reboot(50));
    // dbg!(reactor.reboot(100_000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Cube, State, Vec3, parse_input};

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
    fn test_parse_cube_line() -> anyhow::Result<()> {
        let cube = Cube::try_from("on x=-20..26,y=-36..17,z=-47..7")?;
        assert_eq!(Vec3::new(-20, -36, -47), cube.start);
        assert_eq!(Vec3::new(26, 17, 7), cube.end);
        assert_eq!(State::On, cube.state);
        Ok(())
    }

    #[test]
    fn test_part1_example() {
        let reactor = parse_input(INPUT).expect("Failed to parse input.");
        assert_eq!(590784, reactor.reboot(50));
    }

    #[test]
    fn test_part2_example() {
        let reactor = parse_input(INPUT).expect("Failed to parse input.");
        assert_eq!(590784, reactor.reboot(100_000));
    }
}
