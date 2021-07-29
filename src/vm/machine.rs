use super::bytecode::*;
use super::stack::*;
use super::frame::Frame;
use super::string_pool::StringPool;

pub struct Machine<O> {
    op_stack: Stack<O>,
    frame_stack: Stack<Frame<O>>,
    string_pool: StringPool,

    bytecode: Bytecode<O>,
    instruction_pointer: usize,

    max_fstack_size: usize
}

impl <O: PartialEq> Machine<O> {
    pub fn new(bytecode: Bytecode<O>, max_fstack_size: usize) -> Machine<O> {
        let main_frame: Frame<O> = Frame::new(bytecode.code.len());
        let mut frame_stack: Stack<Frame<O>> = Stack::new();

        frame_stack.push(main_frame);

        let mut str_pool = StringPool::new();
        for (i, str) in &bytecode.predef_strings {
            str_pool.push_i(*i, str.to_string());
        }

        Machine {
            op_stack: Stack::new(),
            frame_stack: frame_stack,
            string_pool: str_pool,
            bytecode: bytecode,
            instruction_pointer: 0,

            max_fstack_size: max_fstack_size
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
        self.op_stack.push(operand)
    }

    pub fn op_pop(&mut self) -> O {
        self.op_stack.pop()
    }

    pub fn op_read(&self) -> &[O] {
        self.op_stack.read()
    }

    pub fn op_read_last(&self) -> &O {
        self.op_stack.read_last()
    }

    pub fn i_jump(&mut self, label: usize) {
        self.instruction_pointer = *self.bytecode.label_ip(label).unwrap_or_else(
            || panic!("Tried jump to undefined label {}", label)
        );
    }

    pub fn f_read(&self, name: usize) -> Option<&O> {
        self.frame_stack.read_last().read(name)
    }

    pub fn f_push(&mut self, name: usize, value: O) {
        self.frame_stack.read_last_mut().push(name, value);
    }

    pub fn f_call(&mut self, label: usize) {
        if self.frame_stack.len() > self.max_fstack_size {
            panic!("Stack overflow");
        }

        self.frame_stack.push(Frame::new(self.instruction_pointer));
        self.i_jump(label)
    }

    pub fn f_return(&mut self) {
        self.instruction_pointer = self.frame_stack.pop().return_iaddress;
    }

    pub fn str_push(&mut self, str: &str) -> usize {
        self.string_pool.push(str.to_string())
    }

    pub fn str_read(&self, id: usize) -> Option<&String> {
        self.string_pool.read(id)
    }

    pub fn str_concat(&mut self, id_a: usize, id_b: usize) -> usize {
        self.string_pool.concat(id_a, id_b)
    }

    pub fn get_const_data(&self, index: usize) -> &O {
        self.bytecode.data.get(index).unwrap_or_else(
            || panic!("Index {} of constant data is out of bounds.", index)
        )
    }
}