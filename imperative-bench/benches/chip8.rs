use criterion::{criterion_group, criterion_main, Criterion};
use imperative_rs::InstructionSet;
use std::fs::File;
use std::io::Read;

#[derive(InstructionSet, Debug)]
enum Chip8 {
    #[opcode = "0x00cn"]
    ScDown { n: u8 },
    #[opcode = "0x00e0"]
    Cls,
    #[opcode = "0x00ee"]
    Rts,
    #[opcode = "0x00fb"]
    ScRight,
    #[opcode = "0x00fc"]
    ScLeft,
    #[opcode = "0x00fe"]
    Low,
    #[opcode = "0x00ff"]
    High,
    #[opcode = "0x1nnn"]
    Jmp {
        #[variable = "n"]
        addr: u16,
    },
    #[opcode = "0x2nnn"]
    Jsr {
        #[variable = "n"]
        addr: u16,
    },
    #[opcode = "0x3xrr"]
    SkEq {
        #[variable = "x"]
        reg: u8,
        #[variable = "r"]
        rhs: u8,
    },
    #[opcode = "0x4xrr"]
    SkNe {
        #[variable = "x"]
        reg: u8,
        #[variable = "r"]
        rhs: u8,
    },
    #[opcode = "0x5xy0"]
    SkEq2 {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x6xrr"]
    Mov {
        #[variable = "x"]
        reg: u8,
        #[variable = "r"]
        rhs: u8,
    },
    #[opcode = "0x7xrr"]
    Add {
        #[variable = "x"]
        reg: u8,
        #[variable = "r"]
        rhs: u8,
    },
    #[opcode = "0x8xy0"]
    Mov2 {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x8xy1"]
    Or {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x8xy2"]
    And {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x8xy3"]
    Xor {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x8xy4"]
    Add2 {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x8xy5"]
    Sub {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x8x06"]
    Shr {
        #[variable = "x"]
        reg: u8,
    },
    #[opcode = "0x8xy7"]
    Rsb {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0x8x0e"]
    Shl {
        #[variable = "x"]
        reg1: u8,
    },
    #[opcode = "0x9xy0"]
    SkNe2 {
        #[variable = "x"]
        reg1: u8,
        #[variable = "y"]
        reg2: u8,
    },
    #[opcode = "0xannn"]
    Mvi {
        #[variable = "n"]
        addr: u16,
    },
    #[opcode = "0xbnnn"]
    Jmi {
        #[variable = "n"]
        addr: u16,
    },
    #[opcode = "0xcxkk"]
    Rand {
        #[variable = "x"]
        reg1: u8,
        k: u8,
    },
    #[opcode = "0xdxyn"]
    Sprite {
        #[variable = "x"]
        pos_x: u8,
        #[variable = "y"]
        pos_y: u8,
        #[variable = "n"]
        height: u8,
    },
    #[opcode = "0xek9e"]
    SkPr {
        #[variable = "k"]
        key: u8,
    },
    #[opcode = "0xeka1"]
    SkUp {
        #[variable = "k"]
        key: u8,
    },
    #[opcode = "0xfr07"]
    GDelay {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr0a"]
    Key {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr15"]
    SDealy {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr18"]
    SSound {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr1e"]
    Adi {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr29"]
    Font {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr30"]
    XFont {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr33"]
    Bcd {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr55"]
    Str {
        #[variable = "r"]
        reg: u8,
    },
    #[opcode = "0xfr65"]
    Ldr {
        #[variable = "r"]
        reg: u8,
    },
}

fn chip8_benches(c: &mut Criterion) {
    let file_name = "benches/test.ch8";
    let mut rom_file =
        File::open(file_name).expect(&format!("Couldn't open rom file at: {}", file_name));
    let mut rom = vec![];
    rom_file
        .read_to_end(&mut rom)
        .expect(&format!("Couldn't read rom file at: {}", file_name));

    c.bench_function("decoding chip8 rom", |b| {
        b.iter(|| {
            let mut pc = 0;
            while pc < rom.len() {
                let (num_bytes, _) = Chip8::decode(&rom[pc..]).expect(&format!(
                    "Failed to decode instruction at {}: {:x?}",
                    pc,
                    &rom[pc..pc + 2]
                ));
                pc += num_bytes;
            }
        })
    });

    let mut instructions = Vec::with_capacity(rom.len());
    let mut pc = 0;
    while let Ok((num_bytes, instr)) = Chip8::decode(&rom[pc..]) {
        pc += num_bytes;
        instructions.push(instr);
    }

    let mut target_rom = [0u8; 2];
    c.bench_function("encoding chip8 rom", |b| {
        b.iter(|| {
            for instr in &instructions {
                let _ = instr
                    .encode(&mut target_rom)
                    .expect(&format!("Failed to encode instruction: {:?}", instr));
            }
        })
    });
}

criterion_group!(benches, chip8_benches);
criterion_main!(benches);
