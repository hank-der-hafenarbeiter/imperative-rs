use imperative_rs::InstructionSet;

#[derive(InstructionSet)]
enum Star {
    #[opcode = "0b0*0*000y_xxxxxxxx"]
    Bin { x: u8, y: bool },
    #[opcode = "0xf*_xy"]
    Hex { x: u8, y: u8 },
}

#[test]
fn encoding_star_opcodes() {
    {
        let mut mem = [0, 0];
        let instr = Star::Bin { x: 0xab, y: true };
        let correct = [0b00000001, 0xab];
        let num_bytes = instr.encode(&mut mem).unwrap();
        assert_eq!(
            num_bytes, 2,
            "Reported wrong number of bytes when decoding 2 byte long instruction with stars"
        );
        assert_eq!(
            correct, mem,
            "Incorrectly encoded 2 byte long instruction with stars"
        );
    }
    {
        let mut mem = [0, 0];
        let instr = Star::Hex { x: 0x0a, y: 0x0b };
        let correct = [0xf0, 0xab];
        let num_bytes = instr.encode(&mut mem).unwrap();
        assert_eq!(
            num_bytes, 2,
            "Reported wrong number of bytes when decoding 2 byte long instruction with stars"
        );
        assert_eq!(
            correct, mem,
            "Incorrectly encoded 2 byte long instruction with stars"
        );
    }
}
