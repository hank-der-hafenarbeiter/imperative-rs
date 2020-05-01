
#[derive(Debug)]
pub enum DecodeError {
    UnknownOpcode,
    UnexpectedEOF,
}

#[derive(Debug)]
pub enum EncodeError {
    UnexpectedEOF,
}

pub trait InstructionSet: std::marker::Sized {
    fn decode(mem:&[u8]) -> Result<(usize, Self), DecodeError>;
    fn encode(&self, buf:&mut [u8]) -> Result<usize, EncodeError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
