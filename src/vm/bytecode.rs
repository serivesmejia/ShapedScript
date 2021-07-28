use crate::vm::instruction::*;

pub struct Bytecode<O> {
    pub code: Vec<usize>,
    pub symbols: Vec<(usize, String)>,
    pub data: Vec<O>,
    pub labels: Vec<(usize, usize)>,

    pub instruction_dict: InstructionDict<O>
}

impl <O> Bytecode<O> {
    pub fn label_ip(&self, label: usize) -> Option<usize> {
        for l in self.labels.as_slice() {
            if l.0 == label {
                return Some(l.1);
            }
        }

        None
    }
}

pub struct BytecodeBuilder<O> {
    pub instruction_dict: InstructionDict<O>,
    pub instructions: Vec<usize>,
    pub labels: Vec<(usize, usize)>,
    pub data: Vec<O>
}

impl <O: PartialEq> BytecodeBuilder <O> {
    pub fn new(instr_dict: InstructionDict<O>) -> BytecodeBuilder<O> {
        let mut labels = vec!();
        labels.push((0, 0));

        BytecodeBuilder {
            instruction_dict: instr_dict,
            instructions: Vec::new(),
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
        for (i, l) in &self.labels {
            if *l == label {
                panic!("Label {} has already been defined", label);
            }
        }

        self.labels.push((label, self.instructions.len()))
    }

    pub fn build(self) -> Bytecode<O> {
        let mut labels = self.labels.clone();
        labels.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));

        let symbols = self.instruction_dict.get_symbols();

        Bytecode {
            code: self.instructions,
            data: self.data,
            labels: labels,
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