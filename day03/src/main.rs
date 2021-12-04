use std::{ops::Shl, fmt::{Formatter, Result, Debug}};

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
}

#[derive(Debug)]
struct BinaryList {
    pub binaries: Vec<Binary>,
    pub count: usize,
}

fn bit_is_unset(pos: usize, binary: u32) -> bool {
    binary & 1_u32.shl(pos - 1) == 0
}

fn bit_is_set(pos: usize, binary: u32) -> bool {
    binary & 1_u32.shl(pos - 1) > 0
}

impl BinaryList {
    pub fn new(binaries: Vec<Binary>, count: usize) -> Self {
        Self { binaries, count }
    }

    /// Find oxygen generator & CO2 scrubber ratings
    /// The 2nd part of the day
    pub fn find_oxygen_co2scrubber_ratings(&self) -> (u32, u32) {
        let oxygen = Self::find_rating(self.count, &self.binaries, bit_is_unset);
        let co2 = Self::find_rating(self.count - 1, &self.binaries, bit_is_set);
        (oxygen, co2)
    }

    /// Partitions the
    fn find_rating<'a, F>(position: usize, list: &[Binary], f: F) -> u32
    where
        F: Fn(usize, u32) -> bool,
    {
        println!("FIND RATING: position: {}, list: {:?}", position, list);
        let (left, right): (Vec<Binary>, Vec<Binary>) = list.iter().partition(|&&bin| f(position, bin.0));
        println!("  LEFT: {:?}", left);
        println!("  RIGHT: {:?}", right);

        if left.len() <= right.len() {
            if right.len() == 1 {
                return right[0].0;
            }
            Self::find_rating(position - 1, &right, f)
        } else {
            if left.len() == 1 {
                return left[0].0;
            }
            Self::find_rating(position - 1, &left, f)
        }
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

        let epsilon = !gamma & (1_u32.shl(self.count) - 1);

        (gamma, epsilon)
    }

    /// Counts all zeros/ones for all positions, starting from highest bit
    pub fn count(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for pos in (0..self.count).rev() {
            let (zeros, ones) = self.group_binaries(pos);
            result.push((zeros.len(), ones.len()));
        }

        result
    }

    /// Group binaries by ones & zeros at given position.
    /// Returns tuple of (zeros, ones)
    fn group_binaries(&self, position: usize) -> (Vec<Binary>, Vec<Binary>) {
        self.binaries
            .iter()
            .partition(|&bin| bin.0 & 1_u32.shl(position) == 0)
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
