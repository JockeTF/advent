use std::{num::ParseIntError, str::FromStr, collections::HashSet, mem::replace};

#[cfg(test)]
mod tests;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug)]
enum ParseError {
    InvalidLine(String),
    InvalidValue(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Clone, Debug)]
struct Machine {
    accumulator: i32,
    instruction: i32,
    program: Vec<Instruction>,
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        ParseError::InvalidValue(value.to_string())
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split(' ').collect::<Vec<_>>();

        use Instruction::*;
        use ParseError::*;

        match split[..] {
            ["acc", val] => Ok(Acc(val.parse()?)),
            ["jmp", val] => Ok(Jmp(val.parse()?)),
            ["nop", val] => Ok(Nop(val.parse()?)),
            _ => Err(InvalidLine(s.into())),
        }
    }
}

impl FromStr for Machine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let program = s
            .trim()
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Machine {
            accumulator: 0,
            instruction: 0,
            program,
        })
    }
}

impl Machine {
    fn step(&mut self) -> Option<()> {
        use Instruction::*;

        let pc = usize::try_from(self.instruction).ok()?;
        let op = self.program.get(pc)?;

        match op {
            Acc(num) => {
                self.accumulator += num;
                self.instruction += 1;
            }
            Jmp(num) => self.instruction += num,
            Nop(_) => self.instruction += 1,
        };

        Some(())
    }

    fn swap(&mut self, pc: usize) -> Option<Instruction> {
        use Instruction::*;

        let repl = match self.program.get(pc)? {
            Jmp(val) => Nop(*val),
            Nop(val) => Jmp(*val),
            Acc(val) => Acc(*val),
        };

        Some(replace(&mut self.program[pc], repl))
    }

    fn run(&mut self) -> (bool, i32, HashSet<i32>) {
        let mut seen = HashSet::with_capacity(self.program.len());

        while self.step().is_some() {
            if !seen.insert(self.instruction) {
                return (true, self.accumulator, seen);
            }
        }

        (false, self.accumulator, seen)
    }
}

fn main() {
    let template = Machine::from_str(INPUT).unwrap();
    let mut machine = template.clone();
    let (looped, acc, seen) = machine.run();

    println!("--- Loop  Check ---");
    println!(" Accumulator: {acc}");
    println!("      Looped: {looped}");

    for instruction in seen.iter() {
        let mut machine = template.clone();
        machine.swap(*instruction as usize);
        let (looped, acc, _) = machine.run();

        if !looped {
            println!("--- Loop  Break ---");
            println!(" Accumulator: {acc}");
            println!("     Swapped: {instruction}");
            break;
        }
    }
}
