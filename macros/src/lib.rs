extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn annotated_benchmark(
    attrib_args: TokenStream,
    bench_template_args: TokenStream,
) -> TokenStream {
    let function = if let Item::Fn(funct) = parse_macro_input!(bench_template_args as Item) {
        funct
    } else {
        panic!("Expected to be applied to a function!")
    };
    TokenStream::from(quote! {#function})
}

fn annotate_function(fn_tokens: syn::ItemFn) -> syn::ItemFn {
    todo!()
    //let x = fn_tokens.clone_into();
}
