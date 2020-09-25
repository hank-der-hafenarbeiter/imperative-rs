pub use imperative_derive::*;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum DecodeError {
    UnknownOpcode,
    UnexpectedEOF,
    Overflow
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum EncodeError {
    UnexpectedEOF,
}

pub trait InstructionSet: std::marker::Sized {
    fn decode(mem:&[u8]) -> Result<(usize, Self), DecodeError>;
    fn encode(&self, buf:&mut [u8]) -> Result<usize, EncodeError>;
}


