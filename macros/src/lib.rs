extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn my_macro(_input: TokenStream) -> TokenStream {
    todo!();
}
