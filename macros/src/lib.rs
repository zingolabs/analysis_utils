extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn annotated_benchmark(
    _attrib_args: TokenStream,
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
    let ident = fn_tokens.sig.ident.to_string();
    let attrs = &fn_tokens
        .attrs
        .iter()
        .map(|x| x.to_token_stream())
        .collect::<proc_macro2::TokenStream>();
    let signature = &fn_tokens.sig;
    let _test = quote!(#[tokio::test]);
    let start_statements_stop_annotate = sandwich_statements(ident, fn_tokens.block.stmts.clone());
    quote!(#attrs #signature #start_statements_stop_annotate)
}
fn setup_and_start_timer() -> proc_macro2::TokenStream {
    quote!(
        let (_, _child_process_handler, _keyowning, keyless) =
            scenarios::chainload::unsynced_faucet_recipient_1153().await;
        let timer_start = Instant::now();
    )
}
fn stop_and_record_time() -> proc_macro2::TokenStream {
    quote!(
        let timer_stop = Instant::now();
        let sync_duration = timer_stop.duration_since(timer_start);
    )
}
fn specify_annotations(nym: String) -> proc_macro2::TokenStream {
    quote!(
        let annotation = zingo_testutils::DurationAnnotation::new(#nym.to_string(), sync_duration);
        zingo_testutils::record_time(&annotation);
    )
}
fn sandwich_statements(
    test_name: String,
    bench_statements: Vec<syn::Stmt>,
) -> proc_macro2::TokenStream {
    let setup_start_time = setup_and_start_timer();
    let stop_rec_time = stop_and_record_time();
    let annotate_statements = specify_annotations(test_name);
    quote!(
        {
        #setup_start_time
        #(#bench_statements)*
        #stop_rec_time
        #annotate_statements
        }
    )
}

#[test]
fn show_annotate_function_expansion() {
    println!(
        "{}",
        annotate_function(
            syn::parse2(quote!(
                #[tokio::test]
                async fn keyless_client_pu_false() {
                    keyless.do_sync(true).await.unwrap();
                }
            ))
            .expect("To succeed.")
        )
        .to_string()
    );
}
