pub struct Stack <O> {
    v: Vec<O>
}

impl <O> Stack<O> {
    pub fn new() -> Stack<O> {
        Stack {
            v: vec!()
        }
    }

    pub fn is_empty(&self) -> bool {
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

    pub fn read_last(&self) -> &O {
        if self.is_empty() {
            panic!("Can't read from an empty stack")
        }

        &self.v[self.v.len() - 1]
    }

    pub fn read_last_mut(&mut self) -> &mut O {
        let len = self.v.len();

        if len == 0 {
            panic!("Can't read from an empty stack")
        }

        &mut self.v[len - 1]
    }

    pub fn read(&self) -> &[O] {
        &self.v
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
}