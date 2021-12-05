use anyhow::anyhow;

#[derive(Debug, Clone)]
struct Value(u32, bool);

impl Value {
    pub fn new(v: u32) -> Self {
        Value(v, false)
    }

    /// Mark the field value as drawn
    pub fn mark(&mut self) {
        self.1 = true;
    }

    /// The value
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Returns true if value is marked
    pub fn marked(&self) -> bool {
        self.1
    }
}

#[derive(Debug, Clone)]
struct Board {
    pub fields: Vec<Value>,
}

impl Board {
    const SIDE: usize = 5;

    pub fn new(fields: Vec<u32>) -> Self {
        let fields = fields.into_iter().map(Value::new).collect::<Vec<_>>();
        Self { fields }
    }

    /// Returns all unmarked numbers
    pub fn unmarked_fields(&self) -> Vec<u32> {
        self.fields
            .iter()
            .filter(|&v| !v.marked())
            .map(Value::value)
            .collect()
    }

    /// Check the board has a row / column of complete numbers
    pub fn is_marked(&self) -> Option<Vec<u32>> {
        for y in 0..Self::SIDE {
            if let Some(row) = self.scan_row(y) {
                return Some(row);
            }
        }
        for x in 0..Self::SIDE {
            if let Some(col) = self.scan_col(x) {
                return Some(col);
            }
        }
        None
    }

    pub fn mark(&mut self, number: u32) -> bool {
        if let Some(value) = self.fields.iter_mut().find(|value| value.value() == number) {
            value.mark();
            return true;
        }
        false
    }

    /// Scans the given row and returns it when all fields were marked
    pub fn scan_row(&self, row: usize) -> Option<Vec<u32>> {
        let fields = self.fields.iter().skip(Self::SIDE * row).take(Self::SIDE);

        if fields.clone().all(Value::marked) {
            return Some(fields.map(Value::value).collect());
        }

        None
    }

    /// Scans the given col and returns it when all fields were marked
    pub fn scan_col(&self, col: usize) -> Option<Vec<u32>> {
        let fields = self.fields.iter().skip(col).step_by(Self::SIDE);

        if fields.clone().all(Value::marked) {
            return Some(fields.map(Value::value).collect());
        }

        None
    }
}

impl TryFrom<&str> for Board {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let numbers = line
            .split_ascii_whitespace()
            .map(str::trim)
            .map(|val| {
                val.parse::<u32>()
                    .map_err(|_| anyhow!("Failed to parse value."))
            })
            .collect::<Result<Vec<u32>, Self::Error>>()?;
        Ok(Board::new(numbers))
    }
}

/// The infamous Submarine BingoSubsystem
#[derive(Debug)]
struct BingoSubsystem {
    pub numbers: Vec<u32>,
    pub boards: Vec<Board>,
}

impl BingoSubsystem {
    pub fn new(numbers: Vec<u32>, boards: Vec<Board>) -> Self {
        Self { numbers, boards }
    }

    /// Iterate over all Bingo numbers and check that there is one board that wins
    pub fn play(&mut self) -> Option<(u32, Vec<u32>)> {
        for number in self.numbers.clone() {
            for board in self.boards.iter_mut() {
                if board.mark(number) && board.is_marked().is_some() {
                    return Some((number, board.unmarked_fields()));
                }
            }
        }

        None
    }

    /// Let the squid win, find the board that wins last
    pub fn play_last(&self) -> Option<(u32, Vec<u32>)> {
        let mut boards = self.boards.clone();

        for number in self.numbers.clone() {
            for board in boards.iter_mut() {
                board.mark(number);
            }

            if boards.len() == 1 {
                return Some((number, boards[0].unmarked_fields()));
            }

            boards.retain(|board| board.is_marked().is_none());
        }

        None
    }
}

/// Parses the input, the format is structured as follows
///
/// * first line contains the Bingo numbers
/// * an empty line separates the Bingo boards from each other
/// * each board contains of 5x5 numbers
fn parse_input(input: &str) -> anyhow::Result<BingoSubsystem> {
    let blocks = input.split("\n\n").map(str::trim).collect::<Vec<_>>();

    let numbers = blocks
        .first()
        .ok_or_else(|| anyhow!("No bingo numbers found."))?
        .split(',')
        .map(str::trim)
        .map(|value| value.parse::<u32>())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let boards = blocks
        .iter()
        .skip(1)
        .map(|&line| Board::try_from(line))
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    Ok(BingoSubsystem::new(numbers, boards))
}

fn main() -> anyhow::Result<()> {
    let mut system = parse_input(include_str!("input.txt"))?;

    let (number, unmarked_fields) = system.play().ok_or(anyhow!("No winning board found."))?;
    let result = number * unmarked_fields.iter().sum::<u32>();
    dbg!(result);

    // let squid win
    let (number, unmarked_fields) = system
        .play_last()
        .ok_or(anyhow!("No winning board found."))?;
    let result = number * unmarked_fields.iter().sum::<u32>();
    dbg!(result);

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
        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            bingo.numbers,
        );
        assert_eq!(3, bingo.boards.len(),);
    }

    #[test]
    fn find_winner_board() {
        let mut bingo = parse_input(INPUT).expect("Failed to parse input.");

        let result = bingo.play();
        assert!(result.is_some());
        let (number, unmarked_fields) = result.expect("Failed to get board.");
        let sum = unmarked_fields.iter().sum::<u32>();
        assert_eq!(24, number);
        assert_eq!(188, sum);
    }

    /// The 2nd board wins
    #[test]
    fn play_for_squid_to_win() {
        let bingo = parse_input(INPUT).expect("Failed to parse input.");

        let result = bingo.play_last();
        assert!(result.is_some());
        let (number, unmarked_fields) = result.expect("Failed to get board.");
        let sum = unmarked_fields.iter().sum::<u32>();
        assert_eq!(13, number);
        assert_eq!(148, sum);
    }
}
