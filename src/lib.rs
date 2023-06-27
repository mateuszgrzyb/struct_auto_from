use std::collections::HashMap;

use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Expr, Field, ItemStruct, Meta, MetaList};

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
///
#[proc_macro_attribute]
pub fn auto_from(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let from = parse_macro_input!(attrs as Ident);

    let into = parse_macro_input!(input as ItemStruct);
    let ImplData {
        raw_into,
        into,
        fields,
        default_fields,
        default_values,
    } = ImplData::from_parsed_input(into);

    let tokens = quote! {
        #raw_into

        impl From<#from> for #into {
            fn from(value: #from) -> Self {
                Self {
                    #(
                        #fields: value.#fields
                    ),*
                    ,
                    #(
                        #default_fields: #default_values
                    ),*
                }
            }
        }
    };

    tokens.into()
}

struct ImplData {
    raw_into: ItemStruct,
    into: Ident,
    fields: Vec<Ident>,
    default_fields: Vec<Ident>,
    default_values: Vec<Expr>,
}

impl ImplData {
    fn from_parsed_input(input: ItemStruct) -> Self {
        let mut raw_into = input.clone();
        let into = input.ident;
        let (default_fields, default_values): (Vec<_>, Vec<_>) =
            Self::extract_defaults_from_input(&mut raw_into)
                .into_iter()
                .unzip();
        let fields = input
            .fields
            .into_iter()
            .filter_map(|f| f.ident)
            .filter(|i| !default_fields.contains(i))
            .collect();

        Self {
            raw_into,
            into,
            fields,
            default_fields,
            default_values,
        }
    }

    fn extract_defaults_from_input(input: &mut ItemStruct) -> HashMap<Ident, Expr> {
        let mut defaults = HashMap::new();

        for field in input.fields.iter_mut() {
            let attrs = AutoFromAttr::from_field(field).unwrap();

            if let (Some(ident), Some(default_value)) = (&mut field.ident, attrs.default_value) {
                defaults.insert(ident.clone(), default_value);
                Self::remove_attrs(field);
            }
        }

        defaults
    }

    fn remove_attrs(field: &mut Field) {
        field.attrs.retain(|a| {
            let Meta::List(MetaList { path, .. }) = &a.meta else {
                return false
            };

            !path.is_ident(&Ident::new("auto_from_attr", Span::call_site()))
        })
    }
}

#[derive(FromField, Default, Debug)]
#[darling(default, attributes(auto_from_attr))]
struct AutoFromAttr {
    default_value: Option<Expr>,
}
