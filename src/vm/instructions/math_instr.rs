use super::Operand;

use crate::vm::machine::Machine;
use crate::vm::instruction::Instruction;

pub fn add_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    let a = machine.op_pop();
    let b = machine.op_pop();

    machine.op_push(a + b);
}