mod vm;
mod util;

use vm::machine::Machine;
use vm::instruction::*;

use util::write_once_dict::WriteOnceDict;

type Operand = i64;

fn push_instr(machine: &mut Machine<Operand>, instruction: Instruction<Operand>, args: &[usize]) {

}

fn main() {
    let mut table = InstructionDict::new();
    table.add(0, "PSH", 1, push_instr);

    let mut vm: Machine<Operand> = Machine::new(table);

    vm.op_push(32);
    println!("{}", vm.op_pop());
}