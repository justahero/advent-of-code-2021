use std::ops::Shl;

use anyhow::anyhow;

#[derive(Debug)]
struct BinaryList {
    pub binaries: Vec<u32>,
    pub count: usize,
}

impl BinaryList {
    pub fn new(binaries: Vec<u32>, count: usize) -> Self {
        Self { binaries, count }
    }

    /// Determines gamma & epsilon rates
    pub fn find_rates(&self) -> (u32, u32) {
        let counts = self.count();

        let mut gamma = 0_u32;
        for (pos, (zeros, ones)) in counts.iter().enumerate() {
            if ones > zeros {
                gamma |= 1_u32.shl(self.count - pos - 1);
            }
        }

        // epsilon is the negative
        let mask = 1_u32.shl(self.count) - 1;
        let epsilon = !gamma & mask;

        (gamma, epsilon)
    }

    /// Counts all zeros/ones for all positions, starting from highest bit
    pub fn count(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for pos in (0..self.count).rev() {
            let ones = self.count_ones(pos);
            let zeros = self.binaries.len() - ones;
            result.push((zeros, ones));
        }

        result
    }

    /// Count all the ones in all binaries in given position
    fn count_ones(&self, position: usize) -> usize {
        self.binaries
            .iter()
            .filter(|&bin| bin & 1_u32.shl(position) > 0)
            .count()
    }
}

/// Parses the input, stores all binaries and number of bits
fn parse_input(input: &str) -> anyhow::Result<BinaryList> {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect::<Vec<_>>();

    let binaries = lines
        .iter()
        .map(|line| u32::from_str_radix(line, 2))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let count = lines.first().ok_or(anyhow!("Failed to get first element."))?.len();

    Ok(BinaryList::new(binaries, count))
}

fn main() -> anyhow::Result<()> {
    let input = parse_input(include_str!("input.txt"))?;
    let (gamma, epsilon) = input.find_rates();
    dbg!(gamma * epsilon);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "#;

    #[test]
    fn count_ones_and_zeros() {
        let binary_list = parse_input(INPUT).expect("Failed to parse input.");
        let list = binary_list.count();
        assert_eq!(binary_list.count, list.len());
        assert_eq!(
            vec![
                (5, 7),
                (7, 5),
                (4, 8),
                (5, 7),
                (7, 5),
            ],
            list,
        );
    }

    #[test]
    fn find_rates() {
        let binary_list = parse_input(INPUT).expect("Failed to parse input.");
        let (gamma, epsilon) = binary_list.find_rates();
        assert_eq!(22, gamma);
        assert_eq!(9, epsilon);
    }
}
