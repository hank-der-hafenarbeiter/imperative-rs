//! This crate provides the `InstructionSet`-trait and corresponding error types, as well as
//! a procedural macro automatically derive the trait for `enum`s. A type implementing
//! `InstructionSet` provides `fn InstructionSet::decode(...) -> {...}` to decode instructions from a `&[u8]`
//! and `fn InstructionSet::encode(...) -> {...}` to encode and write an instruction into a `&[u8]`.
//!```
//!use imperative::InstructionSet;
//!
//!#[derive(InstructionSet)]
//!enum Is {
//!    //constant opcode
//!    #[opcode = "0x0000"]
//!    Nop,
//!    //hex opcode with split variable x
//!    #[opcode = "0x1x0x"]
//!    Inc{x:u8},
//!    //hex opcode with three variables
//!    #[ opcode = "0x2xxyyzz" ]
//!    Add{x:u8, y:u8, z:u8},
//!    //bin opcode with two variables and underscores for readability
//!    #[ opcode = "0b100000000_xxxxyyyyy_xyxyxyxy" ]
//!    Mov{x:u8, y:i8},
//!}
//!
//!fn main() {
//!    let mut mem = [0u8; 1024];
//!    let (num_bytes, Is::Nop) = Is::decode(&mem).unwrap();
//!    let instruction = Is::Add{x:0xab, y:0xcd, z:0xef};
//!    assert_eq!(4, instruction.encode(&mut mem[100..]).unwrap());
//!    assert_eq!([0x2a, 0xbc, 0xde, 0xf0], mem[100..104]);
//!}
//!
//!```
#[doc(hidden)]
pub use imperative_rs_derive::*;
/// This type is returned by `fn InstructionSet::decode(...)` in case no instruction could be
/// decoded.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum DecodeError {
    /// This variant is emitted if the slice contains no known opcode.
    UnknownOpcode,
    /// Is emitted if the slice ended before a complete opcode could be found. Extending the end
    /// of the slice could lead to successful decoding.
    UnexpectedEOF,
    /// Is emitted if the opcode encodes a variable that is too big for the corresponding variable.
    /// An example would be if the opcode contains a 9 bit variable that is put into a `u8` during
    /// decoding.
    /// ```
    /// use imperative::{InstructionSet, DecodeError};
    /// #[derive(InstructionSet)]
    /// enum Is {
    ///     #[opcode = "0b0000000v_vvvvvvvv"]
    ///     A{v:u8},
    /// }
    ///
    /// fn main () {
    ///     let mem = [0b00000001, 0b11111111];
    ///     let instr = Is::decode(&mem);
    ///     assert_eq!(DecodeError::Overflow, instr); //trying to cast 256 into an u8
    /// }
    ///
    /// ```
    Overflow
}

/// This Type is returned by `fn InstructionSet::encode(...) -> {...} when the instruction could not
/// be encoded.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum EncodeError {
    /// Instruction couldn't be encoded because the provided buffer was too short.
    UnexpectedEOF,
}

/// This `trait` defines an instruction set. It provides functionality to decode from or encode to
/// opcodes. It can be autoderived for suitable `enum`s by a procedual macro provided by this crate.
pub trait InstructionSet: std::marker::Sized {
    /// Used to decode an instruction (i.e. `Self`) from a byte buffer. The buffer needs to be
    /// provided as a `&[u8]` and the function returns a result containing either a tuple containing
    /// the number of bytes written and the resulting instruction or an `DecodeError`.
    fn decode(mem:&[u8]) -> Result<(usize, Self), DecodeError>;
    /// Used to encode instructions into a byte buffer. The buffer needs to be provided as a
    /// `&mut [u8]`. The function returns a result containing either the number of bytes read or an
    /// `EncodeError`
    fn encode(&self, buf:&mut [u8]) -> Result<usize, EncodeError>;
}


