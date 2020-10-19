use imperative_rs::InstructionSet;

#[derive(InstructionSet, Debug, PartialEq, PartialOrd)]
enum Is {
    #[opcode = "0x0000"]
    A,
    #[opcode = "0x1x0x"]
    B { x: u8 },
    #[opcode = "0x2xyz"]
    C { x: u8, y: u8, z: u8 },
    #[opcode = "0x3xyx"]
    D { x: u8, y: i8 },
}

#[test]
fn decoding_hex_opcodes() {
    let a = Is::A;
    let buf = [0x00, 0x00];
    let (num_bytes, instr) = Is::decode(&buf).expect("Failed to decode unit instruction");
    assert_eq!(
        2, num_bytes,
        "Reported wrong number of bytes on unit instruction"
    );
    assert_eq!(
        a, instr,
        "Decoded instruction as {:x?}. Correct: {:x?}",
        instr, a
    );

    let b = Is::B { x: 0xab };
    let buf = [0x1a, 0x0b];
    let (num_bytes, instr) =
        Is::decode(&buf).expect("Failed to decode instruction with one variable");
    assert_eq!(
        2, num_bytes,
        "Reported wrong number of bytes on instruction with variable"
    );
    assert_eq!(
        b, instr,
        "Decoded instruction as {:x?}. Correct: {:x?}",
        instr, b
    );

    let c = Is::C { x: 1, y: 2, z: 3 };
    let buf = [0x21, 0x23];
    let (num_bytes, instr) =
        Is::decode(&buf).expect("Failed to decode instruction with two variables");
    assert_eq!(
        2, num_bytes,
        "Reported wrong number of bytes on instruction with multiple variables"
    );
    assert_eq!(
        c, instr,
        "Decoded instruction as {:x?}. Correct: {:x?}",
        instr, c
    );

    let d = Is::D { x: 0xab, y: 1 };
    let buf = [0x3a, 0x1b];
    let (num_bytes, instr) =
        Is::decode(&buf).expect("Failed to decode instruction with interleaved variables");
    assert_eq!(
        2, num_bytes,
        "Reported wrong number of bytes on instruction with multiple interleaved variables"
    );
    assert_eq!(
        d, instr,
        "Decoded instruction as {:x?}. Correct: {:x?}",
        buf, d
    );
}
