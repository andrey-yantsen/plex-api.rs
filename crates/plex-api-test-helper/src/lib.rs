use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    Error, FnArg, Ident, ItemFn, Pat, Token,
};

struct OnlineTestArgs {
    authenticated: bool,
    authenticated_by_username: bool,
}

impl Parse for OnlineTestArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ret = Self {
            authenticated: false,
            authenticated_by_username: false,
        };
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        for arg in vars {
            match arg.to_string().as_str() {
                "authenticated" => ret.authenticated = true,
                "authenticated_by_username" => ret.authenticated_by_username = true,
                _ => return Err(Error::new(arg.span(), format!("Unknown argument: {}", arg))),
            }
        }
        Ok(ret)
    }
}

enum UserType {
    Free,
    FreeGuest,
    PlexPass,
    PlexPassGuest,
}

struct OfflineTestArgs {
    user_type: UserType,
}

impl Parse for OfflineTestArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ret = Self {
            user_type: UserType::Free,
        };
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        for arg in vars {
            match arg.to_string().as_str() {
                "guest_user" => ret.user_type = UserType::FreeGuest,
                "plexpass_user" => ret.user_type = UserType::PlexPass,
                "plexpass_guest_user" => ret.user_type = UserType::PlexPassGuest,
                _ => return Err(Error::new(arg.span(), format!("Unknown argument: {}", arg))),
            }
        }
        Ok(ret)
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn online_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as OnlineTestArgs);

    let fn_type = parse_macro_input!(input as ItemFn);
    let fn_signature = fn_type.sig;
    let fn_ident = fn_signature.ident;
    let fn_arguments = fn_signature.inputs;
    let fn_return_type = fn_signature.output;

    let mut myplex_argument: Option<Box<Pat>> = None;
    let mut client_argument: Option<Box<Pat>> = None;
    let mut server_argument: Option<Box<Pat>> = None;

    for arg in fn_arguments {
        if let FnArg::Typed(pat_type) = arg {
            let ty = &pat_type.ty;
            match quote!(#ty).to_string().as_str() {
                "MyPlex" => myplex_argument = Some(pat_type.pat),
                "Client" => client_argument = Some(pat_type.pat),
                "Server" => server_argument = Some(pat_type.pat),
                _ => {}
            }
        }
    }

    if myplex_argument.is_none() && client_argument.is_none() && server_argument.is_none() {
        abort_call_site!(
            "The online test must accept at least one of MyPlex, Client or Server as an argument."
        );
    }

    let plex_client_ident: Ident = match client_argument {
        Some(pat) => Ident::new(&quote!(#pat).to_string(), Span::call_site()),
        None => Ident::new("__plex_client", Span::call_site()),
    };

    let fn_block = fn_type.block;

    let mut function_prefix = quote!();
    let allow_auth_by_token = !args.authenticated_by_username;
    if args.authenticated || args.authenticated_by_username {
        let env_auth_token = ::std::option_env!("PLEX_API_AUTH_TOKEN");
        let env_username = ::std::option_env!("PLEX_API_USERNAME");
        let env_password = ::std::option_env!("PLEX_API_PASSWORD");

        if env_username.is_none()
            || env_password.is_none()
            || env_username.unwrap().is_empty()
            || env_password.unwrap().is_empty()
        {
            if !allow_auth_by_token {
                function_prefix = quote!(
                    #[ignore = "Username and password must be provided for this test."]
                );
            } else if env_auth_token.is_none() || env_auth_token.unwrap().is_empty() {
                function_prefix = quote!(
                    #[ignore = "Auth token or username&password must be provided for this test."]
                );
            }
        }
    }

    let mut fn_block = quote!(
        #fn_block
    );

    if let Some(myplex_argument) = myplex_argument {
        if !args.authenticated && !args.authenticated_by_username {
            abort_call_site!(
                "The online test working with MyPlex must be marked as `authenticated` or `authenticated_by_username`."
            );
        }

        let myplex_ident = Ident::new(&quote!(#myplex_argument).to_string(), Span::call_site());

        fn_block = quote!(
            let mut #myplex_ident = ::plex_api::MyPlexBuilder::default()
                .set_client(#plex_client_ident)
                .build()
                .await
                .expect("failed to create myplex client");
            #fn_block
        );
    }

    if let Some(server_argument) = server_argument {
        let server_ident = Ident::new(&quote!(#server_argument).to_string(), Span::call_site());

        fn_block = quote!(
            let mut #server_ident = {
                let __plex_server_url = ::std::env::var("PLEX_API_AUTH_TOKEN");
                assert!(__plex_server_url.is_ok(), "PLEX_SERVER_URL must be set");
                ::plex_api::Server::new(__plex_server_url.unwrap(), #plex_client_ident)
                    .expect("failed to create myplex client")
            }
            #fn_block
        );
    }

    TokenStream::from(quote! {
        #[tokio::test]
        #[cfg_attr(not(feature = "tests_only_online"), ignore = "Feature tests_only_online not set, only offline tests will be run.")]
        #function_prefix
        async fn #fn_ident() #fn_return_type {
            let #plex_client_ident = {
                let mut builder = ::plex_api::ClientBuilder::default();

                let __plex_auth_token = ::std::env::var("PLEX_API_AUTH_TOKEN");
                let __plex_client_id = ::std::env::var("X_PLEX_CLIENT_IDENTIFIER");
                let __plex_api_username = ::std::env::var("PLEX_API_USERNAME");
                let __plex_api_password = ::std::env::var("PLEX_API_PASSWORD");
                let __allow_auth_by_token = #allow_auth_by_token;
                let mut __was_token_set = false;

                if __plex_client_id.is_ok() {
                    builder = builder.set_x_plex_client_identifier(__plex_client_id.unwrap());
                }

                if __plex_auth_token.is_ok() && __allow_auth_by_token {
                    let __plex_auth_token = __plex_auth_token.unwrap();
                    if !__plex_auth_token.is_empty() {
                        builder = builder.set_x_plex_token(__plex_auth_token);
                        __was_token_set = true;
                    }
                }

                if !__was_token_set && __plex_api_username.is_ok() && __plex_api_password.is_ok() {
                    let __tmp_myplex = ::plex_api::MyPlexBuilder::default()
                        .set_username_and_password(&__plex_api_username.unwrap(), &__plex_api_password.unwrap())
                        .build()
                        .await
                        .expect("unable to login using the provided username & password");
                    builder = builder.set_x_plex_token(__tmp_myplex.client().x_plex_token().to_owned());
                }

                builder.build().expect("failed to build plex client")
            };

            #fn_block
        }
    })
}

#[proc_macro_attribute]
pub fn async_offline_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as OfflineTestArgs);

    let fn_type = parse_macro_input!(input as ItemFn);
    let fn_signature = fn_type.sig;
    let fn_ident = fn_signature.ident;
    let fn_arguments = fn_signature.inputs;
    let fn_return_type = fn_signature.output;

    let mut myplex_argument: Option<Box<Pat>> = None;
    let mut client_argument: Option<Box<Pat>> = None;
    let mut server_argument: Option<Box<Pat>> = None;

    for arg in fn_arguments {
        if let FnArg::Typed(pat_type) = arg {
            let ty = &pat_type.ty;
            match quote!(#ty).to_string().as_str() {
                "MyPlex" => myplex_argument = Some(pat_type.pat),
                "Client" => client_argument = Some(pat_type.pat),
                "Server" => server_argument = Some(pat_type.pat),
                _ => {}
            }
        }
    }

    let fn_block = fn_type.block;

    let mut fn_block = quote!(
        #fn_block
    );

    if myplex_argument.is_some() || client_argument.is_some() || server_argument.is_some() {
        let auth_token = format!("auth_token_offline_{}", fn_ident);

        let plex_client_ident: Ident = match client_argument {
            Some(pat) => Ident::new(&quote!(#pat).to_string(), Span::call_site()),
            None => Ident::new("__plex_client", Span::call_site()),
        };

        if let Some(myplex_argument) = myplex_argument {
            let myplex_ident = Ident::new(&quote!(#myplex_argument).to_string(), Span::call_site());

            let signing_mock_file = match args.user_type {
                UserType::Free => "tests/files/myplex/api/v2/user/user_info_free.json",
                UserType::FreeGuest => "tests/files/myplex/api/v2/user/user_info_free.json",
                UserType::PlexPass => "tests/files/myplex/api/v2/user/user_info_plexpass.json",
                UserType::PlexPassGuest => {
                    "tests/files/myplex/api/v2/user/user_info_plexpass_guest.json"
                }
            };

            fn_block = quote!(
                let mut #myplex_ident = {
                    let m = ::mockito::mock("GET", ::plex_api::url::MYPLEX_USER_INFO_PATH)
                        .with_status(200)
                        .with_header("content-type", "text/json")
                        .with_body_from_file(#signing_mock_file)
                        .match_body("")
                        .create();

                    let ret = ::plex_api::MyPlexBuilder::default()
                        .set_client(#plex_client_ident)
                        .build()
                        .await
                        .expect("failed to create myplex client");

                    ::mockito::reset();

                    ret
                };
                #fn_block
            );
        }

        if let Some(server_argument) = server_argument {
            let server_ident = Ident::new(&quote!(#server_argument).to_string(), Span::call_site());

            fn_block = quote!(
                let mut #server_ident = ::plex_api::Server::new(::mockito::server_url(), #plex_client_ident)
                        .expect("failed to create myplex client");
                #fn_block
            );
        }

        fn_block = quote!(
            let #plex_client_ident = {
                use ::isahc::config::Configurable as _;
                ::plex_api::ClientBuilder::default()
                    .set_api_url(::mockito::server_url())
                    .set_http_client(
                        ::isahc::HttpClient::builder()
                            // We're doing everything locally and using static mocks, no reasons to have big timeouts
                            .timeout(::std::time::Duration::from_secs(2))
                            .connect_timeout(::std::time::Duration::from_secs(1))

                            // Normally Plex doesn't do redirects and None is the default value in our default client
                            .redirect_policy(::isahc::config::RedirectPolicy::None)

                            // mockito does not support Expect-100 header, so we disabling it here
                            .expect_continue(::isahc::config::ExpectContinue::disabled())
                            .build()
                            .expect("failed to create testing http client")
                    )
                    .set_x_plex_token(#auth_token.to_owned())
                    .build()
                    .expect("failed to build plex client")
            };

            #fn_block
        );
    }

    TokenStream::from(quote! {
        #[tokio::test]
        #[cfg_attr(feature = "tests_only_online", ignore = "Feature tests_only_online is set, running only online tests.")]
        async fn #fn_ident() #fn_return_type {
            #fn_block
        }
    })
}
