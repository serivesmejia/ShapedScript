use super::instruction::*;
use super::stack::*;

pub struct Machine<'a, O> {
    stack: Stack<O>,
    instr_table: InstructionDict<'a, O>
}

impl <'a, O> Machine<'a, O> {
    pub fn new(instr_table: InstructionDict<'a, O>) -> Machine<'a, O> {
        Machine {
            stack: Stack::new(),
            instr_table: instr_table
        }
    }

    pub fn op_push(&mut self, operand: O) {
        self.stack.push(operand)
    }

    pub fn op_pop(&mut self) -> O {
        self.stack.pop()
    }
}