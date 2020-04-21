
pub trait Opcode {
    
    fn decode(&[u8]) -> Result<(usize, Self), ()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
