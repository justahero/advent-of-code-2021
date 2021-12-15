use std::collections::HashMap;

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
    ///
    /// TODO refactor this algorithm, only calculate, dont create any strings
    ///
    pub fn steps(&self, steps: usize) -> HashMap<char, usize> {
        let input = self.template.clone();
        let rules = self.rules.clone();

        // let mut map: HashMap<char, usize> = input.chars().map(|c| (c, 0)).collect();
        /*
        let mut map: HashMap<char, usize> = input.chars().fold(HashMap::new(), |mut result, c| {
            *result.entry(c).or_insert(0) += 1;
            result
        });
        */

        let mut map: HashMap<char, usize> = HashMap::new();
        for i in 1..input.len() {
            let text = &input[(i-1)..=i];
            println!("FIND {}", text);
            self.insert(steps, text, &rules, &mut map);
        }

        map
    }

    fn insert(&self, step: usize, input: &str, rules: &HashMap<String, char>, map: &mut HashMap<char, usize>) {
        // put all occurrences of given chars to map
        if step == 0 {
            // println!("INSERT {}", input);
            for c in input.chars() {
                *map.entry(c).or_insert(0) += 1;
            }
            return;
        }

        // check if there is a rule for the given input, then recurse down one step
        if let Some(c) = rules.get(&input.to_string()) {
            let (l, r) = input.split_at(1);
            self.insert(step - 1, &format!("{}{}", l, c), rules, map);
            self.insert(step - 1, &format!("{}{}", c, r), rules, map);
        }
    }

    /// Runs the polymer process `steps` time, then counts the number of letter occurrences
    /// to calculate the final result:
    /// `most_common - least_common`
    pub fn calculate(&self, steps: usize) -> usize {
        let map = self.steps(steps);
        println!("CALCULATE MAP: {:?}", map);

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
    fn test_calculate_first_half() {
        let input = parse_input(INPUT);
        assert_eq!(1588, input.calculate(40));
    }
}
