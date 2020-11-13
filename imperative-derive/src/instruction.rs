use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;
use std::mem;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::{Brace, Paren};
use syn::Result as SynResult;
use syn::{
    parse_quote, Attribute, Error, FieldsNamed, FieldsUnnamed, Ident, LitInt, LitStr, Token, Type,
    Visibility,
};

lazy_static! {
    static ref HEX_TO_BIN: HashMap<char, &'static str> = vec!(
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('a', "1010"),
        ('b', "1011"),
        ('c', "1100"),
        ('d', "1101"),
        ('e', "1110"),
        ('f', "1111")
    )
    .into_iter()
    .collect();
}

fn is_supported_type(ty: &Type) -> bool {
    //!Returns true if the input type is supported by this library else returns false
    *ty == parse_quote!(u8)
        || *ty == parse_quote!(i8)
        || *ty == parse_quote!(u16)
        || *ty == parse_quote!(i16)
        || *ty == parse_quote!(u32)
        || *ty == parse_quote!(i32)
        || *ty == parse_quote!(u64)
        || *ty == parse_quote!(i64)
        || *ty == parse_quote!(u128)
        || *ty == parse_quote!(i128)
        || *ty == parse_quote!(usize)
        || *ty == parse_quote!(isize)
        || *ty == parse_quote!(bool)
}

fn size_of(ty: &Type) -> usize {
    //! Returns the number of bits the input type fills
    if *ty == parse_quote!(u8) {
        8
    } else if *ty == parse_quote!(i8) {
        8
    } else if *ty == parse_quote!(u16) {
        16
    } else if *ty == parse_quote!(i16) {
        16
    } else if *ty == parse_quote!(u32) {
        32
    } else if *ty == parse_quote!(i32) {
        32
    } else if *ty == parse_quote!(u64) {
        64
    } else if *ty == parse_quote!(i64) {
        64
    } else if *ty == parse_quote!(u128) {
        128
    } else if *ty == parse_quote!(i128) {
        128
    } else if *ty == parse_quote!(usize) {
        8 * mem::size_of::<usize>()
    } else if *ty == parse_quote!(isize) {
        8 * mem::size_of::<isize>()
    } else if *ty == parse_quote!(bool) {
        1
    } else {
        panic!(format!(
            "Unexpected type {:?} when getting memory size.",
            ty
        ));
    }
}

pub(crate) enum Instruction {
    WithVars(InstrWithVars),
    Unit(UnitInstr),
}

impl Instruction {
    pub(crate) fn encoder_block(&self) -> TokenStream2 {
        match self {
            Instruction::WithVars(instr) => instr.encoder_block(),
            Instruction::Unit(instr) => instr.encoder_block(),
        }
    }

    pub(crate) fn decoder_block(&self) -> TokenStream2 {
        match self {
            Instruction::WithVars(instr) => instr.decoder_block(),
            Instruction::Unit(instr) => instr.decoder_block(),
        }
    }
    pub(crate) fn opcode(&self) -> &Opcode {
        match self {
            Instruction::WithVars(instr) => &instr.opcode,
            Instruction::Unit(instr) => &instr.opcode,
        }
    }
}

impl Parse for Instruction {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let attr: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let _: Visibility = input.parse()?;
        let ident: Ident = input.parse()?;
        let opcode = Opcode::from_attrs(&ident, attr)?;
        if input.peek(Brace) {
            let fields = input.parse()?;
            let instr = InstrWithVars::new(ident, fields, opcode)?;
            Ok(Instruction::WithVars(instr))
        } else if input.peek(Paren) {
            let fields: FieldsUnnamed = input.parse()?;
            Err(Error::new(fields.span(), "Variants with unnamed fields not supported. Use Enum::Variant{X:usize, Y:u32} notation"))
        } else {
            Ok(Instruction::Unit(UnitInstr { ident, opcode }))
        }
    }
}

pub(crate) struct UnitInstr {
    ident: Ident,
    opcode: Opcode,
}

impl UnitInstr {
    fn decoder_block(&self) -> TokenStream2 {
        let self_ident = &self.ident;
        let num_bytes = self.opcode.num_bytes();
        let conditions = self.opcode.build_match_conditions();

        quote! {
            if #conditions { Ok((#num_bytes, Self::#self_ident)) }
        }
    }

    fn encoder_block(&self) -> TokenStream2 {
        let self_ident = &self.ident;
        let num_bytes = self.opcode.num_bytes();
        let code_strings: Vec<LitInt> = self
            .opcode
            .code_strings()
            .map(|s| LitInt::new(&format!("0b{}", s), self.opcode.span()))
            .collect();
        let byte_indices = 0..code_strings.len();

        quote! {
            Self::#self_ident => {
                if buf.len() < #num_bytes {
                    return Err(imperative_rs::EncodeError::UnexpectedEOF);
                }
                #(buf[#byte_indices] = #code_strings);*;
                return Ok(#num_bytes);
            },
        }
    }
}

pub(crate) struct InstrWithVars {
    ident: Ident,
    opcode: Opcode,
    var_map: HashMap<char, (Ident, Type)>,
}

impl InstrWithVars {
    fn new(ident: Ident, fields: FieldsNamed, opcode: Opcode) -> SynResult<Self> {
        let instr = Self {
            ident,
            opcode,
            var_map: Self::map_variables(fields)?,
        };
        instr.check_opcode()?;
        Ok(instr)
    }

    fn check_opcode(&self) -> SynResult<()> {
        let mut res: SynResult<()> = Ok(());
        let mut variables: HashMap<&char, (&Ident, &Type, usize)> = self
            .var_map
            .iter()
            .map(|(c, (i, t))| (c, (i, t, 0)))
            .collect();

        for bit in self.opcode.bytes.iter().flatten() {
            match variables.get_mut(bit) {
                Some(entry) => entry.2 += 1,
                None => {
                    if *bit != '0' && *bit != '1' && *bit != '*' {
                        let err = Error::new(self.opcode.span,
                                &format!("Opcode contains {} which is neither a valid digit nor a variable name.", bit));
                        if let Err(ref mut total_error) = res {
                            total_error.combine(err);
                        } else {
                            res = Err(err);
                        }
                    }
                }
            }
        }
        //check for variables that are not used in the opcode.
        for (c, (ident, ty, num_bits)) in variables.iter() {
            if *num_bits == 0 {
                let err = Error::new(
                    ident.span(),
                    &format!(
                        "Variable {:?} (with symbol: {:?}) declared but never used in opcode.",
                        ident, c
                    ),
                );
                if let Err(ref mut total_error) = res {
                    total_error.combine(err);
                } else {
                    res = Err(err);
                }
            }
            if size_of(ty) < *num_bits {
                let err = Error::new(
                    ident.span(),
                    &format!(
                        "Variable {} (with symbol: {}) has fewer bits ({}) than positions in opcode ({}). (e.g. u8 with 9 bits in opcode)",
                        ident, c, size_of(ty), num_bits
                    ),
                );
                if let Err(ref mut total_error) = res {
                    total_error.combine(err);
                } else {
                    res = Err(err);
                }
            }
        }
        res
    }

    fn map_variables(fields: FieldsNamed) -> SynResult<HashMap<char, (Ident, Type)>> {
        //! This functions takes an iterator over the named fields of an enum variant (i.e.
        //! Enum::Variant{*var0:type0, var1:type1,...*}) which represent the variables of the
        //! instruction. It tries to construct a hashmap with the
        //! variable's symbol as key and the identifier and type as value.
        //! This is the connection between the symbols in the opcode and the variables in the
        //! instruction. When the variable name isn't modified by an attribute (i.e. `#[variable =
        //! "x"]`) the identifier is used as the variables symbol.
        //! For each variable/field this function checks:
        //! * if a valid `variable` attribute is given
        //! * if the variable is actually used in the opcode
        //! * if the variable name is valid (i.e. length 1, not hexdigit)
        //! * if the variable is of a supported type
        //! If a check fails for a field, the other checks are omitted. If any check fails for a
        //! given field the rest of the fields will still be checked.
        let mut res: SynResult<()> = Ok(());
        let mut variables = HashMap::new();
        for f in fields.named.into_iter() {
            let ident = f.ident.as_ref().unwrap();
            let var_name = if let Some(attr) =
                f.attrs.iter().find(|&attr| attr.path.is_ident("variable"))
            {
                let meta = attr.parse_meta()?;
                match meta {
                    syn::Meta::NameValue(name_value) => match name_value.lit {
                        syn::Lit::Str(str_lit) => str_lit.value(),
                        _ => {
                            let err = Error::new(
                                name_value.lit.span(),
                                "Variable names must be defined as string literals (e.g. \"x\"",
                            );
                            if let Err(ref mut total_error) = res {
                                total_error.combine(err);
                            } else {
                                res = Err(err);
                            }
                            continue;
                        }
                    },
                    _ => {
                        let err = Error::new(meta.span(), "Variable attribute declared but not value given (e.g. #[variable = \"x\"]");
                        if let Err(ref mut total_error) = res {
                            total_error.combine(err);
                        } else {
                            res = Err(err);
                        }
                        continue;
                    }
                }
            } else {
                ident.to_string()
            };

            if var_name.len() != 1 {
                let err = Error::new(ident.span(), "Variable names must be one symbol long");
                if let Err(ref mut total_error) = res {
                    total_error.combine(err);
                } else {
                    res = Err(err);
                }
                continue;
            }
            let var_name = var_name.chars().next().unwrap();
            if var_name.is_lowercase() && var_name.is_ascii_hexdigit() {
                let err = Error::new(ident.span(), "Variable names can't be lower case hexdigits");
                if let Err(ref mut total_error) = res {
                    total_error.combine(err);
                } else {
                    res = Err(err);
                }
                continue;
            }
            if var_name.is_numeric() {
                let err = Error::new(ident.span(), "Variable names can't be numeric");
                if let Err(ref mut total_error) = res {
                    total_error.combine(err);
                } else {
                    res = Err(err);
                }
                continue;
            }
            if !is_supported_type(&f.ty) {
                let err = Error::new(
                    ident.span(),
                    format!("The type {:?} is currently not supported.", &f.ty),
                );
                if let Err(ref mut total_error) = res {
                    total_error.combine(err);
                } else {
                    res = Err(err);
                }
                continue;
            }
            let (ident, ty) = (f.ident, f.ty);
            variables.insert(var_name, (ident.unwrap(), ty));
        }
        res.map(|_| variables)
    }

    fn decoder_block(&self) -> TokenStream2 {
        //! This constructs the last if-clause and decoder when decoding a byte buffer. This code
        //! is the leaf of the binary tree constructed by the matcher. But the matcher only rules
        //! out more and more instruction until one is left. This doesn't mean that this last
        //! instruction is actually correct. This function requests this match condition, the
        //! length of the instruction in bytes and the decoder for the instructions variables from
        //! the `Opcode` and puts it all together into a complete decoder for this instruction
        let num_bytes = self.opcode.num_bytes();
        let var_decoders = self.opcode.build_var_decoders(&self.var_map);
        let match_conditions = self.opcode.build_match_conditions();
        let ident = &self.ident;
        quote! {
            if #match_conditions {
                Ok((#num_bytes, Self::#ident{
                    #var_decoders
                }))
            }
        }
    }

    fn encoder_block(&self) -> TokenStream2 {
        //! This function constructs a match-arm for the encoding of this variable. This is used in
        //! the match block of the encoder function
        let ident = &self.ident;
        let encoder = self.opcode.build_encoder(&self.var_map);
        let var_idents: Vec<&Ident> = self.var_map.iter().map(|(_, (ident, _))| ident).collect();

        quote! {
            Self::#ident{ #(#var_idents),* } => {#encoder},
        }
    }
}

fn hex_to_bin_string(src_str: &str) -> String {
    //! The user can give opcodes either as hex or binary string. This function converts hex
    //! strings into binary strings, so there is only one kind internally.
    //! "0x0f" => "0b00001111"
    let mut res_str = String::with_capacity(4 * src_str.len());
    for c in src_str.chars().skip(2) {
        if let Some(bin_str) = HEX_TO_BIN.get(&c) {
            res_str.push_str(bin_str);
        } else {
            for _ in 0..4 {
                res_str.push(c);
            }
        }
    }
    res_str
}

/// This struct models the opcode given by the user. It offers multiple ways to iterate over
/// the opcode:
/// * `get_position_map_of(..)` returns an iterator over the bit and byte positions where the
/// supplied variable symbol appears and enumerates the positions. "0b0x0x"-> (0, (0, 8)), (1,
/// (0, 6))
/// * `mask_strings(..)` returns an iterator over a strings. Each of which is a integer literal
/// where all constant bits of the opcode are 1 and all variable bits are zero. `mem_byte & mask |
/// code` is true for all bytes exactly when this opcode is hit.
/// * `code_strings(..)` same as `mask_strings(..)` but returns codes: For each byte gives an int
/// literal that is '1' when the opcode is constant and '1' in the corresponding bit position.
/// * `collision_iter(..)` returns an iterator over strings for each byte. Each string contains a
/// '*' when the corresponding bit can be either '0' or '1' (i.e. containing a variable). If the
/// opcode is constantly '0' or '1' in this bit the string contains '0' or '1' in this position.
/// Additionally this struct produces encoder and decoder for the variables encoded in the opcode.
pub(crate) struct Opcode {
    bytes: Vec<[char; 8]>,
    span: Span,
}

impl Opcode {
    fn from_attrs(ident: &Ident, attrs: Vec<Attribute>) -> SynResult<Opcode> {
        //! Constructs an ´Opcode´ from an `Ident` and a `Vec<Attribute>`. Fails if no ´#[opcode =
        //! ".."]´ is defined.
        for attribute in attrs {
            if attribute.path.is_ident("opcode") {
                let tokens: TokenStream = attribute.tokens.into();
                return syn::parse(tokens);
            }
        }
        Err(Error::new(ident.span(), format!("No opcode defined for Instruction {}. Define Opcodes by adding #[opcode = \"0x...\"] above the Instruction", ident.to_string())))
    }

    fn num_bytes(&self) -> usize {
        //! length of this opcode in bytes
        self.bytes.len()
    }

    fn get_position_map_of<'a>(
        &'a self,
        var_name: char,
    ) -> Box<
        dyn Iterator<
                Item = (
                    ::std::primitive::usize,
                    (::std::primitive::usize, ::std::primitive::usize),
                ),
            > + 'a,
    > {
        //! For the given variable symbol returns the position where it occures (in (byte_idx,
        //! bit_idx) and how many bits it needs to be left shifted for it's target position.
        Box::new(
            self.bytes
                .iter()
                .enumerate()
                .rev() //step through bytes in reverse order
                .flat_map(move |(byte_idx, byte)| {
                    //iterate over byte in reverse
                    byte.iter()
                        .enumerate()
                        .rev()
                        .filter(move |(_, c)| **c == var_name) //filter positions that belong to this var
                        .map(move |(bit_idx, _)| (byte_idx, bit_idx))
                }) //save bit and byte position
                .enumerate() //count how many positions there are
                .map(move |(idx, opcode_pos)| (idx, opcode_pos)),
        ) //fill up bits in target starting at least significant bit
    }

    fn mask_strings<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
        //! Returns an iterator over the masks strings for each byte. Mask strings are used to
        //! identify this opcode: `mem[byte_idx] & mask[idx] == code[idx]` is true when and only
        //! when this opcode is hit
        Box::new(self.bytes.iter().map(|byte| {
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

    fn code_strings<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
        //! Returns an iterator over the code strings for each byte. For further explainations see
        //! `Opcode::mask_strings(..)` above.
        Box::new(self.bytes.iter().map(|byte| {
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

    pub(crate) fn collision_iter<'a>(&'a self) -> Box<dyn Iterator<Item = char> + 'a> {
        //! Returns an iterator over the bits of this opcode that is used by the `CollisionGuard`
        //! to check if this opcode can be distinguished from all other opcodes found so far.
        Box::new(
            self.bytes
                .iter()
                .flatten()
                .map(|&c| if c == '1' || c == '0' { c } else { '*' }),
        )
    }

    fn build_var_decoders(&self, variables: &HashMap<char, (Ident, Type)>) -> TokenStream2 {
        //! This function takes a variable map from the corresponding instruction and for each
        //! variable constructs a block that reads the corresponding bits in the memory, shifts
        //! them in the right position and bitwise or's them all together.
        //! This function should always return a valid (in terms of parseability) `TokenStream2`
        //! but if it an unsupported type should turn up here (which would be a bug in
        //! `Instruction::parse()` this function will cause a compile error pointing at the
        //! violating variable
        let mut var_decoders = vec![];
        for (c, (ident, ty)) in variables.iter() {
            let mut masks = vec![];
            let mut src_bytes = vec![];
            let mut left_shifts = vec![];
            let mut right_shifts = vec![];
            let mut src_pos_iter = self.get_position_map_of(*c).peekable();
            while let Some((tar_bit, (src_byte, src_bit))) = src_pos_iter.next() {
                let mut mask = vec!['0', '0', '0', '0', '0', '0', '0', '0'];
                mask[src_bit] = '1';

                let mut num_bits = 1; //how many bits will be decoded by this mask
                loop {
                    let next_is_neighbour =
                        src_pos_iter.peek().map_or(false, |(_, (byte, bit))| {
                            *byte == src_byte && src_bit == *bit + num_bits
                        });
                    if next_is_neighbour {
                        let (_, (_, next_src_bit)) = src_pos_iter.next().unwrap();
                        mask[next_src_bit] = '1';
                        num_bits += 1;
                    } else {
                        break;
                    }
                }

                let mut mask_str: String = "0b".to_string();
                mask_str.extend(mask.iter());
                masks.push(LitInt::new(&mask_str, self.span()));
                src_bytes.push(src_byte);
                right_shifts.push(7 - src_bit);
                left_shifts.push(tar_bit);
            }
            var_decoders.push(
                if *ty == parse_quote!(u8) ||
                *ty == parse_quote!(u16) ||
                *ty == parse_quote!(u32) ||
                *ty == parse_quote!(u64) ||
                *ty == parse_quote!(u128) ||
                *ty == parse_quote!(usize) ||
                *ty == parse_quote!(i8) ||
                *ty == parse_quote!(i16) ||
                *ty == parse_quote!(i32) ||
                *ty == parse_quote!(i64) ||
                *ty == parse_quote!(i128) ||
                *ty == parse_quote!(isize) {
                    quote! {
                        #ident: #((((mem[#src_bytes] & #masks) >> #right_shifts) as #ty) << #left_shifts)|*
                    }
                } else if *ty == parse_quote!(bool) {
                    //#ident: #((mem[#src_bytes] >> #right_shifts) != 0)|*
                    quote!{
                        #ident: #((((mem[#src_bytes] & #masks) >> #right_shifts) != 0))|*
                    }
                } else {
                    Error::new(ty.span(), format!("Unsupported type {:?}", ty)).to_compile_error()
            });
        }

        quote! {
            #(#var_decoders),*
        }
    }

    fn build_encoder(&self, variables: &HashMap<char, (Ident, Type)>) -> TokenStream2 {
        //! This function takes the variable map from the corresponding `Instruction` and
        //! constructs a decoder from this opcode to each variable and joins them to a variable
        //! decoder block that is used in the encoder function.
        //! ```ignore
        //! //This is just a mock up of the encode function
        //! fn encode(&self, mem:&[u8] -> Result<u8, Imperative_rs::EncodeError> {
        //!     match *self {
        //!
        //!         Self::Instr1{ var_bool} => {
        //!             mem[0] = if var_bool { 1 } else {0}; //This line is constructed by this
        //!                                                  //function. More variables means more
        //!                                                  //lines. Integers are a bit trickier
        //!                                                  //than this bool.
        //!         }
        //!
        //!     }
        //! }
        //!
        //! ```
        let code_bytes: Vec<LitInt> = self
            .code_strings()
            .map(|s| LitInt::new(&format!("0b{}", s), self.span()))
            .collect();
        let num_bytes = self.bytes.len();
        let code_indices = 0..num_bytes;
        let mut tokens = quote! {
                if buf.len() < #num_bytes {
                    return Err(imperative_rs::EncodeError::UnexpectedEOF);
                }
                #(buf[#code_indices] = #code_bytes);*;
        };

        for (c, (ident, ty)) in variables.iter() {
            let mut positions_iter = self.get_position_map_of(*c).peekable();
            while let Some((src_bit, (tar_byte, tar_bit))) = positions_iter.next() {
                let lshift = 7 - tar_bit;
                let rshift = src_bit;
                let mut mask = 1;

                let mut num_bits = 1; //number of bits decoded by this mask
                loop {
                    let next_is_neighbour =
                        positions_iter.peek().map_or(false, |(_, (byte, bit))| {
                            *byte == tar_byte && tar_bit == *bit + num_bits
                        });
                    if next_is_neighbour {
                        let _ = positions_iter.next().unwrap();
                        mask = (mask << 1) + 1;
                        num_bits += 1;
                    } else {
                        break;
                    }
                }

                tokens.extend(
                    if *ty == parse_quote!(u8) ||
                        *ty == parse_quote!(u16) ||
                        *ty == parse_quote!(u32) ||
                        *ty == parse_quote!(u64) ||
                        *ty == parse_quote!(u128) ||
                        *ty == parse_quote!(usize) ||
                        *ty == parse_quote!(i8) ||
                        *ty == parse_quote!(i16) ||
                        *ty == parse_quote!(i32) ||
                        *ty == parse_quote!(i64) ||
                        *ty == parse_quote!(i128) ||
                        *ty == parse_quote!(isize) {
                        quote! {
                            buf[#tar_byte] |= (((#ident >> #rshift) & #mask as #ty) << #lshift) as ::std::primitive::u8;
                        }
                    } else if *ty == parse_quote!(bool) {
                        quote!{
                            buf[#tar_byte] |= ((if *#ident {1} else {0}) << #lshift) as ::std::primitive::u8;
                        }
                    } else {
                        Error::new(ty.span(), format!("Unsupported type {:?}", ty)).to_compile_error()
                });
            }
        }
        tokens.extend(quote! {return Ok(#num_bytes);});
        tokens
    }

    pub(crate) fn build_match_conditions(&self) -> TokenStream2 {
        //! Puts together mask and code strings to produce an expression that evaluates to `true`
        //! when and only when the memory contains this opcode
        let num_bytes = self.bytes.len();
        let mut tokens = quote! { mem.len() >= #num_bytes };
        for (idx, (code_str, mask_str)) in self.code_strings().zip(self.mask_strings()).enumerate()
        {
            let mask = LitInt::new(&format!("0b{}", mask_str), self.span);
            let code = LitInt::new(&format!("0b{}", code_str), self.span);
            tokens.extend(quote! {
                && mem[#idx] & #mask == #code
            });
        }
        tokens
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }
}

impl Parse for Opcode {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let _: Token![=] = input.parse()?;
        let literal: LitStr = input.parse()?;
        let mut literal_string: String = literal.value();
        literal_string.retain(|c| c != '_');
        let prefix: Vec<char> = literal_string.chars().take(2).collect();
        if prefix.len() != 2 || prefix[0] != '0' || (prefix[1] != 'x' && prefix[1] != 'b') {
            Err(Error::new(literal.span(), "Invalid opcode. Valid opcodes start with either '0x' or '0b' followed by at least one digit/variable"))
        } else {
            let code: Vec<char> = if prefix[1] == 'x' {
                hex_to_bin_string(&literal_string).chars().collect()
            } else {
                literal_string.chars().skip(2).collect()
            };
            let mut bytes = vec![];
            for (pos, c) in code.iter().enumerate() {
                let bit_idx = pos % 8;
                let byte_idx = (pos - bit_idx) / 8;
                if bytes.len() <= byte_idx {
                    bytes.push(['0'; 8]);
                }
                bytes[byte_idx][bit_idx] = *c;
            }
            Ok(Opcode {
                bytes,
                span: literal.span(),
            })
        }
    }
}

/// This type implements collision detection between opcodes. With the new matcher this type is
/// almost obsolete and might be refactored soon
pub(crate) struct CollisionGuard<'a>(Vec<&'a Opcode>);

impl<'a> CollisionGuard<'a> {
    pub(crate) fn new() -> Self {
        CollisionGuard(vec![])
    }

    pub(crate) fn is_collision(op1: &Opcode, op2: &Opcode) -> bool {
        for (c1, c2) in op1.collision_iter().zip(op2.collision_iter()) {
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

    pub(crate) fn collides_or_insert(&mut self, opcode: &'a Opcode) -> Option<&'a Opcode> {
        for ex_opcode in &self.0 {
            if CollisionGuard::is_collision(opcode, ex_opcode) {
                return Some(ex_opcode);
            }
        }
        self.0.push(opcode);
        None
    }
}
