use syn::{Ident, Generics, braced, Attribute, Token, Error, Visibility};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Result as SynResult;
use quote::{ToTokens, quote};
use proc_macro2::TokenStream as TokenStream2;
use crate::instruction::{Instruction, CollisionGuard};

pub(crate) struct InstructionSet {
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
                fn decode(mem:&[u8]) -> ::std::result::Result<(::std::primitive::usize, #ident#generics), imperative_rs::DecodeError> {
                    #first_if
                    #(else #decode_blocks)* 
                    else {
                        Err(imperative_rs::DecodeError::UnknownOpcode)
                    }
                }
            }
        } else {
            quote!()
        };
        

        let encode_fn = quote!{
            fn encode(&self, buf:&mut [u8]) -> ::std::result::Result<::std::primitive::usize, imperative_rs::EncodeError>  {
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


