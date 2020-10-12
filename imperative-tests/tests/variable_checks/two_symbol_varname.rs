use imperative_rs::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    #[opcode = "0xffgg"]
    A{gg:u8},
}

fn main() {}
