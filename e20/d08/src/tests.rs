use std::str::FromStr;

use crate::Instruction;
use crate::Machine;
use crate::Program;
use crate::State;

const SAMPLE: &str = "
    nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6
";

fn step(m: &mut Machine, pc: i32, ac: i32, ins: Instruction) {
    assert_eq!(m.program.get(m.pc), Some(&ins));
    assert!(matches!(m.step(), State::Running));
    assert_eq!(ac, m.ac);
    assert_eq!(pc, m.pc);
}

#[test]
fn test_first() {
    use Instruction::*;

    let program = Program::from_str(SAMPLE).unwrap();
    let mut machine = Machine::from(program);

    let steps = [
        (1, 0, Nop(0)),
        (2, 1, Acc(1)),
        (6, 1, Jmp(4)),
        (7, 2, Acc(1)),
        (3, 2, Jmp(-4)),
        (4, 5, Acc(3)),
    ];

    for (pc, ac, ins) in steps {
        step(&mut machine, pc, ac, ins);
    }

    assert!(matches!(machine.step(), State::Looping));
}

#[test]
fn test_second() {
    use Instruction::*;

    let mut program = Program::from_str(SAMPLE).unwrap();
    program.swap(7, Nop(-4)).unwrap();

    let mut machine = Machine::from(program);

    let steps = [
        (1, 0, Nop(0)),
        (2, 1, Acc(1)),
        (6, 1, Jmp(4)),
        (7, 2, Acc(1)),
        (8, 2, Nop(-4)),
        (9, 8, Acc(6)),
    ];

    for (pc, ac, ins) in steps {
        step(&mut machine, pc, ac, ins);
    }

    assert!(matches!(machine.step(), State::Exiting));
}
