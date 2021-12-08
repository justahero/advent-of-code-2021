use anyhow::anyhow;
use std::{fmt::Display, ops::Shl};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Digit(u16);

impl Digit {
    pub fn new(val: u16) -> Self {
        Self(val)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn set(&mut self, pos: u16) {
        assert!(pos < 7);
        self.0 |= 1_u16.shl(pos);
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
}

/// Converst segments to a digit, e.g. `cf` -> 0b100100
impl From<&str> for Digit {
    fn from(val: &str) -> Self {
        val.chars().fold(Digit::empty(), |mut digit, char| {
            digit.set((char as u32 - 'a' as u32) as u16);
            digit
        })
    }
}

struct DisplayLine {
    pub segments: Vec<Digit>,
    pub digits: Vec<Digit>,
}

impl DisplayLine {
    pub fn new(segments: Vec<Digit>, digits: Vec<Digit>) -> Self {
        Self { segments, digits }
    }

    /// Returns the easily detectable digits: 1, 4, 7 or 8
    pub fn count_easy_digits(&self) -> usize {
        self.digits
            .iter()
            .filter(|&digit| [2, 3, 4, 7].contains(&digit.count_ones()))
            .count()
    }
}

impl From<&str> for DisplayLine {
    fn from(line: &str) -> Self {
        let (segments, digits) = line.split_once('|').expect("Failed to split line");
        let segments = segments.split_whitespace().map(Digit::from).collect_vec();
        let digits = digits.split_whitespace().map(Digit::from).collect_vec();
        Self { segments, digits }
    }
}

struct DisplayNotes {
    pub lines: Vec<DisplayLine>,
}

impl DisplayNotes {
    pub fn new(lines: Vec<DisplayLine>) -> Self {
        Self { lines }
    }

    pub fn count_easy_digits(&self) -> usize {
        self.lines.iter().map(|line| line.count_easy_digits()).sum::<usize>()
    }
}

fn parse_input(input: &str) -> DisplayNotes {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(DisplayLine::from)
        .collect_vec();
    DisplayNotes::new(lines)
}

fn main() {
    let notes = parse_input(include_str!("input.txt"));

    dbg!(notes.count_easy_digits());
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, Digit};

    const INPUT: &str = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "#;

    #[test]
    fn parses_digit_from_string() {
        assert_eq!(Digit::new(0b10010), "be".into());
        assert_eq!(Digit::new(0b1111111), "abcdefg".into());
    }

    #[test]
    fn parses_first_input_line() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let input = &parse_input(input).lines[0];
        assert_eq!(
            vec![
                Digit::new(0b0010010),
                Digit::new(0b1111111),
                Digit::new(0b1111110),
                Digit::new(0b1111101),
                Digit::new(0b1010110),
                Digit::new(0b1111100),
                Digit::new(0b1111011),
                Digit::new(0b0111110),
                Digit::new(0b0101111),
                Digit::new(0b0011010),
            ],
            input.segments,
        );
        assert_eq!(2, input.count_easy_digits());
    }

    #[test]
    fn count_easy_digits_from_input() {
        let lines = parse_input(INPUT);
        assert_eq!(26, lines.count_easy_digits());
    }
}
