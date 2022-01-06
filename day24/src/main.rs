use anyhow::anyhow;
use itertools::Itertools;

trait Applicable {
    fn apply(&self, alu: &mut ALU);
}

peg::parser! {
    grammar line_parser() for str {
        rule register() -> Register
            = reg:$(['w' | 'x' | 'y' | 'z']) { Register::from(reg) }

        rule variable_register() -> Variable
            = reg:register() { Variable::Register(Register::from(reg)) }

        rule number() -> Variable
            = n:$(['-']* ['0'..='9']+) { Variable::Number(n.parse().unwrap()) }

        rule variable() -> Variable
            = r:variable_register() / r:number() { Variable::from(r) }

        rule input() -> Instruction
            = "inp " reg:register() { Instruction::Input(reg) }

        rule add() -> Instruction
            = "add " a:register() " " b:variable() {
                Instruction::Add(a, b)
            }

        rule mul() -> Instruction
            = "mul " a:register() " " b:variable() {
                Instruction::Mul(a, b)
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
enum Register {
    W,
    X,
    Y,
    Z,
}

impl From<&str> for Register {
    fn from(s: &str) -> Self {
        match s {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => panic!("Invalid register."),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Variable {
    Register(Register),
    Number(i32),
}

impl From<i32> for Variable {
    fn from(v: i32) -> Self {
        Self::Number(v)
    }
}

impl From<Register> for Variable {
    fn from(reg: Register) -> Self {
        Self::Register(reg)
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Input(Register),
    Add(Register, Variable),
    Mul(Register, Variable),
    Mod(Register, Variable),
}

impl Applicable for Instruction {
    fn apply(&self, alu: &mut ALU) {
        match self {
            Instruction::Input(variable) => (),
            Instruction::Add(a, b) => (),
            Instruction::Mul(a, b) => (),
            Instruction::Mod(a, b) => (),
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
    use crate::{Instruction, Register, Variable, parse_input};

    #[test]
    fn test_parse_input() {
        let input = r#"
            inp x
            mul x -1
        "#;
        let instructions = parse_input(input).expect("Failed to parse.");
        assert_eq!(
            vec![
                Instruction::Input(Register::X),
                Instruction::Mul(Register::X, Variable::Number(-1)),
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
