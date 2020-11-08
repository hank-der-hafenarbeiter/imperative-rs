use imperative_rs::InstructionSet;

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
        let mut buf = [0x00; 5];
        let correct = [0x0f, 0x00, 0x00, 0x00, 0x00];
        let num_bytes = one
            .encode(&mut buf)
            .expect("Failed to decode unit instruction");
        assert_eq!(
            1, num_bytes,
            "Reported wrong number of bytes when encoding one byte unit instruction"
        );
        assert_eq!(correct, buf, "Encoded one byte uni instruction incorrectly");
    }
    {
        let two = Unit::Two;
        let mut buf = [0x00; 5];
        let correct = [0x1f, 0xff, 0x00, 0x00, 0x00];
        let num_bytes = two
            .encode(&mut buf)
            .expect("Failed to decode unit instruction with 2 bytes");
        assert_eq!(
            2, num_bytes,
            "Reported wrong number of bytes when encoding two bytes unit instruction"
        );
        assert_eq!(correct, buf, "Encoded two byte uni instruction incorrectly");
    }
    {
        let three = Unit::Three;
        let mut buf = [0x00; 5];
        let correct = [0x2f, 0xff, 0xff, 0x00, 0x00];
        let num_bytes = three
            .encode(&mut buf)
            .expect("Failed to decode unit instruction with 3 bytes");
        assert_eq!(
            3, num_bytes,
            "Reported wrong number of bytes when encoding three bytes unit instruction"
        );
        assert_eq!(
            correct, buf,
            "Encoded three byte uni instruction incorrectly"
        );
    }
    {
        let four = Unit::Four;
        let mut buf = [0x00; 5];
        let correct = [0x3f, 0xff, 0xff, 0xff, 0x00];
        let num_bytes = four
            .encode(&mut buf)
            .expect("Failed to decode unit instruction with 4 bytes");

        assert_eq!(
            4, num_bytes,
            "Reported wrong number of bytes when encoding four bytes unit instruction"
        );
        assert_eq!(
            correct, buf,
            "Encoded four byte uni instruction incorrectly"
        );
    }
    {
        let five = Unit::Five;
        let mut buf = [0x00; 5];
        let correct = [0x4f, 0xff, 0xff, 0xff, 0xff];
        let num_bytes = five
            .encode(&mut buf)
            .expect("Failed to decode unit instruction with 5 bytes");

        assert_eq!(
            5, num_bytes,
            "Reported wrong number of bytes when encoding five bytes unit instruction"
        );
        assert_eq!(
            correct, buf,
            "Encoded five byte uni instruction incorrectly"
        );
    }
}

#[derive(InstructionSet, Debug, PartialEq, PartialOrd)]
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
        let mut buf = [0x00; 5];
        let correct = [0x01, 0x00, 0x00, 0x00, 0x00];
        let variant = Vars::One { x: 1 };
        let num_bytes = variant
            .encode(&mut buf)
            .expect("Failed to encode 1 byte instruction with variable(s)");
        assert_eq!(
            1, num_bytes,
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            correct, buf,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
    }
    {
        let mut buf = [0x00; 5];
        let correct = [0x1a, 0xbc, 0x00, 0x00, 0x00];
        let variant = Vars::Two { x: 0xab, y: 0x0c };
        let num_bytes = variant
            .encode(&mut buf)
            .expect("Failed to encode 2 byte instruction with variable(s)");
        assert_eq!(
            2, num_bytes,
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            correct, buf,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
    }
    {
        let mut buf = [0x00; 5];
        let correct = [0x2a, 0xbc, 0xde, 0x00, 0x00];
        let variant = Vars::Three {
            x: 0x0a,
            y: 0x0b,
            z: 0xcde,
        };
        let num_bytes = variant
            .encode(&mut buf)
            .expect("Failed to encode 3 byte instruction with variable(s)");
        assert_eq!(
            3, num_bytes,
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            correct, buf,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
    }
    {
        let mut buf = [0x00; 5];
        let correct = [0x30, 0xaf, 0xbf, 0xcd, 0x00];
        let variant = Vars::Four {
            w: 0x0a,
            x: 0x0b,
            y: 0x0c,
            z: 0x0d,
        };
        let num_bytes = variant
            .encode(&mut buf)
            .expect("Failed to encode 4 byte instruction with variable(s)");
        assert_eq!(
            4, num_bytes,
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            correct, buf,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
    }
    {
        let mut buf = [0x00; 5];
        let correct = [0x4a, 0xbc, 0xde, 0xf1, 0x23];
        let variant = Vars::Five {
            v: 0x0a,
            w: 0xbc,
            x: 0xde,
            y: 0xf1,
            z: 0x23,
        };
        let num_bytes = variant
            .encode(&mut buf)
            .expect("Failed to encode 5 byte instruction with variable(s)");
        assert_eq!(
            5, num_bytes,
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            correct, buf,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
    }
}
