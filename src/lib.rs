use crate::{enums::auto_from_enum, structs::auto_from_struct};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Error};

mod enums;
mod structs;
/// Auto implement `From` trait.
///
/// When specifying conversion, all fields in the receiving struct type
/// must either be defined in the sender, or have their default values
/// defined on the receiver.
///
/// Default value attribute lets you override data from sender.
///
/// <br>
///
/// # Examples
/// ```
/// use struct_auto_from::auto_from;
///
/// // Enums
///
/// enum Foo {
///     Variant1,
///     Variant2,
/// }
///
/// #[auto_from(Foo)]
/// enum Bar {
///     Variant1,
///     Variant2,
/// }
///
/// // Structs
///
/// #[derive(Clone)]
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
///
/// let m1: Model1 = Model1 {
///     id: 0,
///     name: "M".into(),
///     attrs: vec![],
/// };
/// let m2: Model2 = m1.clone().into();
///
/// assert_eq!(m1.id, m2.id);
/// assert_eq!(m1.name, m2.name);
/// assert_eq!(m1.attrs, m2.attrs);
/// ```
///
/// Using the default values
///
/// ```
/// use std::collections::HashMap;
/// use struct_auto_from::auto_from;
///
/// #[derive(Clone)]
/// struct Model1 {
///     id: i32,
///     name: String,
///     attrs: Vec<String>,
/// }
///
/// #[auto_from(Model1)]
/// struct Model2 {
///     #[auto_from_attr(default_value = -1)]
///     id: i32,
///     name: String,
///     attrs: Vec<String>,
///     #[auto_from_attr(default_value = Default::default())]
///     metadata: HashMap<String, String>,
/// }
///
///
/// let m1: Model1 = Model1 {
///     id: 0,
///     name: "M".into(),
///     attrs: vec![],
/// };
/// let m2: Model2 = m1.clone().into();
///
/// assert_eq!(-1, m2.id);
/// assert_eq!(m1.name, m2.name);
/// assert_eq!(m1.attrs, m2.attrs);
/// assert!(m2.metadata.is_empty());
/// ```
#[proc_macro_attribute]
pub fn auto_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let bla = input.clone();
    let parsed = parse_macro_input!(bla as DeriveInput);

    match parsed.data {
        syn::Data::Enum(_) => auto_from_enum(attrs, input),
        syn::Data::Struct(_) => auto_from_struct(attrs, input),
        _ => Error::new(
            parsed.span(),
            "auto_from can only be applied to structs or enums",
        )
        .to_compile_error()
        .into(),
    }
}
