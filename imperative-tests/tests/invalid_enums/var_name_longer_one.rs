use imperative_rs::InstructionSet;
#[derive(InstructionSet)]
enum Inst {
    #[ opcode = "0xff_vw_ff" ]
    A{vw:u8},
}

fn main() {}