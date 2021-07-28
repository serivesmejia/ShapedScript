pub struct Stack <O> {
    v: Vec<O>
}

impl <O> Stack<O> {
    pub fn new() -> Stack<O> {
        Stack {
            v: vec!()
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.v.is_empty()
    }

    pub fn push(&mut self, operand: O) {
        self.v.push(operand)
    }

    pub fn pop(&mut self) -> O {
        match self.v.pop() {
            Some(v) => v,
            None => panic!("Can't pop from an empty stack")
        }
    }

    pub fn read(&self) -> &[O] {
        &self.v
    }
}