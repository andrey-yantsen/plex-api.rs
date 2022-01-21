use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error, ItemFn};

#[proc_macro_attribute]
pub fn async_offline_test(args: TokenStream, input: TokenStream) -> TokenStream {
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
        #[cfg_attr(feature = "tests_only_online", ignore = "Feature tests_only_online is set, running only online tests.")]
        #[::rstest::rstest]
        #(#fn_attrs)*
        #fn_vis #fn_signature {
            #fn_block
        }
    })
}
