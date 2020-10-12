use imperative_rs::InstructionSet;


#[derive(InstructionSet)]
enum Instructionset {
    #[ opcode = "0x0x_xx" ]
    A{x:u8},
    #[ opcode = "0x1x_xx_xx" ]
    B{x:u16},
    #[ opcode = "0x2x_xx_xx_xx_xx" ]
    C{x:u32},
    #[ opcode = "0x3x_xx_xx_xx_xx_xx_xx_xx_xx" ]
    D{x:u64},
}

fn main() {
    let mut mem = [0xffu8; 10];
    //0x3x_xx_xx_xx_xx_xx_xx_xx_xx
    mem[0] = 0x3f;
    let _ = Instructionset::decode(&mem);
}

