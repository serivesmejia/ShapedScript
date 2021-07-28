use super::Operand;

use crate::vm::machine::Machine;
use crate::vm::instruction::Instruction;

use std::convert::TryInto;

pub fn push_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    machine.op_push(
        *machine.get_const_data(args[0])
    )
}

pub fn jump_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    machine.i_jump(
        (*machine.get_const_data(args[0])).try_into().unwrap()
    )
}

pub fn print_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    println!("{:?}", machine.op_read())
}