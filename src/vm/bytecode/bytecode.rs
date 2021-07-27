use crate::InstructionDict;
use crate::WriteOnceDict;
use std::collections::HashMap;

pub struct Bytecode<O> {
    pub code: Vec<usize>,
    pub symbols: HashMap<String, usize>,
    pub data: Vec<O>,
    pub labels: HashMap<String, usize>
}

pub struct BytecodeBuilder<'a, O> {
    pub instr_table: &'a InstructionDict<'a, O>,
    pub instructions: Vec<usize>
}

impl <'a, O> BytecodeBuilder <'a, O> {
    pub fn new(instr_table: &'a InstructionDict<O>) -> BytecodeBuilder<'a, O> {
        BytecodeBuilder {
            instr_table: instr_table,
            instructions: vec!()
        }
    }

}