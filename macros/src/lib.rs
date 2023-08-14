extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn annotated_benchmark(attrib_args: TokenStream, bench_template: TokenStream) -> TokenStream {
    let function = if let Item::Fn(funct) = parse_macro_input!(bench_template as Item) {
        funct
    } else {
        panic!("Expected to be applied to a function!")
    };
    let scenario = parse_macro_input!(attrib_args as syn::Ident);
    let processed_benchmark = generate_benchmark(function, scenario);
    TokenStream::from(quote! {#processed_benchmark})
}
fn generate_benchmark(fn_tokens: syn::ItemFn, scenario: syn::Ident) -> proc_macro2::TokenStream {
    // Process input tokens after handling attribute_args
    let ident = fn_tokens.sig.ident.to_string();
    let attrs = &mut fn_tokens
        .attrs
        .iter()
        .map(|x| x.to_token_stream())
        .collect::<proc_macro2::TokenStream>();

    attrs.extend(quote!(#[allow(unused_variables)]));
    attrs.extend(quote!(#[tokio::test]));
    let signature = &fn_tokens.sig;
    let start_statements_stop_annotate =
        sandwich_statements(scenario, ident, fn_tokens.block.stmts.clone());
    quote!(#attrs #signature #start_statements_stop_annotate)
}
fn setup_and_start_timer(scenario: &syn::Ident) -> proc_macro2::TokenStream {
    quote!(
        let (regtest_manager, child_process_handler, keyowning, keyless) =
            scenarios::chainload::#scenario().await;
        let timer_start = Instant::now();
    )
}
fn stop_and_record_time() -> proc_macro2::TokenStream {
    quote!(
        let timer_stop = Instant::now();
        let sync_duration = timer_stop.duration_since(timer_start);
    )
}
fn specify_annotations(scenario: syn::Ident, nym: String) -> proc_macro2::TokenStream {
    quote!(
        let annotation = zingo_testutils::DurationAnnotation::new(#scenario.to_string(), #nym.to_string(), sync_duration);
        zingo_testutils::record_time(&annotation);
    )
}
fn sandwich_statements(
    scenario: syn::Ident,
    test_name: String,
    bench_statements: Vec<syn::Stmt>,
) -> proc_macro2::TokenStream {
    let setup_start_time = setup_and_start_timer(&scenario);
    let stop_rec_time = stop_and_record_time();
    let annotate_statements = specify_annotations(scenario, test_name);
    quote!(
        {
            let mut count = 0;
            loop {
                #setup_start_time
                #(#bench_statements)*
                #stop_rec_time
                #annotate_statements
                count+=1;
                if count == 2 {break;}
            }
        }
    )
}

#[test]
fn show_annotate_function_expansion() {
    println!(
        "{}",
        generate_benchmark(
            syn::parse2(quote!(
                async fn keyless_client_pu_false() {
                    keyless.do_sync(true).await.unwrap();
                }
            ))
            .expect("To succeed."),
            syn::parse2(quote!(unsynced_faucet_recipient_1153)).expect("to parse to Ident")
        )
        .to_string()
    );
}
