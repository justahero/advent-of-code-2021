use std::collections::HashMap;

use itertools::Itertools;

/// TODO replace String into a `Vec<u8>` ?
#[derive(Debug)]
struct Polymer {
    pub template: String,
    pub rules: HashMap<String, String>,
}

impl Polymer {
    pub fn new(template: &str, rules: &[(String, String)]) -> Self {
        let rules = rules.iter().cloned().collect::<HashMap<String, String>>();
        Self {
            template: template.to_string(),
            rules,
        }
    }

    /// Processes the given number of steps, creates a resulting string with all insertions
    /// after steps are processed.
    ///
    pub fn steps(&self, steps: usize) -> HashMap<String, usize> {
        let mut pairs: HashMap<String, usize> = HashMap::new();
        for (l, r) in self.template.chars().tuple_windows() {
            *pairs.entry(format!("{}{}", l, r)).or_insert(0) += 1_usize;
        }

        for _ in 0..steps {
            let mut pairs2 = HashMap::new();
            for (pair, count) in pairs.iter() {
                let (l, r) = pair.split_at(1);
                let c = self.rules.get(pair).unwrap();
                *pairs2.entry(format!("{}{}", l, c)).or_insert(0) += count;
                *pairs2.entry(format!("{}{}", c, r)).or_insert(0) += count;
            }

            pairs = pairs2.clone();
        }

        // Calculate single letter frequencies
        let mut counts: HashMap<String, usize> = HashMap::new();
        for (pair, count) in pairs.iter() {
            let (l, _r) = pair.split_at(1);
            *counts.entry(l.to_string()).or_insert(0) += count;
        }
        *counts.get_mut(&self.template.chars().last().unwrap().to_string()).unwrap() += 1;

        counts
    }

    /// Runs the polymer process `steps` time, then counts the number of letter occurrences
    /// to calculate the final result:
    /// `most_common - least_common`
    pub fn calculate(&self, steps: usize) -> usize {
        let map = self.steps(steps);
        println!("CALCULATE: {:?}", map);

        let (lowest, highest) = map
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
    let rules: Vec<(String, String)> = lines[1..]
        .iter()
        .map(|&line| {
            let (rule, c) = line.split_once(" -> ").expect("Failed to parse");
            (rule.to_string(), c.to_string())
        })
        .collect_vec();

    Polymer::new(template, &rules)
}

fn main() {
    let polymer = parse_input(include_str!("input.txt"));
    dbg!(polymer.calculate(10));
    dbg!(polymer.calculate(40));
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
    fn test_calculate_first_half() {
        let input = parse_input(INPUT);
        assert_eq!(1588, input.calculate(10));
    }

    #[test]
    fn test_calculate_second_half() {
        let input = parse_input(INPUT);
        assert_eq!(2188189693529, input.calculate(40));
    }
}
