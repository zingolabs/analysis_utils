extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn duration_annotation_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function = if let Item::Fn(funct) = parse_macro_input!(input as Item) {
        funct
    } else {
        panic!("Arrgggh!")
    };
    TokenStream::from(quote! {#function})
}
