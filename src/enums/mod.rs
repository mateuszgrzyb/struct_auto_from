use crate::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, ItemEnum};

struct EnumData {
    raw_into: ItemEnum,
    into: Ident,
    variants: Vec<Ident>,
}

impl EnumData {
    fn from_parsed_input(input: ItemEnum) -> Self {
        let raw_into = input.clone();
        let into = input.ident;
        let variants = input.variants.into_iter().map(|f| f.ident).collect();

        Self {
            raw_into,
            into,
            variants,
        }
    }
}
pub fn auto_from_enum(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let from = parse_macro_input!(attrs as Ident);

    let into = parse_macro_input!(input as ItemEnum);
    let EnumData {
        raw_into,
        into,
        variants,
    } = EnumData::from_parsed_input(into);

    let tokens = quote! {
        #raw_into

        impl From<#from> for #into {
            fn from(value: #from) -> Self {
                match value {
                    #(
                        #from::#variants => Self::#variants
                    ),*
                }
            }
        }
    };

    tokens.into()
}
