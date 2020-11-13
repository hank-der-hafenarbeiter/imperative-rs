use imperative_rs::InstructionSet;

#[derive(InstructionSet)]
enum Star {
    #[opcode = "0x00"]
    Prefix,
    #[opcode = "0x0*"]
    SameAsPrefix,
    #[opcode = "0x00_11"]
    TwoBytes,
    #[opcode = "0x**_00"]
    TwoBytesWithStars,
}

fn main() {}
