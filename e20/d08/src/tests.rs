use std::str::FromStr;

use crate::Instruction;
use crate::Machine;

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

fn step(m: &mut Machine, pc: i32, acc: i32, ins: Instruction) {
    assert_eq!(m.program[m.instruction as usize], ins);
    assert_eq!(m.step(), Some(()));
    assert_eq!(acc, m.accumulator);
    assert_eq!(pc, m.instruction);
}

#[test]
fn test_first() {
    use Instruction::*;
    let mut machine = Machine::from_str(SAMPLE).unwrap();

    let steps = [
        (1, 0, Nop(0)),
        (2, 1, Acc(1)),
        (6, 1, Jmp(4)),
        (7, 2, Acc(1)),
        (3, 2, Jmp(-4)),
        (4, 5, Acc(3)),
        (1, 5, Jmp(-3)),
    ];

    for (pc, acc, ins) in steps {
        step(&mut machine, pc, acc, ins);
    }
}

#[test]
fn test_second() {
    use Instruction::*;
    let mut machine = Machine::from_str(SAMPLE).unwrap();

    machine.swap(7);

    let steps = [
        (1, 0, Nop(0)),
        (2, 1, Acc(1)),
        (6, 1, Jmp(4)),
        (7, 2, Acc(1)),
        (8, 2, Nop(-4)),
        (9, 8, Acc(6)),
    ];

    for (pc, acc, ins) in steps {
        step(&mut machine, pc, acc, ins);
    }

    assert_eq!(machine.step(), None);
}
