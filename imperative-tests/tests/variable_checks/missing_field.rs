use imperative::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    #[opcode = "0xfvwf"]
    A{v:u8},
}

fn main() {}
