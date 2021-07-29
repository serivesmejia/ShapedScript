use std::collections::HashMap;
use std::collections::BTreeMap;

use crate::vm::instruction::*;

pub struct Bytecode<O> {
    pub code: Vec<usize>,
    pub symbols: HashMap<usize, String>,
    pub data: Vec<O>,
    pub predef_strings: HashMap<usize, String>,
    pub labels: BTreeMap<usize, usize>,

    pub instruction_dict: InstructionDict<O>
}

impl <O> Bytecode<O> {
    pub fn label_ip(&self, label: usize) -> Option<&usize> {
        self.labels.get(&label)
    }
}

pub struct BytecodeBuilder<O> {
    pub instruction_dict: InstructionDict<O>,
    pub instructions: Vec<usize>,
    pub predef_strings: HashMap<usize, String>,
    pub labels: BTreeMap<usize, usize>,
    pub data: Vec<O>
}

impl <O: PartialEq> BytecodeBuilder <O> {
    pub fn new(instr_dict: InstructionDict<O>) -> BytecodeBuilder<O> {
        let mut labels = BTreeMap::new();
        labels.insert(0, 0);

        BytecodeBuilder {
            instruction_dict: instr_dict,
            instructions: Vec::new(),
            predef_strings: HashMap::new(),
            labels: labels,
            data: Vec::new()
        }
    }

    pub fn add_p(&mut self, cname: &str, params: Vec<O>) {
        self.add(cname, Some(params))
    }

    pub fn add_np(&mut self, cname: &str) {
        self.add(cname, None)
    }

    pub fn add(&mut self, cname: &str, params: Option<Vec<O>>) {
        let instruction = self.instruction_dict.get_by_cname(cname);

        self.instructions.push(instruction.opcode);
        self.instructions.push(instruction.param_num);

        match params {
            Some(prms) =>  {
                if prms.len() != instruction.param_num {
                    panic!(
                        "Instruction {} requires {} parameters ({} passed)", 
                        instruction.cname, instruction.param_num, prms.len()
                    );
                }        

                for p in prms {
                    let pos = self.push_data(p);
                    self.instructions.push(pos);
                }
            }
            None => {
                if instruction.param_num != 0 {
                    panic!(
                        "Instruction {} requires {} parameters (0 passed)",
                        instruction.cname, instruction.param_num
                    );
                }
            }
        }
    }

    pub fn add_label(&mut self, label: usize) {
        for (_, l) in &self.labels {
            if *l == label {
                panic!("Label {} has already been defined", label);
            }
        }

        self.labels.insert(label, self.instructions.len());
    }

    pub fn add_str(&mut self, str: &str) -> usize {
        for (i, s) in &self.predef_strings {
            if *s == str {
                return *i
            }
        }
        
        let i = self.predef_strings.len();
        self.predef_strings.insert(i, str.to_string());

        i
    }

    pub fn build(self) -> Bytecode<O> {
        let symbols = self.instruction_dict.get_symbols();

        Bytecode {
            code: self.instructions,
            data: self.data,
            predef_strings: self.predef_strings,
            labels: self.labels,
            symbols: symbols,

            instruction_dict: self.instruction_dict
        }
    }

    pub fn push_data(&mut self, data: O) -> usize {
        let pos = self.data.iter().position(| d | d == &data);

        match pos {
            Some(p) => p,
            None => {
                self.data.push(data);
                self.data.len() - 1
            }
        }
    }
}