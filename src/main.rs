mod vm;
mod util;

use vm::machine::Machine;
use vm::instructions::shaped_instructions;
use vm::bytecode::*;

use std::convert::TryInto;

fn main() {
    let mut builder = BytecodeBuilder::new(shaped_instructions());

    let str = builder.add_str("hello world");
    let str2 = builder.add_str(", sebastian");

    /*
    builder.add_p("PUSHO", vec!(1));

    builder.add_label(1);
    builder.add_p("PUSHO", vec!(1));
    builder.add_np("PLUS");
    builder.add_np("PRINTOS");

    builder.add_p("JUMP", vec!(1));*/

    builder.add_p("CONCATS", vec!(str.try_into().unwrap(), str2.try_into().unwrap()));
    builder.add_np("PRINTSS");

    builder.add_p("JUMP", vec!(1));
 
    Machine::new(builder.build(), 1000000).run();
}