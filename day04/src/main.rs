use anyhow::anyhow;

#[derive(Debug)]
struct Board {
    
}

#[derive(Debug)]
struct BingoSubsystem {

}

impl BingoSubsystem {
    pub fn new() -> Self {
        Self {}
    }
}

/// Parses the input, the format is structured as follows
///
/// * first line contains the Bingo numbers
/// * an empty line separates the Bingo boards from each other
/// * each board contains of 5x5 numbers
fn parse_input(input: &str) -> anyhow::Result<BingoSubsystem> {
    let blocks = input
        .split("\n\n")
        .map(str::trim)
        .collect::<Vec<_>>();

    println!("BLOCKS: {:?}", blocks);

    Ok(BingoSubsystem::new())
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
    "#;

    #[test]
    fn parses_input_successfully() {
        let bingo = parse_input(INPUT).expect("Failed to parse input.");
    }
}
