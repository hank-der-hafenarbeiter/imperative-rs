use imperative::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    #[opcode = "0xffff"]
    Instr{w:u8},
}

fn main() {}
