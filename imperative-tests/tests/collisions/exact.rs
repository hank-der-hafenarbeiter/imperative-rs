use imperative::InstructionSet;


#[derive(InstructionSet)]
enum Instructionset {
    #[ opcode = "0xff_ff_ff" ]
    A,
    #[ opcode = "0xff_ff_ff" ]
    B,
}

fn main() {}
