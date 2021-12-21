#[derive(Debug)]
enum Value {
    Literal(u8),
    Pair(Pair),
}

#[derive(Debug)]
struct Pair {
    pub left: Box<Value>,
    pub right: Box<Value>,
}

impl TryFrom<&str> for Pair {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Pair>> {
    input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(Pair::try_from)
        .collect::<anyhow::Result<Vec<_>>>()
}

fn main() -> anyhow::Result<()> {
    let input = parse_input(include_str!("input.txt"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Pair;

    #[test]
    fn check_parse_input() {

    }

    #[test]
    fn can_parse_pairs() {
        assert!(Pair::try_from("[1,2]").is_ok());
        assert!(Pair::try_from("[[1,2],3]").is_ok());
        assert!(Pair::try_from("[9,[8,7]]").is_ok());
        assert!(Pair::try_from("[[1,9],[8,5]]").is_ok());
        assert!(Pair::try_from("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").is_ok());
        assert!(Pair::try_from("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]").is_ok());
        assert!(Pair::try_from("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]").is_ok());
    }
}
