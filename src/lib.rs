extern crate proc_macro;

use proc_macro::TokenStream;

mod from_hashmap;
mod derived_stuff;


#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap_derive(input: TokenStream) -> TokenStream {
    from_hashmap::derive_from_hashmap_impl(input).into()
}

//  `DerivedStuff` (for debugging)
// #[proc_macro_derive(DerivedStuff)]
// pub fn derived_stuff(input: TokenStream) -> TokenStream {
//     derived_stuff::derive_stuff_impl(input)
// }
