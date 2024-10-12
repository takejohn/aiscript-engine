use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, Type};

/// 構造体の`Loc`型のフィールドを用いて`ast::NodeBase`を実装する  
/// 構造体をラップする列挙体にも使用可能
#[proc_macro_derive(NodeBase)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let expand = match input.data {
        Data::Struct(data) => derive_for_struct(ident, data),
        Data::Enum(data) => derive_for_enum(ident, data),
        _ => panic!("expected struct or enum"),
    };

    return proc_macro::TokenStream::from(expand);
}

fn derive_for_struct(ident: Ident, data: DataStruct) -> proc_macro2::TokenStream {
    let Fields::Named(fields) = data.fields else {
        panic!("expected named fields");
    };

    let field_ident = &fields
        .named
        .iter()
        .find(|&field| {
            let Type::Path(path) = &field.ty else {
                return false;
            };
            let Some(seg) = path.path.segments.last() else {
                return false;
            };
            return seg.ident == "Loc";
        })
        .expect("No field with type Loc found")
        .ident;

    return quote! {
        impl crate::ast::NodeBase for #ident {
            fn loc(&self) -> &crate::ast::Loc {
                &self.#field_ident
            }
        }
    };
}

fn derive_for_enum(ident: Ident, data: DataEnum) -> proc_macro2::TokenStream {
    let arms = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let Fields::Unnamed(fields) = &variant.fields else {
            panic!("expected unnamed fields for variant '{}'", variant_ident)
        };
        if fields.unnamed.len() != 1 {
            panic!("expected single field for variant '{}'", variant_ident);
        }
        return quote! {
            Self::#variant_ident(v) => crate::ast::NodeBase::loc(v)
        };
    });

    return quote! {
        impl crate::ast::NodeBase for #ident {
            fn loc(&self) -> &crate::ast::Loc {
                match self {
                    #(#arms),*
                }
            }
        }
    };
}
