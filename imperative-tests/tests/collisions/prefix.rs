use imperative_rs::InstructionSet;

#[derive(InstructionSet)]
enum Is{
    #[opcode = "0xff"]
    A,
    #[opcode = "0xffff"]
    B,
}

fn main() {}
