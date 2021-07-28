use super::Operand;

use crate::vm::machine::Machine;

pub fn plus_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    let a = machine.op_pop();
    let b = machine.op_pop();

    machine.op_push(a + b);
}

pub fn times_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    let a = machine.op_pop();
    let b = machine.op_pop();

    machine.op_push(a - b);
}

pub fn div_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    let a = machine.op_pop();
    let b = machine.op_pop();

    machine.op_push(a / b);
}