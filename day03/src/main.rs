use std::{
    fmt::{Debug, Formatter, Result},
    ops::Shl,
};

use anyhow::anyhow;

#[derive(Clone, Copy)]
struct Binary(u32);

impl Debug for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:05b}", self.0)
    }
}

impl Binary {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn is_bit_zero(&self, position: usize) -> bool {
        self.0 & 1_u32.shl(position - 1) == 0
    }
}

#[derive(Debug)]
struct BinaryList {
    pub binaries: Vec<Binary>,
    pub count: usize,
}

fn most_common<'a>(zeros: Vec<Binary>, ones: Vec<Binary>) -> Vec<Binary> {
    if ones.len() >= zeros.len() {
        ones
    } else {
        zeros
    }
}

fn least_common<'a>(zeros: Vec<Binary>, ones: Vec<Binary>) -> Vec<Binary> {
    if ones.len() < zeros.len() {
        ones
    } else {
        zeros
    }
}

impl BinaryList {
    pub fn new(binaries: Vec<Binary>, count: usize) -> Self {
        Self { binaries, count }
    }

    /// Find oxygen generator & CO2 scrubber ratings
    /// The 2nd part of the day
    pub fn find_oxygen_co2scrubber_ratings(&self) -> (u32, u32) {
        let oxygen = Self::find_rating(self.count, &self.binaries, most_common);
        let co2 = Self::find_rating(self.count, &self.binaries, least_common);
        (oxygen, co2)
    }

    fn find_rating<'a, F>(position: usize, binaries: &[Binary], cmp_fn: F) -> u32
    where
        F: Fn(Vec<Binary>, Vec<Binary>) -> Vec<Binary>,
    {
        // a bit wonky
        if binaries.len() == 1 || position == 0 {
            return binaries[0].0;
        }

        let (zeros, ones) = Self::group_binaries(&binaries, position);

        Self::find_rating(position - 1, &cmp_fn(zeros, ones), cmp_fn)
    }

    /// Determines gamma & epsilon ratings
    pub fn find_gama_epsilon_ratings(&self) -> (u32, u32) {
        let gamma = self
            .count()
            .iter()
            .enumerate()
            .filter(|(_pos, (zeros, ones))| ones > zeros)
            .fold(0_u32, |gamma, (pos, _)| {
                gamma + 1_u32.shl(self.count - pos - 1)
            });

        // mask out epsilon rate
        let epsilon = !gamma & (1_u32.shl(self.count) - 1);

        (gamma, epsilon)
    }

    /// Counts all zeros/ones for all positions, starting from highest bit
    pub fn count(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for pos in (0..self.count).rev() {
            let (zeros, ones) = Self::group_binaries(&self.binaries, pos);
            result.push((zeros.len(), ones.len()));
        }

        result
    }

    /// Group binaries by ones & zeros at given position.
    /// Returns tuple of (zeros, ones)
    fn group_binaries(binaries: &[Binary], position: usize) -> (Vec<Binary>, Vec<Binary>) {
        binaries
            .iter()
            .partition(|&bin| bin.is_bit_zero(position))
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
        .filter_map(|line| u32::from_str_radix(line, 2).ok())
        .map(Binary::new)
        .collect::<Vec<_>>();

    let count = lines
        .first()
        .ok_or(anyhow!("Failed to get first element."))?
        .len();

    Ok(BinaryList::new(binaries, count))
}

fn main() -> anyhow::Result<()> {
    let input = parse_input(include_str!("input.txt"))?;

    let (gamma, epsilon) = input.find_gama_epsilon_ratings();
    dbg!(gamma * epsilon);

    let (oxygen, co2) = input.find_oxygen_co2scrubber_ratings();
    dbg!(oxygen * co2);

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
        assert_eq!(vec![(5, 7), (7, 5), (4, 8), (5, 7), (7, 5),], list,);
    }

    #[test]
    fn find_gamma_epsilon_ratings() {
        let binary_list = parse_input(INPUT).expect("Failed to parse input.");
        let (gamma, epsilon) = binary_list.find_gama_epsilon_ratings();
        assert_eq!(22, gamma);
        assert_eq!(9, epsilon);
    }

    #[test]
    fn find_oxygen_rating() {
        let binary_list = parse_input(INPUT).expect("Failed to parse input.");
        let (oxygen, co2) = binary_list.find_oxygen_co2scrubber_ratings();
        assert_eq!(23, oxygen);
        assert_eq!(10, co2);
    }
}
