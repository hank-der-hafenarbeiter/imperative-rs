use imperative::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    #[opcode = "0xff"]
    A{f:u8},
}

fn main() {}
