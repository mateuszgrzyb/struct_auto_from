use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

/// Auto implement `From` trait.
///
/// Both structs are expected to have the same fields.
///
/// <br>
///
/// # Example
/// ```
/// use auto_from::auto_from;
///
/// struct Model1 {
///     id: i32,
///     name: String,
///     attrs: Vec<String>,
/// }
///
/// #[auto_from(Model1)]
/// struct Model2 {
///     id: i32,
///     name: String,
///     attrs: Vec<String>,
/// }
///
/// ...
///
/// let m1: Model1 = /* ... */;
/// let m2: Model2 = m1.into();
/// ```
#[proc_macro_attribute]
pub fn auto_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let from = parse_macro_input!(attrs as Ident);
    let into = parse_macro_input!(input as ItemStruct);
    let raw_into = into.clone();

    let into_name = into.ident;
    let attrs = into.fields.into_iter().filter_map(|f| f.ident);

    let tokens = quote! {
        #raw_into

        impl From<#from> for #into_name {
            fn from(value: #from) -> Self {
                Self {
                    #(
                        #attrs: value.#attrs
                    ),*
                }
            }
        }
    };

    tokens.into()
}
