use imperative_rs::InstructionSet;
#[derive(InstructionSet)]
enum Instructionset {
    #[ opcode = "0xff_ff_ff" ]
    A,
    #[ opcode = "0xff_vv_ff" ]
    B{v:u8},
}

fn main() {}
