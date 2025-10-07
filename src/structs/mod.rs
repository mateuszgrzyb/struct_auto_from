use crate::quote;
use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::collections::{HashMap, HashSet};
use syn::{parse_macro_input, Expr, Field, Ident, ItemStruct, Meta, MetaList};

pub fn auto_from_struct(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let from = parse_macro_input!(attrs as Ident);

    let into = parse_macro_input!(input as ItemStruct);
    let ImplData {
        raw_into,
        into,
        fields,
        default_fields,
        default_values,
        option_fields,
        vec_fields,
    } = ImplData::from_parsed_input(into);

    let tokens = quote! {
        #raw_into

        impl From<#from> for #into {
            fn from(value: #from) -> Self {
                Self {
                    #(
                        #fields: value.#fields.into()
                    ),*
                    ,
                    #( #vec_fields: value.#vec_fields.into_iter().map(|x| x.into()).collect()
                    ),*
                     ,
                    #(
                        #option_fields: value.#option_fields.map(|x| x.into())
                    ),*

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
    vec_fields: Vec<Ident>,
    option_fields: Vec<Ident>,
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

        let vec_fields = Self::extract_type_fields_from_input(&mut raw_into, "Vec");
        let option_fields = Self::extract_type_fields_from_input(&mut raw_into, "Option");
        let fields = input
            .fields
            .into_iter()
            .filter_map(|f| f.ident)
            .filter(|i| !default_fields.contains(i))
            .filter(|i| !vec_fields.contains(i))
            .filter(|i| !option_fields.contains(i))
            .collect();

        Self {
            raw_into,
            into,
            fields,
            default_fields,
            option_fields,
            vec_fields,
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

    fn extract_type_fields_from_input(input: &mut ItemStruct, type_string: &str) -> Vec<Ident> {
        let mut fields = HashSet::new();
        for field in input.fields.iter_mut() {
            if let Some(ident) = &field.ident {
                if matches!(
                    &field.ty,
                    syn::Type::Path(type_path) if type_path.path.segments.first().is_some_and(|segment| segment.ident == type_string)
                ) {
                    fields.insert(ident.clone());
                };
            }
        }
        fields.into_iter().collect::<Vec<Ident>>()
    }

    fn remove_attrs(field: &mut Field) {
        field.attrs.retain(|a| {
            let Meta::List(MetaList { path, .. }) = &a.meta else {
                return false;
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
