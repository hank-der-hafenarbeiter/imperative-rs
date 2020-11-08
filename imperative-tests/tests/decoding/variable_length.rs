use imperative_rs::InstructionSet;

#[derive(InstructionSet, Debug, PartialEq)]
enum Unit {
    #[opcode = "0x0f"]
    One,
    #[opcode = "0x1f_ff"]
    Two,
    #[opcode = "0x2f_ff_ff"]
    Three,
    #[opcode = "0x3f_ff_ff_ff"]
    Four,
    #[opcode = "0x4f_ff_ff_ff_ff"]
    Five,
}

#[test]
fn variable_length_no_vars() {
    {
        let one = [0x0f];
        let correct = Unit::One;
        let (num_bytes, instr) = Unit::decode(&one).unwrap();
        assert_eq!(
            1, num_bytes,
            "Reported wrong number of bytes when decoding 1 byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded one byte unit instruction incorrectly"
        );
    }
    {
        let two = [0x1f, 0xff];
        let correct = Unit::Two;
        let (num_bytes, instr) = Unit::decode(&two).unwrap();
        assert_eq!(
            2, num_bytes,
            "Reported wrong number of bytes when decoding 2 byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded 2 byte unit instruction incorrectly"
        );
    }
    {
        let three = [0x2f, 0xff, 0xff];
        let correct = Unit::Three;
        let (num_bytes, instr) = Unit::decode(&three).unwrap();
        assert_eq!(
            3, num_bytes,
            "Reported wrong number of bytes when decoding 3 byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded 3 byte unit instruction incorrectly"
        );
    }
    {
        let four = [0x3f, 0xff, 0xff, 0xff];
        let correct = Unit::Four;
        let (num_bytes, instr) = Unit::decode(&four).unwrap();
        assert_eq!(
            4, num_bytes,
            "Reported wrong number of bytes when decoding four byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded four byte unit instruction incorrectly"
        );
    }
    {
        let five = [0x4f, 0xff, 0xff, 0xff, 0xff];
        let correct = Unit::Five;
        let (num_bytes, instr) = Unit::decode(&five).unwrap();
        assert_eq!(
            5, num_bytes,
            "Reported wrong number of bytes when decoding five byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded five byte unit instruction incorrectly"
        );
    }
}

#[derive(InstructionSet, Debug, PartialEq)]
enum Vars {
    #[opcode = "0x0x"]
    One { x: u8 },
    #[opcode = "0x1x_xy"]
    Two { x: u8, y: u8 },
    #[opcode = "0x2x_yz_zz"]
    Three { x: u8, y: u8, z: u16 },
    #[opcode = "0x30_wf_xf_yz"]
    Four { w: u8, x: u8, y: u8, z: u8 },
    #[opcode = "0x4v_ww_xx_yy_zz"]
    Five { v: u8, w: u8, x: u8, y: u8, z: u8 },
}

#[test]
fn with_variables() {
    {
        let one = [0x0f];
        let correct = Vars::One { x: 0x0f };
        let (num_bytes, instr) = Vars::decode(&one).unwrap();
        assert_eq!(
            1, num_bytes,
            "Reported wrong number of bytes when decoding 1 byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded one byte instruction with variables incorrectly"
        );
    }
    {
        let two = [0x1a, 0xbc];
        let correct = Vars::Two { x: 0xab, y: 0x0c };
        let (num_bytes, instr) = Vars::decode(&two).unwrap();
        assert_eq!(
            2, num_bytes,
            "Reported wrong number of bytes when decoding 2 byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded 2 byte instruction with variables incorrectly"
        );
    }
    {
        let three = [0x2a, 0xbc, 0xde];
        let correct = Vars::Three {
            x: 0x0a,
            y: 0xb,
            z: 0xcde,
        };
        let (num_bytes, instr) = Vars::decode(&three).unwrap();
        assert_eq!(
            3, num_bytes,
            "Reported wrong number of bytes when decoding 3 byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded 3 byte instruction with variables incorrectly"
        );
    }
    {
        let four = [0x30, 0xaf, 0xbf, 0xcd];
        let correct = Vars::Four {
            w: 0x0a,
            x: 0x0b,
            y: 0x0c,
            z: 0x0d,
        };
        let (num_bytes, instr) = Vars::decode(&four).unwrap();
        assert_eq!(
            4, num_bytes,
            "Reported wrong number of bytes when decoding four byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded four byte instruction with variables incorrectly"
        );
    }
    {
        let five = [0x4a, 0xbc, 0xde, 0xf1, 0x23];
        let correct = Vars::Five {
            v: 0x0a,
            w: 0xbc,
            x: 0xde,
            y: 0xf1,
            z: 0x23,
        };
        let (num_bytes, instr) = Vars::decode(&five).unwrap();
        assert_eq!(
            5, num_bytes,
            "Reported wrong number of bytes when decoding five byte unit instruction"
        );
        assert_eq!(
            instr, correct,
            "Encoded five byte instruction with variables incorrectly"
        );
    }
}
