pub mod core_instr;
pub mod math_instr;
pub mod system_instr;
pub mod string_instr;

pub mod util;

use core_instr::*;
use math_instr::*;
use system_instr::*;
use string_instr::*;

use crate::vm::instruction::*;

type Operand = i64;

pub fn shaped_instructions() -> InstructionDict<Operand> {
    let mut dict: InstructionDict<Operand> = InstructionDict::new();

    // CORE
    dict.a_add("PUSHO", 1, push_instr);
    dict.a_add("JUMP", 1, jump_instr);

    dict.a_add("CALL", 1, call_instr);
    dict.a_add("RET", 0, return_instr);

    // CORE - FRAME STACK

    // pushing local values
    dict.a_add("PUSHL", 2, push_local_instr);
    // ..and from the stack
    dict.a_add("PUSHL_OSV", 1, push_local_ostackv_instr);
    dict.a_add("PUSHL_OSNV", 0, push_local_ostacknv_instr);

    // pushing local values to the stack
    dict.a_add("PUSHO_L", 1, push_ostack_local_instr);
    dict.a_add("PUSHON_L", 0, push_ostackn_local_instr);

    // MATH
    dict.a_add("PLUS", 0, plus_instr);
    dict.a_add("MINUS", 0, minus_instr);
    dict.a_add("TIMES", 0, times_instr);
    dict.a_add("DIV", 0, div_instr);
    dict.a_add("POW", 0, pow_instr);

    // SYSTEM
    dict.a_add("PRINT_OS", 0, print_ostack_instr);
    dict.a_add("PRINT_SS", 0, print_string_ostack_instr);

    // STRINGS
    dict.a_add("CONCAT_OS", 0, concat_twostr_ostack_instr);
    dict.a_add("INV_CONCAT_OS", 0, inverse_concat_twostr_ostack_instr);

    dict.a_add("CONCAT", 2, concat_twostr_instr);

    dict
}