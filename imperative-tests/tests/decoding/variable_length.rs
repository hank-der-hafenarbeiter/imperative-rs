use imperative_rs::{EncodeError, InstructionSet};

#[derive(InstructionSet)]
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
    let mut buf = [0x00; 12];
    let mut short_buf = [0x00; 1];

    {
        let one = Unit::One;
        let mut correct = [0x00; 12];
        correct[0] = 0x0f;
        assert_eq!(
            Ok(1),
            one.encode(&mut buf),
            "Reported wrong number of bytes when encoding one byte unit instruction"
        );
        assert_eq!(buf, correct, "Encoded one byte uni instruction incorrectly");
        assert_eq!(Ok(1), one.encode(&mut short_buf), "Reported wrong number of bytes when encoding one byte unit instruction into one byte array");
        assert_eq!(
            short_buf[0], correct[0],
            "Encoded one byte uni instruction incorrectly into one byte array"
        );
    }
    {
        let two = Unit::Two;
        buf = [0x00; 12];

        let mut correct = [0xff; 12];
        correct[0] = 0x1f;
        for idx in 2..12 {
            correct[idx] = 0x00;
        }

        short_buf = [0x00];
        assert_eq!(
            Ok(2),
            two.encode(&mut buf),
            "Reported wrong number of bytes when encoding two bytes unit instruction"
        );
        assert_eq!(buf, correct, "Encoded one byte uni instruction incorrectly");
        assert_eq!(
            Err(EncodeError::UnexpectedEOF),
            two.encode(&mut short_buf),
            "Didn't report error when writing two byte instruction into one byte array"
        );
        assert_eq!(
            short_buf[0], 0x00,
            "Modified buffer when failing to write two byte instruction into one byte buffer"
        );
    }
    {
        let three = Unit::Three;
        buf = [0x00; 12];

        let mut correct = [0xff; 12];
        correct[0] = 0x2f;
        for idx in 3..12 {
            correct[idx] = 0x00;
        }

        short_buf = [0x00];
        assert_eq!(
            Ok(3),
            three.encode(&mut buf),
            "Reported wrong number of bytes when encoding three bytes unit instruction"
        );
        assert_eq!(buf, correct, "Encoded one byte uni instruction incorrectly");
        assert_eq!(
            Err(EncodeError::UnexpectedEOF),
            three.encode(&mut short_buf),
            "Didn't report error when writing three byte instruction into one byte array"
        );
        assert_eq!(
            short_buf[0], 0x00,
            "Modified buffer when failing to write three byte instruction into one byte buffer"
        );
    }
    {
        let four = Unit::Four;
        buf = [0x00; 12];

        let mut correct = [0xff; 12];
        correct[0] = 0x3f;
        for idx in 4..12 {
            correct[idx] = 0x00;
        }

        short_buf = [0x00];
        assert_eq!(
            Ok(4),
            four.encode(&mut buf),
            "Reported wrong number of bytes when encoding four bytes unit instruction"
        );
        assert_eq!(
            buf, correct,
            "Encoded four byte uni instruction incorrectly"
        );
        assert_eq!(
            Err(EncodeError::UnexpectedEOF),
            four.encode(&mut short_buf),
            "Didn't report error when writing four byte instruction into one byte array"
        );
        assert_eq!(
            short_buf[0], 0x00,
            "Modified buffer when failing to write four byte instruction into one byte buffer"
        );
    }
    {
        let five = Unit::Five;
        buf = [0x00; 12];

        let mut correct = [0xff; 12];
        correct[0] = 0x4f;
        for idx in 5..12 {
            correct[idx] = 0x00;
        }

        short_buf = [0x00];
        assert_eq!(
            Ok(5),
            five.encode(&mut buf),
            "Reported wrong number of bytes when encoding five bytes unit instruction"
        );
        assert_eq!(
            buf, correct,
            "Encoded five byte uni instruction incorrectly"
        );
        assert_eq!(
            Err(EncodeError::UnexpectedEOF),
            five.encode(&mut short_buf),
            "Didn't report error when writing four byte instruction into one byte array"
        );
        assert_eq!(
            short_buf[0], 0x00,
            "Modified buffer when failing to write five byte instruction into one byte buffer"
        );
    }
}

#[derive(InstructionSet)]
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
        let mut buf = [0; 8];
        let mut correct = [0xff; 8];
        let variant = Vars::One { x: 1 };
        let num_bytes = 1;
        correct[0] = 1;
        for idx in num_bytes..8 {
            correct[idx] = 0;
        }
        assert_eq!(
            Ok(num_bytes),
            variant.encode(&mut buf),
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            buf, correct,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
    }
    {
        let mut buf = [0; 8];
        let mut correct = [0xff; 8];
        let mut short = [0];
        let variant = Vars::Two { x: 0xab, y: 0xc };
        let num_bytes = 2;
        correct[0] = 0x1a;
        correct[1] = 0xbc;
        for idx in num_bytes..8 {
            correct[idx] = 0;
        }
        assert_eq!(
            Ok(num_bytes),
            variant.encode(&mut buf),
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            buf, correct,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
        assert_eq!(Err(EncodeError::UnexpectedEOF), variant.encode(&mut short), "Didn't report error when trying to encode {} byte instruction with variables into one byte array", num_bytes);
        assert_eq!(short[0], 0x00, "Modified buffer when failing to write {} byte instruction with variables into one byte buffer", num_bytes);
    }
    {
        let mut buf = [0; 8];
        let mut correct = [0xff; 8];
        let mut short = [0];
        let variant = Vars::Three {
            x: 0xa,
            y: 0xb,
            z: 0xcde,
        };
        let num_bytes = 3;
        correct[0] = 0x2a;
        correct[1] = 0xbc;
        correct[2] = 0xde;
        for idx in num_bytes..8 {
            correct[idx] = 0;
        }
        assert_eq!(
            Ok(num_bytes),
            variant.encode(&mut buf),
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            buf, correct,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
        assert_eq!(Err(EncodeError::UnexpectedEOF), variant.encode(&mut short), "Didn't report error when trying to encode {} byte instruction with variables into one byte array", num_bytes);
        assert_eq!(short[0], 0x00, "Modified buffer when failing to write {} byte instruction with variables into one byte buffer", num_bytes);
    }
    {
        let mut buf = [0; 8];
        let mut correct = [0xff; 8];
        let mut short = [0];
        let variant = Vars::Four {
            w: 0xa,
            x: 0xb,
            y: 0xc,
            z: 0xd,
        };
        let num_bytes = 4;
        correct[0] = 0x30;
        correct[1] = 0xaf;
        correct[2] = 0xbf;
        correct[3] = 0xcd;
        for idx in num_bytes..8 {
            correct[idx] = 0;
        }
        assert_eq!(
            Ok(num_bytes),
            variant.encode(&mut buf),
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            buf, correct,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
        assert_eq!(Err(EncodeError::UnexpectedEOF), variant.encode(&mut short), "Didn't report error when trying to encode {} byte instruction with variables into one byte array", num_bytes);
        assert_eq!(short[0], 0x00, "Modified buffer when failing to write {} byte instruction with variables into one byte buffer", num_bytes);
    }
    {
        let mut buf = [0; 8];
        let mut correct = [0xff; 8];
        let mut short = [0];
        let variant = Vars::Five {
            v: 0x1,
            w: 0x23,
            x: 0x45,
            y: 0x67,
            z: 0x89,
        };
        let num_bytes = 5;
        correct[0] = 0x41;
        correct[1] = 0x23;
        correct[2] = 0x45;
        correct[3] = 0x67;
        correct[4] = 0x89;
        for idx in num_bytes..8 {
            correct[idx] = 0;
        }
        assert_eq!(
            Ok(num_bytes),
            variant.encode(&mut buf),
            "Reported wrong number of bytes when encoding {} bytes instruction with variables",
            num_bytes
        );
        assert_eq!(
            buf, correct,
            "Encoding {} byte instruction with variables incorrectly",
            num_bytes
        );
        assert_eq!(Err(EncodeError::UnexpectedEOF), variant.encode(&mut short), "Didn't report error when trying to encode {} byte instruction with variables into one byte array", num_bytes);
        assert_eq!(short[0], 0x00, "Modified buffer when failing to write {} byte instruction with variables into one byte buffer", num_bytes);
    }
}
