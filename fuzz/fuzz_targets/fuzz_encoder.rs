#![no_main]
use libfuzzer_sys::fuzz_target;
use imperative::*;

#[derive(InstructionSet, Debug, PartialEq, PartialOrd)]
enum Instructions {
    #[opcode = "0xav_ww_xx_yy_zz"]
    A{v:u8, w:u8, x:u8, y:u8, z:u8},
    #[opcode = "0xbv_xx_xx_yy_yy"]
    B{v:u8, w:u8, x:u16, y:u16},
    #[opcode = "0xcv_xx_xx_xx_xx"]
    C{v:u8, x:u32},
    #[opcode = "0xdv_xx_xx_xx_xx_xx_xx_xx_xx"]
    D{v:u8, x:u64},
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
struct Vars {
    eight:u8,
    sixteen:u16,
    thirtytwo:u32,
    sixtyfour:u64,
}

fuzz_target!(|vars: Vars| {
    {
        let mut buf = [0u8; 9];
        let instr = Instructions::A{v:vars.eight, w:vars.eight, x:vars.eight, y:vars.eight, z:vars.eight};
        let mut bytes_used = 5;
        let encode_result = instr.encode(&mut buf);
        assert_eq!(Ok(bytes_used), encode_result, "Incorrect encoding result.")
        assert_eq!(Ok(instr), Instructions::decode(&buf), "Re-decoded instr doesn't match original.");
    }   {
        let mut buf = [0u8; 9];
        let instr = Instructions:B{v:vars.eight, w:vars.eight, x:vars.sixteen, y:vars.sixteen};
        let mut bytes_used = 5;
        let encode_result = instr.encode(&mut buf);
        assert_eq!(Ok(bytes_used), encode_result, "Incorrect encoding result.")
        assert_eq!(Ok(instr), Instructions::decode(&buf), "Re-decoded instr doesn't match original.");
    }   {
        let mut buf = [0u8; 9];
        let instr = Instructions::C{v:vars.eight, x:vars.thirtytwo};
        let mut bytes_used = 5;
        let encode_result = instr.encode(&mut buf);
        assert_eq!(Ok(bytes_used), encode_result, "Incorrect encoding result.")
        assert_eq!(Ok(instr), Instructions::decode(&buf), "Re-decoded instr doesn't match original.");
    }   {
        let mut buf = [0u8; 9];
        let instr = Instructions::D{v:vars.eight, x:vars.sixtytwo};
        let mut bytes_used = 9;
        let encode_result = instr.encode(&mut buf);
        assert_eq!(Ok(bytes_used), encode_result, "Incorrect encoding result.")
        assert_eq!(Ok(instr), Instructions::decode(&buf), "Re-decoded instr doesn't match original.");
    }

});