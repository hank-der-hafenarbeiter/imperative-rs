# Imperative [![Latest Version]][crates.io] [![Documentation]][docs.rs]

[Documentation]: https://img.shields.io/badge/docs.rs-rustdoc-green
[Latest Version]: https://img.shields.io/crates/v/imperative_rs.svg
[docs.rs]: https://docs.rs/imperative-rs/
[crates.io]: https://crates.io/crates/imperative-rs

`imperative` tries to make it easier to define instruction sets. Using `imperative` 
instruction sets are defined by an `enum` and the `InstructionSet` trait, which can
be automatically derived. The result is an enum which can decode from `&[u8]` or encode
into `&mut [u8]`. The instructions can be of variable length and can contain multiple
variables of whatever type, as long as they can be parsed from binary or hexadecimal
integer strings.

Instruction sets created this way can be used to write emulators. When the instruction
set is defined, all that's left to do is to `match` the decoded instruction and implement
the desired behaviour in the match arms.

This project is still in it's infancy so if you plan to build a reliable emulator, you
are probably better of writing the codec for your instruction set by hand. 

## Example

```rust
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
