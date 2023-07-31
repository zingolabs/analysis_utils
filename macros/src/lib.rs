extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
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
    let function = annotate_function(function);
    TokenStream::from(quote! {#function})
}

fn annotate_function(fn_tokens: syn::ItemFn) -> proc_macro2::TokenStream {
    let other_attributes = quote!(fn_tokens.attrs);
    let nym = quote!(fn_tokens.sig.ident);
    let tokenized_statements = sandwich_statements(fn_tokens.block.stmts);
    dbg!(other_attributes);
    dbg!(nym);
    tokenized_statements
}
fn prefix_statements() -> proc_macro2::TokenStream {
    todo!()
}
fn suffix_statements() -> proc_macro2::TokenStream {
    todo!()
}
fn sandwich_statements(bench_statements: Vec<syn::Stmt>) -> proc_macro2::TokenStream {
    quote!({
        #(#bench_statements)*
    })
}

#[test]
fn show_macro_expansion() {
    println!(
        "{}",
        annotate_function(
            syn::parse2(quote!(
                #[tokio::test]
                async fn keyless_client_pu_false() {
                    timing_run("keyless", false).await;
                }
            ))
            .expect("To succeed.")
        )
        .to_string()
    );
}
