pub mod core_instr;
pub mod math_instr;

use core_instr::*;
use math_instr::*;

use crate::vm::instruction::*;

type Operand = i64;

pub fn shaped_instructions() -> InstructionDict<Operand> {
    let mut dict: InstructionDict<Operand> = InstructionDict::new();

    // CORE
    dict.add(0, "PSH", 1, push_instr);
    dict.add(1, "JMP", 1, jump_instr);

    // MATH
    dict.add(2, "ADD", 0, add_instr);
 
    // SYSTEM
    dict.add(3, "PRT", 0, print_instr);

    dict
}