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
                    min_x: x.0,
                    max_x: x.1,
                    min_y: y.0,
                    max_y: y.1,
                    min_z: z.0,
                    max_z: z.1,
                    state,
                }
            }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    On,
    Off,
}

impl From<&str> for State {
    fn from(val: &str) -> Self {
        match val {
            "on" => State::On,
            "off" => State::Off,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Cube {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_z: i32,
    pub max_z: i32,
    pub state: State,
}

impl TryFrom<&str> for Cube {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        line_parser::cube(line).map_err(|e| anyhow!("Failed to parse line '{}'", e))
    }
}

#[derive(Debug)]
struct Reactor {
    pub cubes: Vec<Cube>,
}

impl Reactor {
    pub fn new(cubes: Vec<Cube>) -> Self {
        Self { cubes }
    }

    pub fn count(&self) -> usize {
        0
    }
}

fn parse_input(input: &str) -> anyhow::Result<Reactor> {
    let cubes = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(Cube::try_from)
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(Reactor::new(cubes))
}

fn main() -> anyhow::Result<()> {
    let reactor = parse_input(include_str!("input.txt"))?;
    dbg!(reactor.count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Cube, State, parse_input};

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
        assert_eq!(-20, cube.min_x);
        assert_eq!(26, cube.max_x);
        assert_eq!(-36, cube.min_y);
        assert_eq!(17, cube.max_y);
        assert_eq!(-47, cube.min_z);
        assert_eq!(7, cube.max_z);
        assert_eq!(State::On, cube.state);
        Ok(())
    }

    #[test]
    fn test_initialisation() {
        let reactor = parse_input(INPUT).expect("Failed to parse input.");
        assert_eq!(590784, reactor.count());
    }
}
