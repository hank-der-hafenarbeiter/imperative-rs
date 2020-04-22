use instruction_set::define_instructionset; 



define_instructionset!{
    #[derive(Debug)]
    pub enum Opcode {
        Ld {z:usize, y:usize} = "0x7z_yz_00_00",
        St {x:usize, y:usize, z:u32} = "0x8xyz2030",
        Mov {s:usize, t:usize} = "0x2sssttt",
        Addi {r:usize, i:u32 } = "0x3rrriii",
        Subi {m:usize, j:u32 } = "0x9mmmjjj",
        Sub {x:usize, y:usize} = "0b1010xxxxxxxxxxxxyyyyyyyyyyyy",
    }
}



fn main () {
    println!("");
    let code = 0x75140000;
    let opcode = Opcode::decode(code).unwrap();
    if let Opcode::Ld{z, y} = opcode {
        assert_eq!(0x54, z);
        assert_eq!(0x1, y);
    }
    println!("{:?}", opcode);
}

