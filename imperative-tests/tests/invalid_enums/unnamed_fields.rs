use imperative::InstructionSet;
#[derive(InstructionSet)]
enum Instructionset {
    #[ opcode = "0xff_vv_ff" ]
    A(u8),
}

fn main() {}