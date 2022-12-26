use std::collections::HashSet;
use std::fmt::Display;
use std::mem::replace;
use std::num::ParseIntError;
use std::str::FromStr;

#[cfg(test)]
mod tests;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug)]
enum ParseError {
    InvalidLine(String),
    InvalidValue(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Clone, Debug)]
struct Program(Vec<Instruction>);

#[derive(Copy, Clone, Debug)]
enum State {
    Exiting,
    Looping,
    Running,
}

#[derive(Clone, Debug)]
struct Machine {
    ac: i32,
    pc: i32,
    program: Program,
    seen: HashSet<i32>,
    state: State,
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

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ins = s
            .trim()
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program(ins))
    }
}

impl Program {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn get(&self, index: impl TryInto<usize>) -> Option<&Instruction> {
        self.0.get(index.try_into().ok()?)
    }

    fn swap(&mut self, index: impl TryInto<usize>, repl: Instruction) -> Option<Instruction> {
        let index = index.try_into().ok()?;
        let target = self.0.get_mut(index)?;

        Some(replace(target, repl))
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Machine({:?}[{}]: {})", self.state, self.pc, self.ac)
    }
}

impl From<Program> for Machine {
    fn from(value: Program) -> Self {
        let capacity = value.len();

        Machine {
            ac: 0,
            pc: 0,
            program: value,
            seen: HashSet::with_capacity(capacity),
            state: State::Running,
        }
    }
}

impl Machine {
    fn step(&mut self) -> State {
        use Instruction::*;

        let Some(op) = self.program.get(self.pc) else {
            self.state = State::Exiting;
            return self.state;
        };

        match op {
            Acc(num) => {
                self.ac += num;
                self.pc += 1;
            }
            Jmp(num) => self.pc += num,
            Nop(_) => self.pc += 1,
        };

        if self.seen.insert(self.pc) {
            self.state = State::Running;
        } else {
            self.state = State::Looping;
        }

        self.state
    }

    fn run(&mut self) -> State {
        while matches!(self.step(), State::Running) {}
        self.state
    }
}

fn main() {
    use Instruction::*;

    let program = Program::from_str(INPUT).unwrap();
    let mut machine = Machine::from(program.clone());

    if !matches!(machine.run(), State::Looping) {
        panic!("Initial program never looped");
    }

    println!("Original: {machine}");

    for num in machine.seen.clone() {
        let repl = match program.get(num) {
            None => panic!("Invalid run"),
            Some(Jmp(val)) => Nop(*val),
            Some(Nop(val)) => Jmp(*val),
            Some(_) => continue,
        };

        let mut trial = program.clone();
        trial.swap(num, repl).unwrap();
        machine = Machine::from(trial);

        if matches!(machine.run(), State::Exiting) {
            println!("Fix[{num}]: {machine}");
            return;
        }
    }

    panic!("Could not repair program");
}
