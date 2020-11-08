use std::boxed::Box;
use crate::instruction::Instruction;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

/// This matcher models the structure of decoding an instruction set by implementing a binary tree.
/// When the list only contains one instruction a `MatchArm::Leaf` is formed.
/// If there are more than one element in the list, the most significant bit is selected (see
/// `MatchArm::find_msb(...)`).
/// When the bit is selected the instructions are split/forked into instructions that contain a 0
/// in that position and those which contain a 1. Instructions that could contain both (i.e. there
/// is variable encoded in that bit) are pushed into both lists. From these lists the zero and one
/// arms of the fork are constructed.
/// This struct implements `quote::ToTokens` through which it constructs the decoder for the
/// `InstructionSet::decode(..)` method.
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
        //! This function ceates the match arms for the given list of instructions. When given the
        //! full instruction set it will construct the full decoder for that instructionset
        match instr_list.len()  {
            0 => panic!("Trying to build MatchArm from empty list"),
            1 => {
                MatchArm::Leaf{instr:instr_list[0]}
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
        //! Forks a list of instructions into two lists depending on the defined most significant
        //! bit. For each instruction the function checks if the bit in it's opcode is constant (i.e
        //! '0' or '1') or contains a variables (i.e. '*'). If it is constant it's sorted into
        //! corresponding branch. If it's variable (meaning the bit could be either 1 or 0) it is
        //! put into both branches
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
        //! This function calculates the most significant bit in terms of information content.
        //! It does so by counting the instructions that contain a '0', '1' and '*' ('*' meaning
        //! that there is a variable encoded in this bit). Then it calculates the information
        //! content of this bit for all instructions where it is constant (i.e. where it is not
        //! '*') and weights it by the proportion of instructions that are variable in this bit.
        //! A bit scores the highest of 1 when half the list has a '0' in this position and the other
        //! half has a '1'. If all instructions are the same or variable (i.e. '*'/there is a
        //! variable decoded in it) in a bit the bit is useless and scores 0.
        //! In simple terms this function trys to split the list into two, trying to minimize the
        //! amount of instructions that need to be duplicated into both lists (because they contain
        //! a variable in the deciding bit) while keeping both lists the same length.
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
                tokens.extend(instr.decoder_block());
                tokens.extend(quote!{
                    else {
                        Err(imperative_rs::DecodeError::UnknownOpcode)
                    }
                });
            },
        }
    }
}
