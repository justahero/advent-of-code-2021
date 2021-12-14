
#[derive(Debug)]
struct Polymer {
    pub template: String,
    pub pairs: Vec<(String, char)>,
}

impl Polymer {
    pub fn new(template: &str) -> Self {
        Self { template: template.to_string(), pairs: Vec::new() }
    }
}

fn parse_input(input: &str) -> Polymer {
    let x = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty());
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
    }
}
