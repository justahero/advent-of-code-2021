use std::{
    collections::HashMap,
    ops::Shl,
};

use itertools::Itertools;

struct DisplayLine {
    pub segments: Vec<u16>,
    pub digits: Vec<u16>,
}

fn parse_binary(val: &str) -> u16 {
    val.chars().fold(0_u16, |mut result, char| {
        let pos = (char as u32 - 'a' as u32) as u16;
        result |= 1_u16.shl(pos);
        result
    })
}

/// Calculates the difference in bits.
fn diff(lhs: u16, rhs: u16) -> u16 {
    let mut lhs = lhs.clone();
    for bit_pos in 0..=6_u16 {
        if rhs & 1_u16.shl(bit_pos) > 0 {
            lhs &= !1_u16.shl(bit_pos);
        }
    }
    lhs
}

impl DisplayLine {
    /// Returns the easily detectable digits: 1, 4, 7 or 8
    pub fn count_easy_digits(&self) -> usize {
        self.digits
            .iter()
            .filter(|&digit| [2, 3, 4, 7].contains(&digit.count_ones()))
            .count()
    }

    /// Returns the four digit value of this display, deduced / analyzed by the given segments
    /// Segments are analyzed as follows:
    ///
    /// ```
    ///  0000
    /// 1    2
    /// 1    2
    ///  3333
    /// 4    5
    /// 4    5
    ///  6666
    /// ```
    ///
    /// Analyzing number of occurences of segments in all digits
    /// * `0` appears 8 times
    /// * `1` appears 6 times
    /// * `2` appears 8 times
    /// * `3` appears 7 times
    /// * `4` appears 4 times
    /// * `5` appears 9 times
    /// * `6` appears 7 times
    ///
    pub fn deduce_digits(&self) -> u32 {
        // let table = self.segments.iter().map(|digit| (digit.count_ones(), digit)).collect::<HashMap<_, Vec<_>>>();
        let mut table = self
            .segments
            .iter()
            .fold(HashMap::new(), |mut table, digit| {
                table
                    .entry(digit.count_ones())
                    .or_insert(Vec::new())
                    .push(digit.clone());
                table
            });

        let one = table.remove(&2).unwrap()[0];
        let four = table.remove(&4).unwrap()[0];
        let seven = table.remove(&3).unwrap()[0];
        let eight = table.remove(&7).unwrap()[0];
        let (index, &three) = table[&5]
            .iter()
            .find_position(|&&digit| (diff(digit, seven)).count_ones() == 2)
            .unwrap();
        if let Some(digits) = table.get_mut(&5) {
            digits.remove(index);
        }
        let (index, &nine) = table[&6]
            .iter()
            .find_position(|&&digit| (three ^ digit).count_ones() == 1)
            .unwrap();
        if let Some(digits) = table.get_mut(&6) {
            digits.remove(index);
        }
        let (index, &six) = table[&6]
            .iter()
            .find_position(|&&digit| (one & (diff(eight, digit))).count_ones() == 1)
            .unwrap();
        if let Some(digits) = table.get_mut(&6) {
            digits.remove(index);
        }
        let zero = table.remove(&6).unwrap()[0];
        let (index, &five) = table[&5]
            .iter()
            .find_position(|&&digit| (six - digit).count_ones() == 1)
            .unwrap();
        if let Some(digits) = table.get_mut(&5) {
            digits.remove(index);
        }
        let two = table.remove(&5).unwrap()[0];

        // hacky version to get final sum
        let list = vec![zero, one, two, three, four, five, six, seven, eight, nine];
        let four_digits = self
            .digits
            .iter()
            .map(|digit| {
                let (val, _) = list.iter().find_position(|&val| val == digit).unwrap();
                val.to_string()
            })
            .join("")
            .parse::<u32>()
            .unwrap();

        four_digits
    }
}

impl From<&str> for DisplayLine {
    fn from(line: &str) -> Self {
        let (segments, digits) = line.split_once('|').expect("Failed to split line");
        let segments = segments.split_whitespace().map(parse_binary).collect_vec();
        let digits = digits.split_whitespace().map(parse_binary).collect_vec();
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
        self.lines
            .iter()
            .map(|line| line.count_easy_digits())
            .sum::<usize>()
    }

    pub fn count_deduced_digits(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| line.deduce_digits())
            .sum::<u32>()
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
    dbg!(notes.count_deduced_digits());
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{DisplayLine, parse_binary, parse_input};

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
        assert_eq!(0b10010, parse_binary("be"));
        assert_eq!(0b1111111, parse_binary("abcdefg"));
    }

    #[test]
    fn parses_first_input_line() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let input = &parse_input(input).lines[0];
        assert_eq!(
            vec![
                0b0010010, 0b1111111, 0b1111110, 0b1111101, 0b1010110, 0b1111100, 0b1111011,
                0b0111110, 0b0101111, 0b0011010,
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

    #[test]
    fn deduces_four_digit_value() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let line = DisplayLine::from(input);
        assert_eq!(5353, line.deduce_digits());
    }

    #[test]
    fn count_deduced_digits() {
        let lines = parse_input(INPUT);
        assert_eq!(61229, lines.count_deduced_digits());
    }

    #[test]
    fn count_all_deduced_digits() {
        let lines = parse_input(INPUT);
        let lines = lines
            .lines
            .iter()
            .map(|line| line.deduce_digits())
            .collect_vec();
        assert_eq!(
            vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315,],
            lines,
        );
    }
}
