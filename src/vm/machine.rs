use super::stack::*;

pub struct Machine<O> {
    stack: Stack<O>
}

impl <O> Machine<O> {
    pub fn new() -> Machine<O> {
        Machine {
            stack: Stack::new()
        }
    }

    pub fn op_push(&mut self, operand: O) {
        self.stack.push(operand)
    }

    pub fn op_pop(&mut self) -> O {
        self.stack.pop()
    }
}