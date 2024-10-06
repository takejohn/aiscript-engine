use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

/// 構造体の`Loc`型のフィールドを用いて`ast::Node`を実装する
#[proc_macro_derive(Node)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let Data::Struct(data) = input.data else {
        panic!("expected struct");
    };

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

    let expand = quote! {
        impl crate::ast::Node for #ident {
            fn loc(&self) -> &crate::ast::Loc {
                &self.#field_ident
            }
        }
    };

    return proc_macro::TokenStream::from(expand);
}
