mod vm;

type Operand = i64;

fn main() {
    let mut vm: vm::machine::Machine<Operand> = vm::machine::Machine::new();

    vm.op_push(32);
    println!("{}", vm.op_pop());
}