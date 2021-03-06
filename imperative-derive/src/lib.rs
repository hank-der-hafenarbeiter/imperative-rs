extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate lazy_static;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn::*;

mod instruction;
mod instructionset;
mod matcher;

use instructionset::InstructionSet;

#[proc_macro_derive(InstructionSet, attributes(opcode, variable))]
pub fn derive_instructionset(input: TokenStream) -> TokenStream {
    let instruction_set = parse_macro_input!(input as InstructionSet);
    let tokens = quote! {#instruction_set};
    tokens.into()
}
