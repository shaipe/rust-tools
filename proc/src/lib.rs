extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_str, AttributeArgs, Block, FnArg, Ident, ItemFn, Pat, PathArguments,
    ReturnType, Type,
};

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the passed item as a function
    let func = syn::parse_macro_input!(item as syn::ItemFn);

    // Break the function down into its parts
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = func;

    // println!("{:?}", block);
    // // Ensure that it isn't an `async fn`
    // if let Some(async_token) = sig.asyncness {
    //     // Error out if so
    //     let error = syn::Error::new(
    //         async_token.spqn(),
    //         "async functions do not support caller tracking functionality
    // help: consider returning `impl Future` instead",
    //     );

    //     return TokenStream::from(error.to_compile_error());
    // }

    // Wrap body in a closure only if function doesn't already have #[track_caller]
    let block = if attrs.iter().any(|attr| attr.path.is_ident("track_caller")) {
        quote! { #block }
    } else {
        quote! {
            (move || #block)()
        }
    };

    // Extract function name for prettier output
    let name = format!("{}", sig.ident);

    // Generate the output, adding `#[track_caller]` as well as a `println!`
    let output = quote! {
        #[track_caller]
        #(#attrs)*
        #vis #sig {
            println!(
                "entering `fn {}`: called from `{}`",
                #name,
                ::core::panic::Location::caller()
            );
            #block
        }
    };

    // Convert the output from a `proc_macro2::TokenStream` to a `proc_macro::TokenStream`
    TokenStream::from(output)
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
