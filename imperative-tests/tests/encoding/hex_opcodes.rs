use imperative_rs::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    #[opcode = "0x0000"]
    A,
    #[opcode = "0x1x0x"]
    B{x:u8},
    #[ opcode = "0x2xyz" ]
    C{x:u8, y:u8, z:u8},
    #[ opcode = "0x3xyx" ]
    D{x:u8, y:i8},
}

#[test]
fn encoding_hex_opcodes() {
    let mut buf = [0u8; 2];
    let a = Is::A;
    let correct = [0x00, 0x00];
    assert_eq!(Ok(2), a.encode(&mut buf), "Failed to encode unit instruction");
    assert_eq!(correct, buf, "Encoded instruction as {:x?}. Correct: {:x?}", buf, correct);

    let b = Is::B{x:0xab};
    let correct = [0x1a, 0x0b];
    assert_eq!(Ok(2), b.encode(&mut buf), "Failed to encode instruction with variable");
    assert_eq!(correct, buf, "Encoded instruction as {:x?}. Correct: {:x?}", buf, correct);

    let c = Is::C{x:1, y:2, z:3};
    let correct = [0x21, 0x23];
    assert_eq!(Ok(2), c.encode(&mut buf), "Failed to encode instruction multiple variables");
    assert_eq!(correct, buf, "Encoded instruction as {:x?}. Correct: {:x?}", buf, correct);

    let d = Is::D{x:0xab, y:1};
    let correct = [0x3a, 0x1b];
    assert_eq!(Ok(2), d.encode(&mut buf), "Failed to encode instruction multiple interleaved variables");
    assert_eq!(correct, buf, "Encoded instruction as {:x?}. Correct: {:x?}", buf, correct);
}

