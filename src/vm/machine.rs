use super::bytecode::*;
use super::stack::*;

pub struct Machine<O> {
    stack: Stack<O>,
    bytecode: Bytecode<O>,
    instruction_pointer: usize
}

impl <'a, O> Machine<O> {
    pub fn new(bytecode: Bytecode<O>) -> Machine<O> {
        Machine {
            stack: Stack::new(),
            bytecode: bytecode,
            instruction_pointer: 0
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.instruction_pointer >= self.bytecode.code.len() {
                break;
            }

            let opcode = self.read();
            let param_num = self.read();

            let mut args = vec!();
            for _ in 0..param_num {
                args.push(self.read());
            }

            let instruction = self.bytecode.instruction_dict.get(opcode);

            let fnc = instruction.func;
            fnc(self, args.as_slice());
        }
    }

    fn read(&mut self) -> usize {
        let code = self.bytecode.code[self.instruction_pointer];
        self.instruction_pointer += 1;

        code
    }

    pub fn op_push(&mut self, operand: O) {
        self.stack.push(operand)
    }

    pub fn op_pop(&mut self) -> O {
        self.stack.pop()
    }

    pub fn op_read(&self) -> &[O] {
        self.stack.read()
    }

    pub fn i_jump(&mut self, label: usize) {
        self.instruction_pointer = self.bytecode.label_ip(label).unwrap_or_else(
            || panic!("Tried jump to undefined label {}", label)
        );
    }

    pub fn get_const_data(&self, index: usize) -> &O {
        self.bytecode.data.get(index).unwrap_or_else(
            || panic!("Index {} of constant data is out of bounds.", index)
        )
    }
}