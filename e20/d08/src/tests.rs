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

macro_rules! assert_step {
    ($m:expr, $pc:expr, $acc:expr, $ins:expr) => {
        assert_eq!($m.program[$m.instruction as usize], $ins);
        assert_eq!($m.step().unwrap(), ());
        assert_eq!($acc, $m.accumulator);
        assert_eq!($pc, $m.instruction);
    };
}

#[test]
fn test_first() {
    use Instruction::*;

    let mut machine = Machine::from_str(SAMPLE).unwrap();

    assert_step!(machine, 1, 0, Nop(0));
    assert_step!(machine, 2, 1, Acc(1));
    assert_step!(machine, 6, 1, Jmp(4));
    assert_step!(machine, 7, 2, Acc(1));
    assert_step!(machine, 3, 2, Jmp(-4));
    assert_step!(machine, 4, 5, Acc(3));
    assert_step!(machine, 1, 5, Jmp(-3));
}

#[test]
fn test_second() {
    use Instruction::*;

    let mut machine = Machine::from_str(SAMPLE).unwrap();

    machine.swap(7);

    assert_step!(machine, 1, 0, Nop(0));
    assert_step!(machine, 2, 1, Acc(1));
    assert_step!(machine, 6, 1, Jmp(4));
    assert_step!(machine, 7, 2, Acc(1));
    assert_step!(machine, 8, 2, Nop(-4));
    assert_step!(machine, 9, 8, Acc(6));
    assert_eq!(machine.step(), None);
}
