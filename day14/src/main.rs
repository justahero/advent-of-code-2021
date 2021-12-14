use std::collections::{HashMap, hash_map::Entry};

use itertools::Itertools;

#[derive(Debug)]
struct Polymer {
    pub template: String,
    // pub pairs: Vec<Rule>,
    pub rules: HashMap<String, char>,
}

impl Polymer {
    pub fn new(template: &str, rules: &Vec<(String, char)>) -> Self {
        let rules = rules.iter().cloned().collect::<HashMap<String, char>>();
        Self { template: template.to_string(), rules }
    }

    /// Processes a single step, creates a resulting string with all insertions
    /// after a single step.
    pub fn step(&self) -> String {
        let input = &self.template;
        let mut rules = self.rules.clone();

        let mut result = String::from(&input[..1]);
        for i in 1..input.len() {
            let s = &String::from(&input[(i-1)..=i]);
            if let Entry::Occupied(entry) = rules.entry(s.clone()) {
                result.push(*entry.get());
            }
            result.push(s.chars().last().unwrap());
        }

        result
    }
}

fn parse_input(input: &str) -> Polymer {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    let template = lines[0];
    let rules = lines[1..].iter().map(|&line| {
        let (rule, c) = line.split_once(" -> ").expect("Failed to parse");
        (rule.to_string(), c.chars().next().unwrap())
    }).collect_vec();

    Polymer::new(template, &rules)
}

fn main() {
    let _polymer = parse_input(include_str!("input.txt"));
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
        assert_eq!(String::from("NCNBCHB"), input.step());
    }
}
