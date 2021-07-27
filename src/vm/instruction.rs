use std::collections::HashMap;
use super::machine::*;

pub type InstructionFunc<O> = fn(machine: &mut Machine<O>, instruction: Instruction<O>, args: &[usize]);

pub struct Instruction<'a, O> {
    opcode: i16,
    cname: &'a str,
    param_num: i16,
    func: InstructionFunc<O>
}

pub struct InstructionDict<'a, O> {
    instructions: HashMap<i16, Instruction<'a, O>>
}

impl <'a, O> InstructionDict<'a, O> {

    pub fn new() -> InstructionDict<'a, O> {
        InstructionDict {
            instructions: HashMap::new()
        }
    }

    pub fn add(&mut self, 
        opcode: i16, cname: &'a str, param_num: i16, func: InstructionFunc<O>
    ) {
        self.instructions.insert(opcode, Instruction {
            opcode: opcode,
            cname: cname,
            param_num: param_num,
            func: func
        });
    }

    pub fn get(&self, opcode: i16) -> Option<&Instruction<O>> {
        self.instructions.get(&opcode)
    }

    pub fn get_by_cname(&self, cname: &str) -> Option<&Instruction<O>> {
        self.instructions.values().find(| ref i | i.cname == cname)
    }

}