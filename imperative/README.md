#imperative

This crate provides a `trait`-definition, error types and a procedural macro to automatically derive
instruction sets from `enum`.

The `imperative::InstructionSet`-trait consists of two functions. 
`fn InstructionSet::encode(&self, buf:&mut [u8]) -> Result<usize, EncodeError>` encodes instructions
 (i.e. `enum`-variants) into opcodes (i.e. `&[u8]`) returning the number of bytes writte.
`fn decode(mem:&[u8]) -> Result<(usize, Self), DecodeError>` decodes an instruction from a `&[u8]` and
returns the instruction and number of bytes read.

Opcodes are defined by attributes on the corresponding `enum`-variants. The opcodes can be defined as
binary or hex strings, can contain underscores for readability and variable names. This way the user can
encode the position of encoded variables, but it also means that variable names can only be one symbol long (for now)
and cannot contain hex digits. 

##Example
```
use imperative::InstructionSet;

#[derive(InstructionSet)]
enum Is {
    //constant opcode
    #[opcode = "0x0000"]
    Nop,
    //hex opcode with split variable x
    #[opcode = "0x1x0x"]
    Inc{x:u8},
    //hex opcode with three variables
    #[ opcode = "0x2xxyyzz" ]
    Add{x:u8, y:u8, z:u8},
    //bin opcode with two variables and underscores for readability
    #[ opcode = "0b100000000_xxxxyyyyy_xyxyxyxy" ]
    Mov{x:u8, y:i8},
}

fn main() {
    let mut mem = [0u8; 1024];
    let (num_bytes, Is::Nop) = Is::decode(&mem).unwrap();
    let instruction = Is::Add{x:0xab, y:0xcd, z:0xef};
    assert_eq!(4, instruction.encode(&mut mem[100..]);
    assert_eq!([0x2a, 0xbc, 0xde, 0xf0], mem[100..104])
}

```