extern crate proc_macro;

use proc_macro::TokenStream;

mod cache_it;
mod from_hashmap;

#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap_derive(input: TokenStream) -> TokenStream {
    from_hashmap::derive_from_hashmap_impl(input)
}

#[proc_macro_attribute]
pub fn cache_it(attr: TokenStream, item: TokenStream) -> TokenStream {
    cache_it::cache(attr, item)
}
