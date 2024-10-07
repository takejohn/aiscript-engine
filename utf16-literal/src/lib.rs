use std::u16;

use proc_macro::TokenStream;
use quote::TokenStreamExt;
use syn::{parse_macro_input, Expr, Lit, LitChar, LitStr};

/// 文字列リテラルが与えられた場合、UTF-16文字列の配列(`[u16]`型)を返す。  
/// 文字リテラルが与えられた場合、その文字コード(`u16`型)を返す。
#[proc_macro]
pub fn utf16(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);

    let Expr::Lit(lit) = input else {
        panic!("expected literal");
    };

    let expand = match lit.lit {
        Lit::Str(s) => str_to_utf16(s),
        Lit::Char(ch) => char_to_utf16(ch),
        _ => panic!("expected str or char"),
    };

    return TokenStream::from(expand);
}

fn str_to_utf16(s: LitStr) -> proc_macro2::TokenStream {
    let values: Vec<u16> = s.value().encode_utf16().collect();

    let mut elms = proc_macro2::TokenStream::new();
    elms.append_separated(
        values
            .iter()
            .map(|&value| proc_macro2::Literal::u16_suffixed(value)),
        proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
    );
    let arr = proc_macro2::Group::new(proc_macro2::Delimiter::Bracket, elms);

    let mut stream = proc_macro2::TokenStream::new();
    stream.append(arr);
    return stream;
}

fn char_to_utf16(ch: LitChar) -> proc_macro2::TokenStream {
    let value = ch.value();
    if value as u32 > u16::MAX as u32 {
        panic!("character out of range");
    }
    let value = value as u16;

    let mut stream = proc_macro2::TokenStream::new();
    stream.append(proc_macro2::Literal::u16_suffixed(value));
    return stream;
}
