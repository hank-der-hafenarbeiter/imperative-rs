use imperative::InstructionSet;


#[derive(InstructionSet)]
enum Instructionset {
    #[ opcode = "0xff_ff_ff" ]
    A,
    #[ opcode = "0b11111111_11111111_11111111" ]
    B,
}

fn main() {}
