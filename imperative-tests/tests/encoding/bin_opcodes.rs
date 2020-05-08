use imperative::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    #[opcode = "0b10101010"]
    A,
    #[opcode = "0b0x0x0x0x"]
    B{x:u8},
    #[ opcode = "0b0x0y0z10" ]
    C{x:u8, y:u8, z:u8},
    #[ opcode = "0b11xxyyxx" ]
    D{x:u8, y:i8},
}

#[test]
fn encoding_bin_opcodes() {
    let mut buf = [0u8; 1];
    let a = Is::A;
    assert_eq!(Ok(1), a.encode(&mut buf), "Failed to encode unit instruction");
    assert_eq!(0b10101010, buf[0], "Encoded instruction as {:b}. Correct: 0b10101010", buf[0]);

    let b = Is::B{x:15};
    assert_eq!(Ok(1), b.encode(&mut buf), "Failed to encode instruction with variable");
    assert_eq!(0b01010101, buf[0], "Encoded instruction as {:b}. Correct 0b01010101", buf[0]);

    let c = Is::C{x:1, y:1, z:1};
    assert_eq!(Ok(1), c.encode(&mut buf), "Failed to encode instruction multiple variables");
    assert_eq!(0b01010110, buf[0], "Encoded instruction as {:b}. Correct 0b01010110", buf[0]);

    let d = Is::D{x:15, y:3};
    assert_eq!(Ok(1), d.encode(&mut buf), "Failed to encode instruction multiple interleaved variables");
    assert_eq!(0b11111111, buf[0], "Encoded instruction as {:b}. Correct 0b11111111", buf[0]);
}

