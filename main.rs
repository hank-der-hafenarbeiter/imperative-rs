use instruction_set::*; 
use istrait::InstructionSet;



#[derive(Debug, InstructionSet)]
pub enum MyInstructions {
    #[opcode = "0b000000x_00000000x"]
    Foo{x:u16},
    #[opcode = "0xff_xx_ff_ff_ff"]
    Bar{x:u8},
    #[opcode = "0xff_ff_ff_ff"]
    Halt,
}




        

fn main () {
    let opcode:&[u8] = &[0xff, 0xff, 0xff, 0xff];
    let mut buf = [0,0,0,0,0,0,0,0];
    let (n, instr) = MyInstructions::decode(&opcode).unwrap();
    println!("{:?}", instr);
    instr.encode(&mut buf);
    println!("{:#?}", buf);
}

