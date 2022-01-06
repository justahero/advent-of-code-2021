use anyhow::anyhow;

peg::parser! {
    grammar line_parser() for str {
        rule variable() -> Variable
            = var:$(['w' | 'x' | 'y' | 'z']) { var.into() }

        rule number() -> i32
            = n:$(['-']* ['0'..='9']+) { n.parse().unwrap() }

        rule input() -> Instruction
            = "inp " var:variable() { Instruction::Input(var) }

        rule add() -> Instruction
            = "add " var:variable() " " value:number() {
                Instruction::Add(Add { a: var, b: value })
            }

        rule mul() -> Instruction
            = "mul " var:variable() " " value:number() {
                Instruction::Mul(Mul { a: var, b: value })
            }

        pub(crate) rule instruction() -> Instruction
            = instruction:input()
            / instruction:add()
            / instruction:mul() {
                instruction
            }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Variable {
    W = 0,
    X,
    Y,
    Z,
}

impl From<&str> for Variable {
    fn from(s: &str) -> Self {
        match s {
            "w" => Variable::W,
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            _ => unreachable!(),
        }
    }
}

impl From<Variable> for usize {
    fn from(v: Variable) -> Self {
        match v {
            Variable::W => 0,
            Variable::X => 1,
            Variable::Y => 2,
            Variable::Z => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Add {
    pub a: Variable,
    pub b: i32,
}

#[derive(Debug, PartialEq)]
struct Mul {
    pub a: Variable,
    pub b: i32,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Input(Variable),
    Add(Add),
    Mul(Mul),
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        line_parser::instruction(line).map_err(|e| anyhow!("Failed to parse '{}'", e))
    }
}

#[derive(Debug)]
struct ALU {
    pub variables: [i32; 4],
}

impl ALU {
    pub fn new() -> Self {
        Self { variables: [0; 4] }
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let instructions = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(Instruction::try_from)
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(instructions)
}

fn main() -> anyhow::Result<()> {
    let instructions =  parse_input(include_str!("input.txt"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Mul, Variable, parse_input};

    #[test]
    fn test_parse_input() {
        let input = r#"
            inp x
            mul x -1
        "#;
        let instructions = parse_input(input).expect("Failed to parse.");
        assert_eq!(
            vec![
                Instruction::Input(Variable::X),
                Instruction::Mul(Mul{ a: Variable::X, b: -1 }),
            ],
            instructions,
        );
    }
}
