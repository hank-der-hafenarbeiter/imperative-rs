use imperative_rs::InstructionSet;

#[derive(InstructionSet, Debug, PartialEq, PartialOrd)]
enum Is {
    #[opcode = "0b10101010"]
    A,
    #[opcode = "0b0x0x0x0x"]
    B { x: u8 },
    #[opcode = "0b0x0y0z10"]
    C { x: u8, y: u8, z: u8 },
    #[opcode = "0b11xxyyxx"]
    D { x: u8, y: i8 },
}

#[test]
fn decoding_bin_opcodes() {
    let mut buf = [0u8; 1];
    let a = Is::A;
    buf[0] = 0b10101010;
    let (num_bytes, instr) = Is::decode(&buf).expect("Failed to decode unit instruction");
    assert_eq!(1, num_bytes, "Reported wrong number of bytes");
    assert_eq!(
        a, instr,
        "Decoded instruction as {:?}. Correct: Is::A",
        instr
    );

    let b = Is::B { x: 15 };
    buf[0] = 0b01010101;
    let (num_bytes, instr) = Is::decode(&buf).expect("Failed to decode unit instruction");
    assert_eq!(1, num_bytes, "Reported wrong number of bytes");
    assert_eq!(
        b, instr,
        "Decoded instruction as {:?}. Correct: Is::B{{x:15}}",
        instr
    );

    let c = Is::C { x: 1, y: 1, z: 1 };
    buf[0] = 0b01010110;
    let (num_bytes, instr) = Is::decode(&buf).expect("Failed to decode unit instruction");
    assert_eq!(1, num_bytes, "Reported wrong number of bytes");
    assert_eq!(
        c, instr,
        "Decoded instruction as {:?}. Correct: Is::C{{x:1, y:1, z:1}} ",
        instr
    );

    let d = Is::D { x: 15, y: 3 };
    buf[0] = 0b11111111;
    let (num_bytes, instr) = Is::decode(&buf).expect("Failed to decode unit instruction");
    assert_eq!(1, num_bytes, "Reported wrong number of bytes");
    assert_eq!(
        d, instr,
        "Decoded instruction as {:?}. Correct: Is::D{{x:15, y:3}} ",
        instr
    );
}
