use itertools::Itertools;

#[derive(Debug)]
struct Rule(String, char);

#[derive(Debug)]
struct Polymer {
    pub template: String,
    pub pairs: Vec<Rule>,
}

impl Polymer {
    pub fn new(template: &str, pairs: Vec<Rule>) -> Self {
        Self { template: template.to_string(), pairs }
    }
}

fn parse_input(input: &str) -> Polymer {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec();

    let template = lines[0];
    let pairs = lines[1..].iter().map(|&line| {
        let (pair, c) = line.split_once(" -> ").expect("Failed to parse");
        Rule(pair.to_string(), c.chars().next().unwrap())
    }).collect_vec();

    Polymer::new(template, pairs)
}

fn main() {
    println!("Hello, world!");
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
        assert_eq!(16, input.pairs.len());
    }

    #[test]
    fn test_single_step_process() {
    }
}
