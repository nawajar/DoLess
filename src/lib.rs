extern crate proc_macro;

use proc_macro::TokenStream;

mod from_hashmap;

#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap_derive(input: TokenStream) -> TokenStream {
    from_hashmap::derive_from_hashmap_impl(input).into()
}
