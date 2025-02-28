use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type, TypePath};

pub(crate) fn derive_from_hashmap_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
        } else {
            panic!("FromHashMap can only be derived on structs with named fields.");
        }
    } else {
        panic!("FromHashMap can only be derived on structs.");
    };

    let field_mappings = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        let field_str = field_name.as_ref().unwrap().to_string();
        let ty = &f.ty;

        // Handle nested struct types
        if let Type::Path(TypePath { path, .. }) = ty {
            let type_name = quote!(#path).to_string();


            //TODO
            if !["String", "u8", "u16", "i32", "f64", "bool", "Option"]
                .iter()
                .any(|&t| type_name.contains(t))
            {
                return quote! {
                    #field_name: fields.iter()
                        .filter(|(k, _)| k.starts_with(&(#field_str.to_string() + ".")))
                        .map(|(k, v)| (k.trim_start_matches(&(#field_str.to_string() + ".")).to_string(), v.clone()))
                        .collect::<std::collections::HashMap<String, String>>()
                        .into(),
                };
            }
        }

        // Handle specific primitive types
        if quote!(#ty).to_string().contains("Option") {
            quote! {
                #field_name: fields.get(#field_str).cloned(),
            }
        } else if quote!(#ty).to_string().contains("u8") {
            quote! {
                #field_name: fields.get(#field_str)
                    .and_then(|val| val.parse::<u8>().ok())
                    .unwrap_or_default(),
            }
        } else if quote!(#ty).to_string().contains("i32") {
            quote! {
                #field_name: fields.get(#field_str)
                    .and_then(|val| val.parse::<i32>().ok())
                    .unwrap_or_default(),
            }
        } else if quote!(#ty).to_string().contains("f64") {
            quote! {
                #field_name: fields.get(#field_str)
                    .and_then(|val| val.parse::<f64>().ok())
                    .unwrap_or_default(),
            }
        } else {
            quote! {
                #field_name: fields.get(#field_str).cloned().unwrap_or_default(),
            }
        }
    });

    let expanded = quote! {
        impl From<std::collections::HashMap<String, String>> for #struct_name {
            fn from(fields: std::collections::HashMap<String, String>) -> Self {
                Self {
                    #(#field_mappings)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
