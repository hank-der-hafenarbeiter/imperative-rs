use imperative::InstructionSet;

#[derive(InstructionSet, Debug, PartialEq, PartialOrd)]
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
        let one = Unit::One;
        let mut buf = [0x00; 12];
        buf[0] = 0x0f;
        let (num_bytes, instr) = Unit::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(1, num_bytes, "Reported wrong number of bytes when encoding one byte unit instruction");
        assert_eq!(one, instr, "Encoded one byte uni instruction incorrectly");
    }
    {
        let two = Unit::Two;
        let mut buf = [0xff; 12];
        buf[0] = 0x1f;
        for idx in 2..12 {
            buf[idx] = 0x00;
        }
        let (num_bytes, instr) = Unit::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(2, num_bytes, "Reported wrong number of bytes when encoding two bytes unit instruction");
        assert_eq!(two, instr, "Encoded one byte uni instruction incorrectly");
    }
    {
        let three = Unit::Three;

        let mut buf = [0xff; 12];
        buf[0] = 0x2f;
        for idx in 3..12 {
            buf[idx] = 0x00;
        }
        let (num_bytes, instr) = Unit::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(3, num_bytes, "Reported wrong number of bytes when encoding three bytes unit instruction");
        assert_eq!(three, instr, "Encoded one byte uni instruction incorrectly");
    }
    {
        let four = Unit::Four;

        let mut buf = [0xff; 12];
        buf[0] = 0x3f;
        for idx in 4..12 {
            buf[idx] = 0x00;
        }
        let (num_bytes, instr) = Unit::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(4, num_bytes, "Reported wrong number of bytes when encoding four bytes unit instruction");
        assert_eq!(four, instr, "Encoded four byte uni instruction incorrectly");

    }
    {
        let five = Unit::Five;

        let mut buf = [0xff; 12];
        buf[0] = 0x4f;
        for idx in 5..12 {
            buf[idx] = 0x00;
        }

        let (num_bytes, instr) = Unit::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(5, num_bytes, "Reported wrong number of bytes when encoding five bytes unit instruction");
        assert_eq!(five, instr, "Encoded five byte uni instruction incorrectly");
    }
}

#[derive(InstructionSet, Debug, PartialEq, PartialOrd)]
enum Vars {
    #[opcode = "0x0x"]
    One{x:u8},
    #[opcode = "0x1x_xy"]
    Two{x:u8, y:u8},
    #[opcode = "0x2x_yz_zz"]
    Three{x:u8, y:u8, z:u16},
    #[opcode = "0x30_wf_xf_yz"]
    Four{w:u8, x:u8, y:u8, z:u8},
    #[opcode = "0x4v_ww_xx_yy_zz"]
    Five{v:u8, w:u8, x:u8, y:u8, z:u8},
}


#[test]
fn with_variables() {

    {
        let mut buf = [0xff; 8];
        let variant = Vars::One{x:1};
        buf[0] = 1;
        let num_bytes = 1;
        for idx in num_bytes..8 {
            buf[idx] = 0;
        }
        let (num_bytes, instr) = Vars::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(1, num_bytes, "Reported wrong number of bytes when encoding {} bytes instruction with variables", num_bytes);
        assert_eq!(variant, instr, "Encoding {} byte instruction with variables incorrectly", num_bytes);
    }     {
        let mut buf = [0xff; 8];
        let variant = Vars::Two{x:0xab, y:0xc};
        buf[0] = 0x1a;
        buf[1] = 0xbc;
        let num_bytes = 2;
        for idx in num_bytes..8 {
            buf[idx] = 0;
        }
        let (num_bytes, instr) = Vars::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(2, num_bytes, "Reported wrong number of bytes when encoding {} bytes instruction with variables", num_bytes);
        assert_eq!(variant, instr, "Encoding {} byte instruction with variables incorrectly", num_bytes);
    }   {
        let mut buf = [0xff; 8];
        let variant = Vars::Three{x:0xa, y:0xb, z:0xcde};
        buf[0] = 0x2a;
        buf[1] = 0xbc;
        buf[2] = 0xde;
        let num_bytes = 3;
        for idx in num_bytes..8 {
            buf[idx] = 0;
        }
        let (num_bytes, instr) = Vars::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(3, num_bytes, "Reported wrong number of bytes when encoding {} bytes instruction with variables", num_bytes);
        assert_eq!(variant, instr, "Encoding {} byte instruction with variables incorrectly", num_bytes);
    }   {
        let mut buf = [0xff; 8];
        let variant = Vars::Four{w:0xa, x:0xb, y:0xc, z:0xd};
        buf[0] = 0x30;
        buf[1] = 0xaf;
        buf[2] = 0xbf;
        buf[3] = 0xcd;
        let num_bytes = 4;
        for idx in num_bytes..8 {
            buf[idx] = 0;
        }
        let (num_bytes, instr) = Vars::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(4, num_bytes, "Reported wrong number of bytes when encoding {} bytes instruction with variables", num_bytes);
        assert_eq!(variant, instr, "Encoding {} byte instruction with variables incorrectly", num_bytes);
    }   {
        let mut buf = [0xff; 8];
        let variant = Vars::Five{v:0x1, w:0x23, x:0x45, y:0x67, z:0x89};
        buf[0] = 0x41;
        buf[1] = 0x23;
        buf[2] = 0x45;
        buf[3] = 0x67;
        buf[4] = 0x89;
        let num_bytes = 5;
        for idx in num_bytes..8 {
            buf[idx] = 0;
        }
        let (num_bytes, instr) = Vars::decode(&buf).expect("Failed to decode unit instruction");
        assert_eq!(5, num_bytes, "Reported wrong number of bytes when encoding {} bytes instruction with variables", num_bytes);
        assert_eq!(variant, instr, "Encoding {} byte instruction with variables incorrectly", num_bytes);
    }
}
