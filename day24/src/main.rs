use std::ops::Range;

use anyhow::anyhow;
use itertools::Itertools;

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

        rule modulo() -> Instruction
            = "mod " a:register() " " b:variable() {
                Instruction::Mod(a, b)
            }

        rule div() -> Instruction
            = "div " a:register() " " b:variable() {
                Instruction::Div(a, b)
            }

        rule equal() -> Instruction
            = "eql " a:register() " " b:variable() {
                Instruction::Equal(a, b)
            }

        pub(crate) rule instruction() -> Instruction
            = instruction:input()
            / instruction:add()
            / instruction:mul()
            / instruction:modulo()
            / instruction:div()
            / instruction:equal() {
                instruction
            }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl From<Register> for usize {
    fn from(reg: Register) -> Self {
        match reg {
            Register::W => 0,
            Register::X => 1,
            Register::Y => 2,
            Register::Z => 3,
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Input(Register),
    Add(Register, Variable),
    Mul(Register, Variable),
    Div(Register, Variable),
    Mod(Register, Variable),
    Equal(Register, Variable),
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
}

impl Default for ALU {
    fn default() -> Self {
        Self {
            variables: [0; 4],
        }
    }
}

impl ALU {
    /// Reads the given register value
    pub fn read(&self, reg: &Register) -> i32 {
        self.variables[usize::from(*reg)]
    }

    fn get_mut(&mut self, reg: &Register) -> &mut i32 {
        &mut self.variables[usize::from(*reg)]
    }

    fn write(&mut self, reg: &Register, value: i32) {
        self.variables[usize::from(*reg)] = value;
    }

    fn variable(&self, variable: &Variable) -> i32 {
        match variable {
            Variable::Register(reg) => self.read(reg),
            Variable::Number(value) => *value,
        }
    }

    pub fn eval(&mut self, instructions: &Vec<Instruction>, inputs: &[i32]) -> i32 {
        println!("> alu::eval instructions: {}, input: {:?}", instructions.len(), inputs);

        let mut inputs = inputs.iter().cloned().collect_vec();

        for instruction in instructions {
            match instruction {
                Instruction::Input(reg) => *self.get_mut(reg) = inputs.pop().unwrap(),
                Instruction::Add(a, b) => *self.get_mut(a) += self.variable(b),
                Instruction::Mul(a, b) => *self.get_mut(a) *= self.variable(b),
                Instruction::Mod(a, b) => *self.get_mut(a) %= self.variable(b),
                Instruction::Div(a, b) => *self.get_mut(a) /= self.variable(b),
                Instruction::Equal(a, b) => {
                    let v = if self.read(a) == self.variable(b) {
                        1
                    } else {
                        0
                    };
                    self.write(a, v);
                }
            }
        }

        self.variables[usize::from(Register::Z)]
    }
}

struct Solver {
    pub programs: Vec<Vec<Instruction>>,
}

impl Solver {
    pub fn new(instructions: &Vec<Instruction>, num_digits: usize) -> Self {
        let chunk = instructions.len() / num_digits;
        let programs = instructions
            .iter()
            .cloned()
            .chunks(chunk)
            .into_iter()
            .map(|program| program.collect_vec())
            .collect_vec();

        Self { programs }
    }

    pub fn run(&self, numbers: Range<i32>) -> i32 {
        0
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
    let solver = Solver::new(&instructions, 14);

    dbg!(solver.run(1..10));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{ALU, Instruction, Register, Variable, parse_input};

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
        let instructions = parse_input(input).unwrap();
        let mut alu = ALU::default();
        assert_eq!(1, alu.eval(&instructions, &[1, 3]));
    }
}
