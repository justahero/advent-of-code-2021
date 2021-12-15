use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;

#[derive(Debug)]
struct Polymer {
    pub template: String,
    // pub pairs: Vec<Rule>,
    pub rules: HashMap<String, char>,
}

impl Polymer {
    pub fn new(template: &str, rules: &[(String, char)]) -> Self {
        let rules = rules.iter().cloned().collect::<HashMap<String, char>>();
        Self {
            template: template.to_string(),
            rules,
        }
    }

    /// Processes the given number of steps, creates a resulting string with all insertions
    /// after steps are processed.
    pub fn steps(&self, steps: usize) -> String {
        let mut input = self.template.clone();
        let mut rules = self.rules.clone();

        for _ in 0..steps {
            let mut result = String::from(&input[..1]);
            for i in 1..input.len() {
                let s = &String::from(&input[(i - 1)..=i]);
                if let Entry::Occupied(entry) = rules.entry(s.clone()) {
                    result.push(*entry.get());
                }
                result.push(s.chars().last().unwrap());
            }
            input = result.clone();
        }

        input
    }

    /// Runs the polymer process `steps` time, then counts the number of letter occurrences
    /// to calculate the final result:
    /// `most_common - least_common`
    pub fn calculate(&self, steps: usize) -> usize {
        let result: String = self.steps(steps);
        let mut m: HashMap<char, usize> = HashMap::new();
        for c in result.chars() {
            *m.entry(c).or_insert(0) += 1;
        }

        let (lowest, highest) = m
            .iter()
            .minmax_by_key(|&(_, len)| len)
            .into_option()
            .expect("Failed to get min max");

        highest.1 - lowest.1
    }
}

fn parse_input(input: &str) -> Polymer {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    let template = lines[0];
    let rules = lines[1..]
        .iter()
        .map(|&line| {
            let (rule, c) = line.split_once(" -> ").expect("Failed to parse");
            (rule.to_string(), c.chars().next().unwrap())
        })
        .collect_vec();

    Polymer::new(template, &rules)
}

fn main() {
    let polymer = parse_input(include_str!("input.txt"));
    dbg!(polymer.calculate(10));
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    const INPUT: &str = r#"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "#;

    #[test]
    fn check_parse_input() {
        let input = parse_input(INPUT);
        assert_eq!(String::from("NNCB"), input.template);
        assert_eq!(16, input.rules.len());
    }

    #[test]
    fn test_single_step_process() {
        let input = parse_input(INPUT);
        assert_eq!(String::from("NCNBCHB"), input.steps(1));
        assert_eq!(String::from("NBCCNBBBCBHCB"), input.steps(2));
        assert_eq!(String::from("NBBBCNCCNBBNBNBBCHBHHBCHB"), input.steps(3));
    }

    #[test]
    fn test_calculate_first_half() {
        let input = parse_input(INPUT);
        assert_eq!(1588, input.calculate(10));
    }
}
