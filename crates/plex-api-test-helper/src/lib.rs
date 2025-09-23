use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error, ItemFn};

fn rewrite_single_test(
    args: TokenStream,
    input: TokenStream,
    extra_attr: TokenStream2,
) -> TokenStream {
    if !args.is_empty() {
        return Error::new(
            TokenStream2::from(args).span(),
            "Attribute does not accept any arguments",
        )
        .to_compile_error()
        .into();
    }

    let fn_type = parse_macro_input!(input as ItemFn);

    let fn_signature = fn_type.sig;
    let fn_vis = fn_type.vis;
    let fn_attrs = fn_type.attrs;
    let fn_block = fn_type.block;

    let async_attr = if fn_signature.asyncness.is_some() {
        quote! { #[async_std::test] }
    } else {
        quote! {}
    };

    TokenStream::from(quote! {
        #[::rstest::rstest]
        #(#fn_attrs)*
        #extra_attr
        #async_attr
        #[awt]
        #fn_vis #fn_signature {
            #fn_block
        }
    })
}

#[proc_macro_attribute]
/// Test works over mocked data and does not require online access
pub fn offline_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let ignore = quote! {
        #[cfg_attr(
            feature = "tests_only_online",
            ignore = "Skipping offline tests.",
        )]
    };
    rewrite_single_test(args, input, ignore)
}

#[proc_macro_attribute]
/// Test requires an unclaimed server
pub fn online_test_unclaimed_server(args: TokenStream, input: TokenStream) -> TokenStream {
    let ignore = quote! {
        #[cfg_attr(
            not(feature = "tests_only_online_unclaimed_server"),
            ignore = "Skipping tests requiring an unclaimed server.",
        )]
    };
    rewrite_single_test(args, input, ignore)
}

#[proc_macro_attribute]
/// Test requires a claimed server.
/// NB! The provided authentication token might not have MyPlex access.
pub fn online_test_claimed_server(args: TokenStream, input: TokenStream) -> TokenStream {
    let ignore = quote! {
        #[cfg_attr(
            not(feature = "tests_only_online_claimed_server"),
            ignore = "Skipping tests requiring a claimed server.",
        )]
    };
    rewrite_single_test(args, input, ignore)
}

#[proc_macro_attribute]
/// Test requires myplex account
pub fn online_test_myplex(args: TokenStream, input: TokenStream) -> TokenStream {
    let ignore = quote! {
        #[cfg_attr(
            not(feature = "tests_only_online_claimed_server"),
            ignore = "Skipping tests requiring a claimed server.",
        )]
        #[cfg_attr(
            feature = "tests_shared_server_access_token",
            ignore = "Skipping tests requiring myplex access.",
        )]
    };
    rewrite_single_test(args, input, ignore)
}

#[proc_macro_attribute]
/// Test requires either owned or unclaimed server
pub fn online_test_non_shared_server(args: TokenStream, input: TokenStream) -> TokenStream {
    let extra_attrs = quote! {
        #[cfg_attr(
            not(any(feature = "tests_only_online_claimed_server", feature = "tests_only_online_unclaimed_server")),
            ignore = "Skipping online tests.",
        )]
        #[cfg_attr(
            feature = "tests_shared_server_access_token",
            ignore = "Skipping tests requiring myplex access.",
        )]
    };

    rewrite_single_test(args, input, extra_attrs)
}

#[proc_macro_attribute]
/// Test can work with both claimed and unclaimed servers.
/// NB! The provided authentication token might not have MyPlex access.
pub fn online_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let extra_attrs = quote! {
        #[cfg_attr(
            not(any(feature = "tests_only_online_claimed_server", feature = "tests_only_online_unclaimed_server")),
            ignore = "Skipping online tests.",
        )]
    };

    rewrite_single_test(args, input, extra_attrs)
}
