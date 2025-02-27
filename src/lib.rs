extern crate proc_macro;

use proc_macro::TokenStream;

mod from_hashmap;

#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap_derive(input: TokenStream) -> TokenStream {
    from_hashmap::derive_custom_model_impl(input).into()
}
