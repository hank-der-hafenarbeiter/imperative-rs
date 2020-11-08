use std::boxed::Box;
use crate::instruction::Instruction;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

pub(crate) enum MatchArm<'a> {
    Fork {
        zero: Box<MatchArm<'a>>,
        one: Box<MatchArm<'a>>,
        msb: usize,
    },
    Leaf {
        instr: &'a Instruction,
    },
}

impl<'a> MatchArm<'a> {

    pub(crate) fn from_list(instr_list: &Vec<&'a Instruction>) -> MatchArm<'a> {
        match instr_list.len()  {
            0 => panic!("Trying to build MatchArm from empty list"),
            1 => {
                MatchArm::Leaf{instr:instr_list[0]}
            },
            2 => {
                let msb = Self::find_msb(&instr_list); 
                let (zero_instrs, one_instrs) = Self::fork_instructions(&instr_list, msb);
                let (zero_arm, one_arm) = (Self::from_list(&zero_instrs), Self::from_list(&one_instrs));
                MatchArm::Fork{zero:Box::new(zero_arm), one:Box::new(one_arm), msb}
            },
            _ => {
                let msb = Self::find_msb(&instr_list); 
                let (zero_instrs, one_instrs) = Self::fork_instructions(&instr_list, msb);
                let (zero_arm, one_arm) = (Self::from_list(&zero_instrs), Self::from_list(&one_instrs));
                MatchArm::Fork{zero:Box::new(zero_arm), one:Box::new(one_arm), msb}
            },
        }
    }

    fn fork_instructions(instr_list:&Vec<&'a Instruction>, msb:usize) -> (Vec<&'a Instruction>, Vec<&'a Instruction>) {
        let mut ones: Vec<&Instruction> = vec!();
        let mut zeros: Vec<&Instruction> = vec!();

        for instr in instr_list {
            let bit = instr.opcode().collision_iter().nth(msb).unwrap();
            match bit {
                '0' => zeros.push(instr),
                '1' => ones.push(instr),
                '*' => {
                    zeros.push(instr);
                    ones.push(instr);
                }
                _ => panic!("Encountered unexpected symbol while building match arms. This is an internal error. Please consider posting on github.com"),

            }
        }

        (zeros, ones)
    }

    fn find_msb(instr_list: &Vec<&Instruction>) -> usize {
        let opcodes: Vec<Vec<char>> = instr_list
            .iter()
            .map(|instr| instr.opcode().collision_iter().collect())
            .collect();
        let num_opcodes:f32 = opcodes.len() as f32;
        let min_len = opcodes.iter().map(|op| op.len()).min().unwrap();
        let mut zeros: Vec<f32> = Vec::with_capacity(min_len); //for each bit position count number of instructions with zero in that position
        let mut ones: Vec<f32> = Vec::with_capacity(min_len); //for each bit position count number of instructions with one in that position
        let mut vars: Vec<f32> = Vec::with_capacity(min_len); //for each bit position count number of instructions with variable in that position

        for _ in 0..min_len {
            zeros.push(0.0);
            ones.push(0.0);
            vars.push(0.0);
        }

        for opcode in &opcodes {
            for (bit_idx, c) in opcode.iter().take(min_len).enumerate() {
                match *c {
                    '0' => zeros[bit_idx] += 1.0,
                    '1' => ones[bit_idx] += 1.0, 
                    '*' => vars[bit_idx] += 1.0,
                    _ => panic!("Encountered unexpected symbol while building match arms. This is an internal error. Please consider posting on github.com"),
                }
            }
        }

        let mut max_score_idx = 0;
        let mut max_score = 0.0;
        for (idx, (o, (z, v))) in zeros.iter().zip(ones.iter().zip(vars.iter())).enumerate() {
            let (o, z, v) = (o/num_opcodes, z/num_opcodes, v/num_opcodes);
            let ones_score = if o > 0.0 {o*o.log2() } else { 0.0 };
            let zeros_score = if z > 0.0 {z*z.log2() } else { 0.0 };
            let score = (1.0 - v) * (-ones_score - zeros_score);
            if score > max_score {
                max_score_idx = idx;
                max_score = score;
            }
        }
        max_score_idx
    }
}

impl<'a> ToTokens for MatchArm<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            MatchArm::Fork{ zero, one, msb } => {
                let (byte_idx, bit_idx) = (msb/8, msb%8);
                let bit_mask:u8 = 1 << 7 - bit_idx;
                tokens.extend(quote! {
                    if #byte_idx >= mem.len() {
                        Err(imperative_rs::DecodeError::UnexpectedEOF)
                    } else if mem[#byte_idx] & #bit_mask == 0 {
                        #zero           
                    } else  {
                        #one
                    }
                });
            },
            MatchArm::Leaf{ instr } => {
                tokens.extend(instr.codec_blocks().1);
                tokens.extend(quote!{
                    else {
                        Err(imperative_rs::DecodeError::UnknownOpcode)
                    }
                });
            },
        }
    }
}
