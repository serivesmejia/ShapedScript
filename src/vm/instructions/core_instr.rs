use super::Operand;

use crate::vm::machine::Machine;
use crate::vm::instruction::Instruction;

pub fn push_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    machine.op_push(
        *machine.get_const_data(args[0])
    )
}

pub fn print_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    println!("{:?}", machine.op_read())
}