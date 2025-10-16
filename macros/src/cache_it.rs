use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, ItemFn, Result, Token,
};

/// Structure representing parsed attribute arguments.
///
/// Example forms supported:
/// ```ignore
/// #[cache(key = format!("user:{}", id))]
/// #[cache(var = redis, key = format!("user:{}", id), name = cached)]
/// ```
struct CacheArgs {
    /// The cache variable name (defaults to `cache`)
    var: Option<Expr>,
    /// The expression used as the cache key
    key: Expr,
    /// The name for the local binding of cached data (defaults to `cache_data`)
    name: Option<Expr>,
}

impl Parse for CacheArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut var = None;
        let mut key = None;
        let mut name = None;

        while !input.is_empty() {
            // Parse `ident = <expr>`
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let expr: Expr = input.parse()?;

            if ident == "var" {
                var = Some(expr);
            } else if ident == "key" {
                key = Some(expr);
            } else if ident == "name" {
                name = Some(expr);
            } else {
                // use clone to create new Ident for error span
                return Err(syn::Error::new_spanned(
                    ident.clone(),
                    format!("unexpected argument `{}`", ident),
                ));
            }

            // optional trailing comma
            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>();
            }
        }

        Ok(CacheArgs {
            var,
            key: key.ok_or_else(|| input.error("missing `key = ...` argument"))?,
            name,
        })
    }
}

/// The main `#[cache(...)]` procedural macro entry point.
///
/// Example:
/// ```ignore
/// #[cache(key = format!("user:{}", id))]
/// fn get_user(cache: &impl Cache<User>) -> Option<User> { ... }
/// ```
pub(crate) fn cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as CacheArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    match expand_cache_it(args, input_fn) {
        Ok(expanded) => expanded.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Expands the cache macro into the final function implementation.
fn expand_cache_it(args: CacheArgs, input_fn: ItemFn) -> Result<TokenStream2> {
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let cache_var = args.var.map_or_else(|| quote!(cache), |v| quote!(#v));
    let binding_name = args.name.map_or_else(|| quote!(cache_data), |n| quote!(#n));
    let key_expr = args.key;

    // let maybe_ret_ty = match &input_fn.sig.output {
    //     syn::ReturnType::Type(_, ty) => Some(ty),
    //     _ => None,
    // };

    // Detect async function to insert `.await` automatically
    let injected = if sig.asyncness.is_some() {
        quote! {
            let #binding_name = #cache_var.get::<_>(&#key_expr).await;
        }
    } else {
        quote! {
            let #binding_name = #cache_var.get::<_>(&#key_expr);
        }
    };

    Ok(quote! {
        #vis #sig {
            #injected
            #block
        }
    })
}
