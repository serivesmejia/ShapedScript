use super::Operand;

use crate::vm::machine::Machine;
use crate::vm::instruction::Instruction;

pub fn push_instr(machine: &mut Machine<Operand>, instruction: &Instruction<Operand>, args: &[usize]) {
    
}

pub fn print_instr(machine: &mut Machine<Operand>, instruction: &Instruction<Operand>, args: &[usize]) {
    println!("{:?}", machine.op_read())
}