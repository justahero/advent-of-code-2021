use std::collections::HashMap;

use itertools::Itertools;

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
    /// TODO refactor this algorithm, only calculate, dont create any strings
    ///
    pub fn steps(&self, steps: usize) -> HashMap<String, usize> {
        let mut input = self.template.chars();

        let mut pairs = HashMap::new();
        for i in 0..(steps - 1) {
            let index = format!("{}{}", input.nth(i).unwrap(), input.nth(i + 1).unwrap());
            *pairs.entry(index).or_insert(0) += 1_usize;
        }

        for step in 0..(steps - 1) {
            println!("STEP: {}", step);
            let mut pairs2 = HashMap::new();
            for (pair, count) in pairs.iter() {
                let (l, r) = pair.split_at(1);
                let c = self.rules.get(pair).unwrap();
                *pairs2.entry(format!("{}{}", l, c)).or_insert(0) += count;
                *pairs2.entry(format!("{}{}", c, r)).or_insert(0) += count;
            }
        }

        /*
        for step in 1..steps {
            let mut pairs2: HashMap<(u8, u8), usize> = HashMap::new();
            println!("STEP: {}", step);
            for (pair, count) in pairs.iter() {
                *pairs2.entry((pair.0, rules[&pair])).or_insert(0) += count;
                *pairs2.entry((rules[&pair], pair.1)).or_insert(0) += count;
            }
            pairs. = pairs2;
        }
        */

        pairs
    }

    /// Runs the polymer process `steps` time, then counts the number of letter occurrences
    /// to calculate the final result:
    /// `most_common - least_common`
    pub fn calculate(&self, steps: usize) -> usize {
        let map = self.steps(steps);
        println!("CALCULATE: {:?}", map);

        let counters = map.iter().fold(HashMap::new(), |mut result, (s, count)| {
            let mut s = s.chars();
            let l = s.next().unwrap();
            let r = s.next().unwrap();

            *result.entry(l).or_insert(0) += count;
            *result.entry(r).or_insert(0) += count;
            result
        });

        let (lowest, highest) = counters
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
        assert_eq!(1588, input.calculate(40));
    }
}
