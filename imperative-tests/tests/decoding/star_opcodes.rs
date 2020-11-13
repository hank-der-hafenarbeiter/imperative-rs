use imperative_rs::InstructionSet;

#[derive(InstructionSet, PartialEq, Debug)]
enum Star {
    #[opcode = "0b0*0*000y_xxxxxxxx"]
    Bin { x: u8, y: bool },
    #[opcode = "0xf*_xy"]
    Hex { x: u8, y: u8 },
}

#[test]
fn encoding_star_opcodes() {
    {
        let mem = [
            0b00000001, 0b11111111, 0b00010001, 0b11111111, 0b01000001, 0b11111111, 0b01010001,
            0b11111111,
        ];
        let (num_bytes0, instr0) = Star::decode(&mem[0..]).unwrap();
        let (num_bytes1, instr1) = Star::decode(&mem[2..]).unwrap();
        let (num_bytes2, instr2) = Star::decode(&mem[4..]).unwrap();
        assert_eq!(2, num_bytes0);
        assert_eq!(2, num_bytes1);
        assert_eq!(2, num_bytes2);
        assert_eq!(Star::Bin { x: 0xff, y: true }, instr0);
        assert_eq!(Star::Bin { x: 0xff, y: true }, instr1);
        assert_eq!(Star::Bin { x: 0xff, y: true }, instr2);
    }
    {
        let mem = [
            0xf0, 0xab, 0xf1, 0xab, 0xf2, 0xab, 0xf3, 0xab, 0xf4, 0xab, 0xf5, 0xab, 0xf6, 0xab,
            0xf7, 0xab, 0xf8, 0xab, 0xf9, 0xab, 0xfa, 0xab, 0xfb, 0xab, 0xfc, 0xab, 0xfd, 0xab,
            0xfe, 0xab, 0xff, 0xab,
        ];
        for idx in 0..8 {
            let (num_bytes, instr) = Star::decode(&mem[2 * idx..]).unwrap();
            assert_eq!(2, num_bytes);
            assert_eq!(Star::Hex { x: 0x0a, y: 0x0b }, instr);
        }
    }
}
