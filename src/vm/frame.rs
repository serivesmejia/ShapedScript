use std::collections::HashMap;

pub struct Frame<O> {
    locals: HashMap<usize, O>,
    pub return_iaddress: usize
}

impl <O: PartialEq> Frame<O> {
    pub fn new(return_iaddress: usize) -> Frame<O> {
        Frame {
            locals: HashMap::new(),
            return_iaddress
        }
    }

    pub fn read(&self, name: usize) -> Option<&O> {
        self.locals.get(&name)
    }

    pub fn push(&mut self, name: usize, value: O) {
        self.locals.insert(name, value);
    }
}