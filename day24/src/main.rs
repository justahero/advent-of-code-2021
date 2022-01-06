use anyhow::anyhow;
use itertools::Itertools;

trait Applicable {
    fn apply(&self, alu: &mut ALU);
}

peg::parser! {
    grammar line_parser() for str {
        rule variable() -> Variable
            = var:$(['w' | 'x' | 'y' | 'z']) { var.into() }

        rule number() -> Variable
            = n:$(['-']* ['0'..='9']+) { Variable::Number(n.parse().unwrap()) }

        rule input() -> Instruction
            = "inp " var:variable() { Instruction::Input(var) }

        rule add() -> Instruction
            = "add " a:variable() " " b:number() {
                Instruction::Add(Add { a, b })
            }

        rule mul() -> Instruction
            = "mul " a:variable() " " b:number() {
                Instruction::Mul(Mul { a, b })
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
    Register(usize),
    Number(i32),
}

impl From<&str> for Variable {
    fn from(s: &str) -> Self {
        match s {
            "w" => Variable::Register(0),
            "x" => Variable::Register(1),
            "y" => Variable::Register(2),
            "z" => Variable::Register(3),
            n => Variable::Number(n.parse::<i32>().expect("Failed to parse number")),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Add {
    pub a: Variable,
    pub b: Variable,
}

impl Applicable for Add {
    fn apply(&self, alu: &mut ALU) {
        // alu.variables[usize::from(self.a)] += b
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct Mul {
    pub a: Variable,
    pub b: Variable,
}

impl Applicable for Mul {
    fn apply(&self, alu: &mut ALU) {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Input(Variable),
    Add(Add),
    Mul(Mul),
}

impl Applicable for Instruction {
    fn apply(&self, alu: &mut ALU) {
        match self {
            Instruction::Input(variable) => alu.set(variable),
            Instruction::Add(add) => add.apply(alu),
            Instruction::Mul(mul) => mul.apply(alu),
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        line_parser::instruction(line).map_err(|e| anyhow!("Failed to parse '{}'", e))
    }
}

impl Instruction {
    pub fn is_input(&self) -> bool {
        match &self {
            Instruction::Input(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct ALU {
    pub variables: [i32; 4],
    pub input: u8,
}

impl Default for ALU {
    fn default() -> Self {
        Self { variables: [0; 4], input: 0 }
    }
}

impl ALU {
    pub fn set(&mut self, variable: &Variable) {
        // self.variables[usize::from(variable)] = self.input as i32;
    }
}

impl ALU {
    pub fn find_highest_number(instructions: &Vec<Instruction>) -> String {
        let mut blocks = Vec::new();
        for (_, group) in &instructions.into_iter().group_by(|&i| i.is_input()) {
            blocks.push(group.collect_vec());
        }

        println!("find_highest_number - instructions: {:?}", blocks);

        let mut result: Vec<Vec<(u8, ALU)>> = Vec::new();
        let mut alu = ALU::default();

        for block in blocks {
            let output: Vec<(u8, ALU)> = (1..=9)
                .into_iter()
                .map(|number| (number, alu.run(&block[..], number)))
                .filter(|(_number, alu)| alu.variables[3] != 0)
                .collect_vec();

            result.push(output);
        }

        "".to_string()
    }

    pub fn run(&self, instructions: &[&Instruction], input: u8) -> ALU {
        let mut alu = self.clone();
        alu.input = input;

        for instruction in instructions {

        }

        alu
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
    let instructions = parse_input(include_str!("input.txt"))?;

    dbg!(ALU::find_highest_number(&instructions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, Instruction, Mul, Variable};

    #[test]
    fn test_parse_input() {
        let input = r#"
            inp x
            mul x -1
        "#;
        let instructions = parse_input(input).expect("Failed to parse.");
        assert_eq!(
            vec![
                Instruction::Input(Variable::Register(1)),
                Instruction::Mul(Mul {
                    a: Variable::Register(1),
                    b: Variable::Number(-1),
                }),
            ],
            instructions,
        );
    }

    #[test]
    fn test_run_instructions() {
        let input = r#"
            inp z
            inp x
            mul z 3
            eql z x
        "#;
    }
}
