#![no_main]
use libfuzzer_sys::fuzz_target;
use imperative::*;

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


fuzz_target!(|data: &[u8]| {
    let mut bytes_used = 0;
    let mut buf = Vec::new();
    while bytes_used < data.len() {
        match Vars::decode(&data[bytes_used..]) {
            Ok((instr_len, instr)) => {
                buf.clear();
                buf.reserve(instr_len);
                for _ in 0..instr_len {
                    buf.push(0);
                }
                let res = instr.encode(&mut buf[..]);
                assert_eq!(Ok(instr_len), res, "Used more/fewer bytes when encoding");
                assert_eq!(data[bytes_used..bytes_used+instr_len], buf[..instr_len], "Encoded opcode doesn't match source");
                bytes_used += instr_len;
            },
            _ => bytes_used += 1,
        }
    }
});
