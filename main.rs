use instruction_set::define_instructionset; 



define_instructionset!{
    #[derive(Debug)]
    pub enum Opcode {
        Ld {z:usize, y:usize} = "0x9zyz0000",
        St {x:usize, y:usize, z:u32} = "0x8xyz2030",
        Mov {s:usize, t:usize} = "0x2sssttt",
        Addi {r:usize, i:u32 } = "0x3rrriii",
        Subi {m:usize, j:u32 } = "0x9mmmjjj",
        Sub {x:usize, y:usize} = "0b1101000000000000000000000000",
        Halt = "0b000101010101010",
    }
}



fn main () {
    println!("");
    let code = 0x75140000;
    let opcode = Opcode::parse(code).unwrap();
    if let Opcode::Ld{z, y} = opcode {
        assert_eq!(0x54, z);
        assert_eq!(0x1, y);
    }
    println!("{:X?}", opcode);
}

