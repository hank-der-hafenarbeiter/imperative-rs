#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate lazy_static;
use crate::proc_macro::{TokenStream};
use crate::proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::HashMap;
use quote::{ToTokens, quote, TokenStreamExt};
use syn::*;
use syn::Result as SynResult;
use syn::punctuated::Punctuated;
use syn::parse::{ParseStream, Parse};
use syn::token::Brace;
use syn::token;
use syn::spanned::Spanned;


lazy_static!{
    static ref HEX_TO_BIN:HashMap<char, &'static str> = vec!(('0', "0000"), ('1', "0001"), ('2', "0010"), ('3', "0011"), ('4', "0100"), ('5', "0101"), ('6', "0110"), ('7', "0111")
        , ('8', "1000"), ('9', "1001"), ('a', "1010"), ('b', "1011"), ('c', "1100"), ('d', "1101"), ('e', "1110"), ('f', "1111")).into_iter().collect();
}

fn ensure_bin_str(discriminant:LitStr) -> Result<LitStr> {
    let src_str = discriminant.value();
    let mut src_str_iter = src_str.chars(); 
    let _ = src_str_iter.next().ok_or(Error::new(discriminant.span(), "Found empty string. Expected String as instruction code."))?;
    let second = src_str_iter.next().ok_or(Error::new(discriminant.span(), "Found instruction code without prefix (0x or 0b)."))?;
    if second == 'x' {
        let mut binary_str = "0x".to_string(); 
        for c in src_str_iter {
            if let Some(s) = HEX_TO_BIN.get(&c) {
                binary_str.push_str(s);
            } else {
                for _ in 0..4 { binary_str.push(c)}
            }
        }
        Ok(LitStr::new(&binary_str, discriminant.span()))
    } else if second == 'b' {
        Ok(discriminant)
    } else {
        Err(Error::new(discriminant.span(), "Instruction codes have to be prefixed by either '0x' or '0b'."))
    }
}

//enum ParseTree {
//    Leaf { instr:Instruction, last_byte:u8 }
//    InnerNode { byte:u8, depth:usize, children:
//}

struct CollisionGuard(Vec<(String, Span)>);

impl CollisionGuard {

    fn new() -> Self {
        CollisionGuard(vec!())
    }

    fn is_collision(s1:&str, s2: &str) -> bool {
         for (c1, c2) in s1.chars().zip(s2.chars()) {
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


    fn collides_or_insert(&mut self, entry: (String, Span)) -> Option<(String, Span)> {
        let (ref new_cg, _) = entry;
        for entry in & self.0 {
            if CollisionGuard::is_collision(new_cg, &entry.0) {
                return Some(entry.clone()); 
            }
        }
        self.0.push(entry);
        None
    }
}

struct InstructionSet {
    attr:Vec<Attribute>,
    vis:Visibility,
    enum_token: Token!(enum),
    ident:Ident,
    generics:Generics,
    brace_token:Brace,
    instructions:Punctuated<Instruction, Token!(,)>,
}

impl Parse for InstructionSet {
    
    fn parse(input: ParseStream) -> SynResult<Self> {
        let content;
        Ok(InstructionSet{
            attr:input.call(Attribute::parse_outer)?,
            vis:input.parse()?,
            enum_token:input.parse()?,
            ident: input.parse()?,
            generics:input.parse()?,
            brace_token: braced!(content in input),
            instructions: content.parse_terminated(Instruction::parse)?,
        })
    }
}

impl ToTokens for InstructionSet {
    
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(&self.attr);
        self.vis.to_tokens(tokens);
        self.enum_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens|{
            self.instructions.to_tokens(tokens); 
        });

        let mut collision_guard = CollisionGuard::new();
        let if_blocks:Vec<TokenStream2> = self.instructions.iter().map(|instr| instr.if_block(&mut collision_guard)).collect();
        let ident = &self.ident;
        tokens.extend(quote!{
            impl #ident {
                pub fn parse(mem:u32) -> Result<#ident, ()> {
                    #(#if_blocks)* 
                    Err(())
                }
            }
        });
    }
}

enum Instruction {
    WithVars(InstrWithVars),
    Unit(UnitInstr),
}

impl Instruction {
    
    fn if_block(&self, existing_codes:&mut CollisionGuard) -> TokenStream2 {
        match self {
            Instruction::WithVars(instr) => instr.if_block(existing_codes),
            Instruction::Unit(instr)  => instr.if_block(existing_codes),
        }
    }
}

impl Parse for Instruction {
    
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _:Vec<Attribute> =  input.call(Attribute::parse_outer)?;
        let _:Visibility = input.parse()?;
        let ident:Ident = input.parse()?;
        if input.peek(token::Brace) {
            Ok(Instruction::WithVars(InstrWithVars{
                ident,
                fields:FieldsNamed::parse(input)?,
                discriminant: {
                    let _:Token![=] = input.parse()?;
                    ensure_bin_str(input.parse()?)?
                },
            }))
        } else if input.peek(token::Paren) {
            let fields:FieldsUnnamed = input.parse()?;
            Err(Error::new(fields.span(), "Variants with unnamed fields not supported. Enum::Variant{X:usize, Y:u32} notation"))
        } else {
            Ok(Instruction::Unit(UnitInstr{
                ident,
                discriminant:{
                    let _:Token![=] = input.parse()?;
                    ensure_bin_str(input.parse()?)?
                }
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
    discriminant:LitStr,
}

impl UnitInstr {
    
    fn if_block(&self, existing_codes: &mut CollisionGuard) -> TokenStream2 {
        let match_str = &self.discriminant.value();
        if let Some(other) = existing_codes.collides_or_insert((match_str.clone(), self.ident.span())) {
            let diag = self.ident.span().unwrap().error("Same instruction code has been used before.");
            diag.span_note(other.1.unwrap(), "Previously used here.").emit();
        }
        let match_int = LitInt::new(&match_str, self.ident.span());
        let self_ident = &self.ident;
        quote!{
            if mem == #match_int { return Ok(Self::#self_ident); }
        }
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
    discriminant:LitStr,
}

impl InstrWithVars {
    
    fn if_block(&self, collision_guard:&mut CollisionGuard) -> TokenStream2 {
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

        let discriminant:Vec<char> = self.discriminant.value().chars().collect();

        for c in discriminant.iter().skip(2) {
            if !variables.contains_key(c) && !(*c == '0' || *c == '1') {
                self.discriminant.span().unwrap().error(format!("Code contains {} which is neither a variable name nor a valid hex digit (0-f).", c)).emit();
                return TokenStream2::new(); 
            }
        }
        

        let prefix_len = 2; // always '0x' for now
        let num_bytes = discriminant.len() - prefix_len;
        let mut empty_mask = vec!['0', 'b'];
        while empty_mask.len() < num_bytes + prefix_len { 
            empty_mask.push('0')
        }
        let mut var_setters = vec!();

        for (var_name, (ident, ty)) in &variables {
            let mut quote = quote!{ 0 };
            let mut pos_iter = discriminant.iter().enumerate().skip(2).filter(|(_, c)| *c == var_name).map(|(i, _)| i).peekable();
            
            while let Some(pos) = pos_iter.next() { 
                let mask_str:String = empty_mask.iter().enumerate().map(|(i, &c)| if i == pos { '1' } else { c }).collect();
                let mask = LitInt::new(&mask_str, self.discriminant.span());
                let shift = **pos_iter.peek().as_ref().unwrap_or(&&(num_bytes + prefix_len)) - pos - 1; 
                quote = quote!{((#quote | mem & #mask) >> #shift)};
            }
            var_setters.push(quote!{#ident: #quote as #ty});
        }

        let discriminant = self.discriminant.value();
        let ident = &self.ident;
        let mask_str = format!("0b{}", discriminant.chars().skip(prefix_len).map(|c| if variables.contains_key(&c) { '1' } else { '0' }).collect::<String>());
        let mask = LitInt::new(&mask_str, self.discriminant.span());
        let code_str =  format!("0b{}", discriminant.chars().skip(prefix_len).map(|c| if variables.contains_key(&c) { '1' } else { c }).collect::<String>());
        let code = LitInt::new(&code_str, self.discriminant.span()); 


        let collision_str = discriminant.chars().skip(prefix_len).map(|c| if variables.contains_key(&c) { '*' }  else { c }).collect();
        if let Some(other) = collision_guard.collides_or_insert((collision_str, self.ident.span())) {
            let diag = self.ident.span().unwrap().error("Same instruction code has been used before.");
            diag.span_note(other.1.unwrap(), "Previously used here.").emit();
        } 

        quote!{
            if mem | #mask == #code {
                return Ok(Self::#ident{
                    #(#var_setters),*
                });
            }
        }

    }
}


impl ToTokens for InstrWithVars {

    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ident.to_tokens(tokens); 
        self.fields.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn define_instructionset(input: TokenStream) -> TokenStream {
    let instruction_set = parse_macro_input!(input as InstructionSet);
    let mut tokens = TokenStream2::new();
    instruction_set.to_tokens(&mut tokens);
    tokens.into()
}

