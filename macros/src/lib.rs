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
    let other_attributes = quote!(#fn_tokens.attrs);
    let ident = &fn_tokens.sig.ident;
    let nym = quote!(#ident);
    let start_statements_stop = sandwich_statements(fn_tokens.block.stmts);
    let annotation = specify_annotations(nym);
    quote!(#start_statements_stop #annotation)
}
fn setup_and_start_timer() -> proc_macro2::TokenStream {
    quote!(
        let (_, child_process_handler, keyowning, _keyless) =
            scenarios::chainload::unsynced_faucet_recipient_1153().await;
        let timer_start = Instant::now();
    )
}
fn stop_and_record_time() -> proc_macro2::TokenStream {
    quote!(
        let timer_stop = Instant::now();
        sync_duration = timer_stop.duration_since(timer_start);
    )
}
fn specify_annotations(nym: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote!(
        let annotation = zingo_testutils::DurationAnnotation::new(
            #nym.to_string(),
            sync_duration,
        );
        zingo_testutils::record_time(&annotation);
    )
}
fn sandwich_statements(bench_statements: Vec<syn::Stmt>) -> proc_macro2::TokenStream {
    let setup_start_time = setup_and_start_timer();
    let stop_rec_time = stop_and_record_time();
    quote!({
        #setup_start_time
        #(#bench_statements)*
        #stop_rec_time
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
