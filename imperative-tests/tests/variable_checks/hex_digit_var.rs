use imperative_rs::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    #[opcode = "0xff"]
    A{f:u8},
}

fn main() {}
