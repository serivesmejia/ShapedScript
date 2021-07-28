mod vm;
mod util;

use vm::machine::Machine;
use vm::instructions::shaped_instructions;
use vm::bytecode::*;

type Operand = i64;

fn main() {
    let instruction_dict = shaped_instructions();

    let mut builder = BytecodeBuilder::new(instruction_dict);
    builder.add_p("PSH", vec!(1));
    builder.add_np("PRT");
 
    let mut vm: Machine<Operand> = Machine::new(builder.build());
    vm.run();
}