use super::Operand;
use super::util::get_data_usize;

use crate::vm::machine::Machine;

/// Pushes an operand into the stack
pub fn push_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    machine.op_push(
        *machine.get_const_data(args[0])
    );
}

/// Jumps the instruction pointer to the specified label
pub fn jump_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    machine.i_jump(
        get_data_usize(machine, args[0])
    );
}

/// Jumps the instruction pointer to the specified label, and pushes a new frame
pub fn call_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    machine.f_call(
        get_data_usize(machine, args[0])
    );
}

/// Pops the last frame and returns to the last instruction
pub fn return_instr(machine: &mut Machine<Operand>, _args: &[usize]) {
    machine.f_return();
}

/// Pushes a local variable with the specified (name, value) to the current frame
pub fn push_local_instr(machine: &mut Machine<Operand>, args: &[usize]) {
    machine.f_push(
        get_data_usize(machine, args[0]),
        *machine.get_const_data(args[1])
    );
}