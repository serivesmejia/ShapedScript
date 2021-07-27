use std::collections::HashMap;
use super::machine::*;

pub type InstructionFunc<O> = fn(machine: &mut Machine<O>, instruction: Instruction<O>, args: &[usize]);

pub struct Instruction<O> {
    opcode: i16,
    cname: &str,
    param_num: i16,
    func: InstructionFunc<O>
}

pub struct InstructionTable<O> {
    instructions: HashMap<i16, Instruction<O>>
}

impl <O> InstructionTable<O> {

    pub fn new() -> InstructionTable<O> {
        InstructionTable {
            instructions: HashMap::new()
        }
    }

    pub fn add(&mut self, 
        opcode: i16, cname: &str, param_num: i16, func: InstructionFunc<O>
    ) {
        self.instructions.insert(opcode, Instruction {
            opcode: opcode,
            cname: cname,
            param_num: param_num,
            func: func
        });
    }

}