use super::Operand;
use crate::vm::machine::Machine;

use std::convert::TryInto;

pub fn get_data_usize(machine: &Machine<Operand>, arg: usize) -> usize {
    (*machine.get_const_data(arg)).try_into().unwrap()
}

pub fn double_pop(machine: &mut Machine<Operand>) -> (Operand, Operand) {
    (machine.op_pop(), machine.op_pop())
}

pub fn double_pop_usize(machine: &mut Machine<Operand>) -> (usize, usize) {
    (machine.op_pop() as usize, machine.op_pop() as usize)
}