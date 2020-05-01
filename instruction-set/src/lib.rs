#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate lazy_static;
use crate::proc_macro::{TokenStream};
use crate::proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::HashMap;
use quote::{ToTokens, quote};
use syn::*;
use syn::Result as SynResult;
use syn::punctuated::Punctuated;
use syn::parse::{ParseStream, Parse};
use syn::token;
use syn::spanned::Spanned;


lazy_static!{
    static ref HEX_TO_BIN:HashMap<char, &'static str> = vec!(('0', "0000"), ('1', "0001"), ('2', "0010"), ('3', "0011"), ('4', "0100"), ('5', "0101"), ('6', "0110"), ('7', "0111")
        , ('8', "1000"), ('9', "1001"), ('a', "1010"), ('b', "1011"), ('c', "1100"), ('d', "1101"), ('e', "1110"), ('f', "1111")).into_iter().collect();
}


fn hex_to_bin_string(src_str:&str) -> String {
    let mut res_str = String::with_capacity(4*src_str.len());
    for c in src_str.chars().skip(2) {
        if let Some(bin_str) = HEX_TO_BIN.get(&c) {
            res_str.push_str(bin_str);
        } else {
            for _ in 0..4 { res_str.push(c); }
        }
    }
    res_str 
}

struct CollisionGuard<'a>(Vec<&'a Opcode>);

impl<'a> CollisionGuard<'a> {

    fn new() -> Self {
        CollisionGuard(vec!())
    }

    fn is_collision(op1:&Opcode, op2: &Opcode) -> bool {
        for (c1, c2) in op1.collision_iter().zip(op2.collision_iter()){
            if c1 != '*' && c2 != '*' {
                if c1 != c2 {
                    return false;
                } else {
                    continue;
                }
            }
        }
        true 
    }


    fn collides_or_insert(&mut self, opcode:&'a Opcode) -> Option<&'a Opcode> {
        for ex_opcode in & self.0 {
            if CollisionGuard::is_collision(opcode, ex_opcode) {
                return Some(ex_opcode); 
            }
        }
        self.0.push(opcode);
        None
    }
}

struct InstructionSet {
    ident:Ident,
    generics:Generics,
    instructions:Punctuated<Instruction, Token!(,)>,
}

impl Parse for InstructionSet {
    
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _ =  input.call(Attribute::parse_outer)?;
        let _ = Visibility::parse(input)?;
        let _:Token!(enum) = input.parse()?;
        let ident = Ident::parse(input)?;
        let generics = Generics::parse(input)?;
        let content;
        let _ = braced!(content in input);
        let instructions = content.parse_terminated(Instruction::parse)?;
        let mut cg = CollisionGuard::new();
        for instr in &instructions {
            if let Some(colliding_opcode) = cg.collides_or_insert(&instr.opcode()) {
                let mut err = syn::Error::new(instr.opcode().span(), "Opcode collides with other opdcode");
                let other = Error::new(colliding_opcode.span(), "Collides with this opcode");
                err.combine(other);
                return Err(err);
            }
        }
        Ok(InstructionSet{
            ident,
            generics,
            instructions,
        })
    }
}

impl ToTokens for InstructionSet {
    
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.ident;
        let generics = &self.generics;
        

        let (encode_blocks, decode_blocks):(Vec<TokenStream2>, Vec<TokenStream2>) = self.instructions.iter().map(|instr| instr.codec_blocks()).unzip();
         
        let mut decode_blocks = decode_blocks.iter();
        let decode_fn = if let Some(first_if) = decode_blocks.next() {
            quote!{
                fn decode(mem:&[u8]) -> std::result::Result<(usize, #ident#generics), istrait::DecodeError> {
                    #first_if
                    #(else #decode_blocks)* 
                    else {
                        Err(istrait::DecodeError::UnknownOpcode)
                    }
                }
            }
        } else {
            quote!()
        };
        

        let encode_fn = quote!{
            fn encode(&self, buf:&mut [u8]) -> std::result::Result<usize, istrait::EncodeError>  {
                match self {
                    #(#encode_blocks)*
                }
            }
        };
        
        tokens.extend(quote!{
            impl InstructionSet for #ident#generics {
                #encode_fn
                #decode_fn
            }
        });
    }
}

struct Opcode {
    bytes:Vec<[char; 8]>,
    span:Span,
}

impl Opcode {

    fn num_bytes(&self) -> usize {
        self.bytes.len()
    }

    fn get_position_map_of<'a>(&'a self, var_name:char) -> Box<dyn Iterator<Item=(usize, (usize, usize))> + 'a> {
        Box::new(self.bytes.iter()
            .enumerate()
            .rev() //step through bytes in reverse order
            .flat_map(move |(byte_idx, byte)| { //iterate over byte in reverse
                byte.iter()
                    .enumerate()
                    .rev()
                    .filter(move |(_, c)| **c == var_name) //filter positions that belong to this var
                    .map(move |(bit_idx, _)| (byte_idx, bit_idx))}) //save bit and byte position
            .enumerate() //count how many positions there are
            .map(move |(idx, opcode_pos)| (idx, opcode_pos))) //fill up bits in target starting at least significant bit
    }
        

    fn mask_strings<'a>(&'a self) -> Box<dyn Iterator<Item=String> + 'a> {
        Box::new(self.bytes.iter().map(|byte|{
            let mut mask = "".to_string();
            for c in byte {
                if *c == '0' || *c == '1' {
                    mask.push('1');
                } else {
                    mask.push('0');
                }
            }
            mask
        }))
    }

    fn code_strings<'a>(&'a self) -> Box<dyn Iterator<Item=String> + 'a> {
        Box::new(self.bytes.iter().map(|byte|{
            let mut code = "".to_string();
            for c in byte {
                if *c == '0' || *c == '1' {
                    code.push(*c);
                } else {
                    code.push('0');
                }
            }
            code
        }))
        
    }
    
    fn collision_iter<'a>(&'a self) -> Box<dyn Iterator<Item=char> + 'a> {
        Box::new(self.bytes.iter()
            .flatten()
            .map(|&c| if c == '1' || c == '0' { c } else { '*'})
        )
    }

    fn build_var_decoders(&self, variables:&HashMap<char, (&Ident, &Type)>) -> TokenStream2 {
        let mut var_decoders = vec!();
        for (c, (ident, ty)) in variables.iter() {
            let mut masks = vec!();
            let mut src_bytes = vec!();
            let mut left_shifts = vec!();
            let mut right_shifts = vec!();
            let mut src_pos_iter = self.get_position_map_of(*c).peekable();
            while let Some((tar_bit, (src_byte, src_bit))) = src_pos_iter.next() {
                let mut mask = vec!('0', '0', '0', '0', '0', '0', '0', '0');
                mask[src_bit] = '1';

                let mut mask_str:String = "0b".to_string();
                mask_str.extend(mask.iter());
                masks.push(LitInt::new(&mask_str, self.span()));
                src_bytes.push(src_byte);
                right_shifts.push(7 - src_bit);
                left_shifts.push(tar_bit);
            }
            var_decoders.push(quote!{
                #ident: #((((mem[#src_bytes] & #masks) >> #right_shifts) as #ty) << #left_shifts)|*
            })
        }

        quote!{
            #(#var_decoders),*
        }
    }

    fn build_encoder(&self, variables:&HashMap<char, (&Ident, &Type)>) -> TokenStream2 {
        let code_bytes:Vec<LitInt> = self.code_strings().map(|s| LitInt::new(&format!("0b{}", s), self.span())).collect();
        let num_bytes = self.bytes.len();
        let code_indices = 0..num_bytes;
        let mut tokens = quote!{
                if buf.len() < #num_bytes {
                    return Err(istrait::EncodeError::UnexpectedEOF);
                }
                #(buf[#code_indices] = #code_bytes);*;
        };
        
        for (c, (ident, _)) in variables.iter() {
            let mut positions_iter = self.get_position_map_of(*c).peekable();
            while let Some((src_bit, (tar_byte, tar_bit))) = positions_iter.next() {
                let mut mask = vec!('0', '0', '0', '0', '0', '0', '0', '0');
                mask[tar_bit] = '1';
                let shift = src_bit as isize - (7 - tar_bit as isize); 
                let mask_str:String = format!("0b{}", mask.iter().collect::<String>());
                let mask = LitInt::new(&mask_str, self.span());
                tokens.extend(quote!{
                buf[#tar_byte] |= ((#ident >> #shift) & #mask) as u8;
                });
            }
        }
        tokens.extend(quote!{return Ok(#num_bytes);});
        tokens
    }

    fn build_match_conditions(&self) -> TokenStream2 {
        let num_bytes = self.bytes.len();
        let mut tokens = quote!{ mem.len() >= #num_bytes }; 
        for (idx, (code_str, mask_str)) in self.code_strings().zip(self.mask_strings()).enumerate() {
            let mask = LitInt::new(&format!("0b{}", mask_str), self.span);
            let code = LitInt::new(&format!("0b{}", code_str), self.span);
            tokens.extend(quote!{
                && mem[#idx] & #mask == #code
            });
        }
        tokens
    }
    
    fn span(&self) -> Span {
        self.span 
    }
}

impl Parse for Opcode {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _:Token![=] = input.parse()?;
        let literal:LitStr = input.parse()?;
        let mut literal_string:String = literal.value();
        literal_string.retain(|c| c != '_');
        let prefix:Vec<char> = literal_string.chars().take(2).collect();
        if prefix.len() != 2 || prefix[0] != '0' || (prefix[1] != 'x' && prefix[1] != 'b') {
            Err(Error::new(literal.span(), "Invalid opcode. Valid opcodes start with either '0x' or '0b' followed by at least one digit/variable"))
        } else {
            let code:Vec<char> = if prefix[1] == 'x' {
                hex_to_bin_string(&literal_string).chars().collect()
            } else {
                literal_string.chars().skip(2).collect()
            };
            let mut bytes = vec!();
            for (pos, c) in code.iter().enumerate() {
                let bit_idx = pos % 8;
                let byte_idx = (pos - bit_idx)/8;
                if bytes.len() <= byte_idx {
                    bytes.push(['0'; 8]);
                }
                bytes[byte_idx][bit_idx] = *c;
            }
            Ok(Opcode{ bytes, span:literal.span() })
        }
    }
}

enum Instruction {
    WithVars(InstrWithVars),
    Unit(UnitInstr),
}

impl Instruction {
    
    fn codec_blocks(&self) -> (TokenStream2, TokenStream2) {
        match self {
            Instruction::WithVars(instr) => instr.codec_blocks(),
            Instruction::Unit(instr)  => instr.codec_blocks(),
        }
    }

    fn opcode(&self) -> &Opcode {
        match self {
            Instruction::WithVars(instr) => &instr.opcode,
            Instruction::Unit(instr) => &instr.opcode,
        }
    }

    fn get_opcode(ident:&Ident, attrs:Vec<Attribute>) -> SynResult<Opcode> {
        for attribute in attrs  {
            if attribute.path.is_ident("opcode") {
                let tokens:TokenStream = attribute.tokens.into();
                return syn::parse(tokens);
            }
        }
        Err(Error::new(ident.span(), format!("No opcode defined for Instruction {}. Define Opcodes by adding #[opcode = \"0x...\"] above the Instruction", ident.to_string())))
    }
}

impl Parse for Instruction {
    
    fn parse(input: ParseStream) -> SynResult<Self> {
        let attr:Vec<Attribute> =  input.call(Attribute::parse_outer)?;
        let _:Visibility = input.parse()?;
        let ident:Ident = input.parse()?;
        let opcode = Instruction::get_opcode(&ident, attr)?;
        if input.peek(token::Brace) {
            Ok(Instruction::WithVars(InstrWithVars{
                ident,
                fields:FieldsNamed::parse(input)?,
                opcode
            }))
        } else if input.peek(token::Paren) {
            let fields:FieldsUnnamed = input.parse()?;
            Err(Error::new(fields.span(), "Variants with unnamed fields not supported. Enum::Variant{X:usize, Y:u32} notation"))
        } else {
            Ok(Instruction::Unit(UnitInstr{
                ident,
                opcode
            }))
        }
    }
}

impl ToTokens for Instruction {
    
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Instruction::WithVars(instr) => instr.to_tokens(tokens),
            Instruction::Unit(instr) => instr.to_tokens(tokens),
        }
    }
}

struct UnitInstr {
    ident:Ident,
    opcode:Opcode,
}

impl UnitInstr {
    
    fn codec_blocks(&self) -> (TokenStream2, TokenStream2) {
        let self_ident = &self.ident;
        let num_bytes = self.opcode.num_bytes();
        let conditions = self.opcode.build_match_conditions();

        let decode_block = quote!{
            if #conditions { Ok((#num_bytes, Self::#self_ident)) }
        };

        let code_strings:Vec<LitInt> = self.opcode.code_strings().map(|s| LitInt::new(&format!("0b{}", s), self.opcode.span())).collect();
        let byte_indices = 0..code_strings.len();

        let encode_block = quote!{
            Self::#self_ident => {
                if buf.len() < #num_bytes {
                    return Err(istrait::EncodeError::UnexpectedEOF);
                }
                #(buf[#byte_indices] = #code_strings);*;
                return Ok(#num_bytes);
            }, 
        };

        (encode_block, decode_block)
    }
}

impl ToTokens for UnitInstr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ident.to_tokens(tokens); 
    }
}

struct InstrWithVars {
    ident:Ident,
    fields:FieldsNamed,
    opcode:Opcode,
}

impl InstrWithVars {
    fn map_variables(&self) -> HashMap<char, (&Ident, &Type)> {
        let mut variables = HashMap::new();
        for f in self.fields.named.iter() {
            let ident = f.ident.as_ref().unwrap();
            let var_name = ident.to_string();
            if var_name.len() != 1 {
                ident.span().unwrap().error("Variables names have to be 1 symbol long").emit();
                continue;
            }
            let var_name = var_name.chars().next().unwrap();
            if var_name.is_lowercase() && var_name.is_ascii_hexdigit() {
                ident.span().unwrap().error("Variables names can't be lower case hexdigits").emit();
                continue;
            }
            if var_name.is_numeric() {
                ident.span().unwrap().error("Variables names can't be numeric").emit();
                continue;
            }
            variables.insert(var_name, (f.ident.as_ref().unwrap(), &f.ty));
        }
        variables
    }
    
    fn codec_blocks(&self) -> (TokenStream2, TokenStream2) {
        let variables = self.map_variables();

        for c in self.opcode.bytes.iter().flat_map(|byte| byte.iter()) {
            if !(variables.contains_key(c) || *c == '0' || *c == '1') {
                self.opcode.span().unwrap().error(format!("Code contains {} which is neither a variable name nor a valid digit.", c)).emit();
                return (quote!(if false {}), quote!{}); 
            }
        }
        
        let num_bytes = self.opcode.num_bytes();
        let var_decoders = self.opcode.build_var_decoders(&variables);
        let match_conditions = self.opcode.build_match_conditions();
        let ident = &self.ident;
        let decode_block = quote!{
            if #match_conditions {
                Ok((#num_bytes, Self::#ident{
                    #var_decoders
                }))
            }
        };

        let encoder = self.opcode.build_encoder(&variables);
        let var_idents:Vec<&Ident> = variables.iter().map(|(_, (ident, _))| *ident).collect();

        let encoder_block = quote!{
            Self::#ident{ #(#var_idents),* } => {#encoder},
        };

        (encoder_block, decode_block)

    }
}


impl ToTokens for InstrWithVars {

    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ident.to_tokens(tokens); 
        self.fields.to_tokens(tokens);
    }
}

#[proc_macro_derive(InstructionSet, attributes(opcode))]
pub fn define_instructionset(input: TokenStream) -> TokenStream {
    let instruction_set = parse_macro_input!(input as InstructionSet);
    let tokens = quote!{#instruction_set};
    tokens.into()
}

