use super::Operand;
use super::util::get_data_usize;

use crate::vm::machine::Machine;
use std::convert::TryInto;

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


/// Pushes a local variable with the specified name and a value popped from the opstack to the current frame
pub fn push_local_ostackv_instr(machine: &mut Machine <Operand>, args: &[usize]) {
    let value = machine.op_pop();

    machine.f_push(
        get_data_usize(machine, args[0]),
        value
    );
}

/// Pushes a local variable with the name and value popped respectively from the opstack to the current frame
pub fn push_local_ostacknv_instr(machine: &mut Machine <Operand>, _args: &[usize]) {
    let name = machine.op_pop();
    let value = machine.op_pop();

    machine.f_push(name as usize, value);
}

/// Pushes the value of the specified local variable from the current frame with the specified name to the opstack
pub fn push_ostack_local_instr(machine: &mut Machine <Operand>, args: &[usize]) {
    machine.op_push(
        *machine.f_read(
            get_data_usize(machine, args[0])
        ).unwrap()
    );
}

/// Pushes the value of a local variable in the current frame to the
/// opstack, the name is popped from the opstack
pub fn push_ostackn_local_instr(machine: &mut Machine <Operand>, _args: &[usize]) {
    let name = machine.op_pop() as usize;
    let value = *machine.f_read(name).unwrap();

    machine.op_push(value);
}