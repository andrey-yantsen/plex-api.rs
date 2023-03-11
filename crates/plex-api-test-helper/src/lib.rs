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

    TokenStream::from(quote! {
        #[::rstest::rstest]
        #(#fn_attrs)*
        #extra_attr
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
            ignore = "Feature tests_only_online is set, running only online tests.",
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
            ignore = "Feature tests_only_online_anonymous is not set, skipping anonymous online tests.",
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
            ignore = "Feature tests_only_online_authenticated is not set, skipping authenticated online tests.",
        )]
    };
    rewrite_single_test(args, input, ignore)
}

#[proc_macro_attribute]
/// Test requires myplex account
pub fn online_test_myplex(args: TokenStream, input: TokenStream) -> TokenStream {
    let ignore = quote! {
        #[cfg_attr(
            feature = "tests_shared_server_access_token",
            ignore = "Feature tests_shared_server_access_token is set, skipping tests requiring myplex access.",
        )]
        #[cfg_attr(
            not(feature = "tests_only_online_claimed_server"),
            ignore = "Feature tests_only_online_authenticated is not set, skipping authenticated online tests.",
        )]
    };
    rewrite_single_test(args, input, ignore)
}

fn rewrite_multirun_test(
    args: TokenStream,
    input: TokenStream,
    extra_attr_claimed: TokenStream2,
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

    let mut fn_signature1 = fn_type.sig;
    let mut fn_signature2 = fn_signature1.clone();

    fn_signature1.ident = syn::Ident::new(
        &format!("{}_claimed_server", fn_signature1.ident),
        fn_signature1.ident.span(),
    );

    fn_signature2.ident = syn::Ident::new(
        &format!("{}_unclaimed_server", fn_signature2.ident),
        fn_signature2.ident.span(),
    );

    let fn_vis = fn_type.vis;
    let fn_attrs = fn_type.attrs;
    let fn_block = fn_type.block;

    TokenStream::from(quote! {
        #[::rstest::rstest(server_claimed as server)]
        #(#fn_attrs)*
        #[cfg_attr(
            not(feature = "tests_only_online_claimed_server"),
            ignore = "Feature tests_only_online_authenticated is not set, skipping authenticated online tests.",
        )]
        #extra_attr_claimed
        #fn_vis #fn_signature1 {
            #fn_block
        }

        #[::rstest::rstest(server_unclaimed as server)]
        #(#fn_attrs)*
        #[cfg_attr(
            not(feature = "tests_only_online_unclaimed_server"),
            ignore = "Feature tests_only_online_anonymous is not set, skipping anonymous online tests.",
        )]
        #fn_vis #fn_signature2 {
            #fn_block
        }
    })
}

#[proc_macro_attribute]
/// Test requires either owned or unclaimed server
pub fn online_test_non_shared_server(args: TokenStream, input: TokenStream) -> TokenStream {
    let ignore = quote! {
        #[cfg_attr(
            feature = "tests_shared_server_access_token",
            ignore = "Feature tests_shared_server_access_token is set, skipping tests requiring myplex access.",
        )]
    };
    rewrite_multirun_test(args, input, ignore)
}

#[proc_macro_attribute]
/// Test can work with both claimed and unclaimed servers.
/// NB! The provided authentication token might not have MyPlex access.
pub fn online_test(args: TokenStream, input: TokenStream) -> TokenStream {
    rewrite_multirun_test(args, input, quote! {})
}
