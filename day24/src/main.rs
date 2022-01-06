use std::{collections::{HashMap, VecDeque}, ops::Range};

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

#[derive(Debug, Clone)]
struct ALU {
    pub variables: [i32; 4],
}

impl ALU {
    pub fn new(zreg: i32) -> Self {
        Self { variables: [0, 0, 0, zreg] }
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

    pub fn run(&mut self, instructions: &Vec<Instruction>, inputs: &[i32]) -> i32 {
        println!("> alu::eval instructions: {}, input: {:?}", instructions.len(), inputs);

        let mut inputs = inputs.iter().cloned().collect::<VecDeque<_>>();

        for instruction in instructions.iter() {
            println!("> instruction: {:?}", instruction);

            match instruction {
                Instruction::Input(reg) => self.write(reg, inputs.pop_front().unwrap()),
                Instruction::Add(reg, b) => *self.get_mut(reg) += self.variable(b),
                Instruction::Mul(reg, b) => *self.get_mut(reg) *= self.variable(b),
                Instruction::Mod(reg, b) => *self.get_mut(reg) %= self.variable(b),
                Instruction::Div(reg, b) => *self.get_mut(reg) /= self.variable(b),
                Instruction::Equal(reg, b) => {
                    println!("  eql - a: {:?}, b: {:?}", reg, b);
                    let v = if self.read(reg) == self.variable(b) {
                        println!("  : 1");
                        1
                    } else {
                        println!("  : 0");
                        0
                    };
                    self.write(reg, v);
                }
            }
            println!("  registers: {:?}", self.variables);
        }

        self.variables[usize::from(Register::Z)]
    }
}

#[derive(Debug)]
struct Solver {
    programs: Vec<Vec<Instruction>>,
    cache: HashMap<(usize, i32), Option<i64>>,
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

        Self { programs, cache: HashMap::new() }
    }

    pub fn run(&mut self, num_digits: usize, prev_z: i32, range: Range<i32>) -> Option<i64> {
        println!("> run num_digits: {}, prev_z: {}", num_digits, prev_z);

        if num_digits >= self.num_digits() {
            if prev_z == 0 {
                return Some(0);
            }
            return None;
        }

        if let Some(&cached) = self.cache.get(&(num_digits, prev_z)) {
            return cached;
        }

        for input in range.clone() {
            let next_z = ALU::new(prev_z).run(&self.programs[num_digits], &vec![input]);
            if let Some(best_suffix) = self.run(num_digits + 1, next_z, range.clone()) {
                let exp = self.num_digits() - num_digits - 1;
                let new_suffix = 10_i64.pow(exp as u32) * input as i64 + best_suffix;

                self.cache.insert((num_digits, prev_z), Some(new_suffix));
                return Some(new_suffix);
            }
        }

        self.cache.insert((num_digits, prev_z), None);
        None
    }

    fn num_digits(&self) -> usize {
        self.programs.len()
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
    let mut solver = Solver::new(&instructions, 14);

    dbg!(solver.run(0, 0, 1..10));

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
        let mut alu = ALU::new(0);
        println!("ALU: {:?}", alu);
        assert_eq!(1, alu.run(&instructions, &[1, 3]));
    }
}
