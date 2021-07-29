use super::Operand;
use super::util::*;

use crate::vm::machine::Machine;

use std::convert::TryInto;

/// Concatenates two strings by popping their ids from the opstack and 
/// pushes the new string id to the opstack
pub fn concat_twostr_ostack_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let (a, b) = double_pop_usize(machine);
    concat(machine, a, b)
}

/// Concatenates two strings by popping their ids from the opstack and 
/// pushes the new string id to the opstack, inverse pop order
pub fn inverse_concat_twostr_ostack_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    let (b, a) = double_pop_usize(machine);
    concat(machine, a, b)
}

/// Concatenates two strings by their pool ids, pushes the new string id to the opstack
pub fn concat_twostr_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    let a = get_data_usize(machine, args[0]);
    let b = get_data_usize(machine, args[1]);
    concat(machine, a, b)
}

fn concat(machine: &mut Machine<Operand>, a: usize, b: usize) {
    let result: i64 = machine.str_concat(a, b).try_into().unwrap();
    machine.op_push(result);
}