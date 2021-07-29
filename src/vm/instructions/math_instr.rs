use super::Operand;
use super::util::*;
use crate::vm::machine::Machine;

use std::convert::TryInto;

/// Pops and sums the last two values in the stack and pushes the result
pub fn plus_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let (a, b) = double_pop(machine);
    machine.op_push(a + b);
}

/// Pops and substracts the last two values in the stack and pushes the result
pub fn minus_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let (a, b) = double_pop(machine);
    machine.op_push(a - b);
}

/// Takes two values, subtracts them and pushes the result into the stack
/// Pops and multiplies the last two values in the stack and pushes the result
pub fn times_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let (a, b) = double_pop(machine);
    machine.op_push(a * b);
}

/// Pops and divides the last two values in the stack and pushes the result
pub fn div_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let (a, b) = double_pop(machine);
    machine.op_push(a / b);
}

/// Pops the last two values in the stack (a, b), elevates a to the power of b and pushes the result
pub fn pow_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let (a, b) = double_pop(machine);
    machine.op_push(a.pow(b.try_into().unwrap()));
}