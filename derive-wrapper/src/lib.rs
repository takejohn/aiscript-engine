use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// 列挙型`Enum`の無名のフィールドを1つもつ列挙子`Enum::variant(T)`のそれぞれについて、`Enum`に`From<T>`を実装する
#[proc_macro_derive(Wrapper)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data) = input.data else {
        panic!("expected enum");
    };

    let ident = input.ident;

    let from_implementations = data.variants.iter().map(|variant| {
        let Fields::Unnamed(fields) = &variant.fields else {
            return None;
        };
        if fields.unnamed.len() != 1 {
            return None;
        }
        let field = fields.unnamed.first().unwrap();
        let variant_ident = &variant.ident;
        let ty = &field.ty;
        return Some(quote! {
            impl ::std::convert::From<#ty> for #ident {
                fn from(value: #ty) -> Self {
                    Self::#variant_ident(value)
                }
            }
        });
    });

    let expand = quote! {
        #(#from_implementations)*
    };

    return TokenStream::from(expand);
}
