use std::collections::HashMap;
use super::machine::*;

pub type InstructionFunc<O> = fn(machine: &mut Machine<O>, args: &[usize]);

pub struct Instruction<O> {
    pub opcode: usize,
    pub cname: String,
    pub param_num: usize,
    pub func: InstructionFunc<O>
}

pub struct InstructionDict<O>(HashMap<usize, Instruction<O>>);

impl <O> InstructionDict<O> {

    pub fn new() -> InstructionDict<O> {
        InstructionDict(HashMap::new())
    }

    pub fn a_add(&mut self,
        cname: &str, param_num: usize, func: InstructionFunc<O>
    ) {
        self.add(self.0.len(), cname, param_num, func);
    }

    pub fn add(&mut self,
        opcode: usize, cname: &str, 
        param_num: usize, func: InstructionFunc<O>
    ) {
        self.0.insert(opcode, Instruction {
            opcode: opcode,
            cname: cname.to_string(),
            param_num: param_num,
            func: func
        });
    }

    pub fn get(&self, opcode: usize) -> &Instruction<O> {
        self.0.get(&opcode).unwrap_or_else(
            || panic!("Can't find instruction with opcode {}", opcode)
        )
    }

    pub fn get_by_cname(&self, cname: &str) -> &Instruction<O> {
        self.0.values().find(| ref i | i.cname == cname).unwrap_or_else(
            || panic!("Can't find instruction with cname {}", cname)
        )
    }

    pub fn get_symbols(&self) -> HashMap<usize, String> {
        let mut result: HashMap<usize, String> = HashMap::new();
        
        let instructions = &self.0;

        for key in instructions.keys() {
            result.insert(*key, (*instructions[key].cname).to_string());
        }

        result
    }
}