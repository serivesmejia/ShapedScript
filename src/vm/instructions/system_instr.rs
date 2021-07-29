use super::Operand;
use crate::vm::machine::Machine;

use std::convert::TryInto;

/// Prints the current values in the op stack as a single line
pub fn print_ostack_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    println!("{:?}", machine.op_read())
}

/// Prints a string placed on the string pool, takes the string id by popping it from the stack
pub fn print_string_ostack_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let id = *machine.op_read_last();
    let str = match machine.str_read(id.try_into().unwrap()) {
        Some(s) => s,
        None => panic!("String with id {} is not present in the pool", id)
    };

    println!("{}", str);
}