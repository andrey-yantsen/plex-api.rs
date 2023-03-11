# How to contribute

I'm delighted you're reading this, and I hope you'll find this project easy to jump in.

## First steps

The project uses [cargo-husky](https://crates.io/crates/cargo-husky) for managing the git
hooks. After you cloned the repository you need to run `cargo test` at least once, to
install the correct hooks configuration.

## Testing

The project provides a sample Plex library for testing, feel free to extend it the way you
see fit. The `cargo xtask plex-data` command populates the test library from the stub data.
If you need to add additional media to the library you should modify the lists in
[`crates/xtask/src/plex_data.rs`](crates/xtask/src/plex_data.rs). To modify the library structure
such as categories and playlists use the `cargo xtask modify-data` command to start plex against
the test data and use the web interface to make the modifications you need. Note that the first
time you run the server against the test data you will have to accept Plex's terms of service.
Assuming you have read and agreed to them you can do this from the command line:

```shell
cargo run -p plex-cli -- -s {server_url} preferences set --i-know-what-i-am-doing -k AcceptedEULA -v true
```

There are multiple test types available:

* Offline tests, with mock data.
* Online tests using a claimed (i.e. assigned to a MyPlex account) local docker container.
* Online tests using an unclaimed local docker container.

The easiest way to run all the available tests is by executing the following
commands (they heavily use docker under the hood, please ensure you have it
installed and running):

```shell
cargo xtask test --token <YOUR_PLEX_API_TOKEN>
cargo xtask test --online --token <YOUR_PLEX_API_TOKEN> --deny-unknown-fields
```

You can omit the `--token <YOUR_PLEX_API_TOKEN>` argument — in this case only
tests with an unclaimed server will be executed.

The first command will run the offline tests with ensuring that all the mocked
data can be parsed completely (i.e. the `tests_deny_unknown_fields` feature will
be enabled), then the online tests using an unclaimed server, and then online
tests using the claimed server. This ensures that your code "works in general".

The second command will run the online tests over claimed server (or an
unclaimed one, if you run it without providing the token), but this time it will
fail the tests if an unexpected struct field or enum value were met in any of
the API responses. This is useful when you add some new functionality, it
ensures that all the fields returned by the API are parsed.

You can get your Plex API token by executing the following command and following
the on-screen instructions:

```shell
cargo run -p plex-api --example get-token
```

Tokens received this way can be used multiple times, so feel free to store one
and reuse. You can destroy an obsolete token either by removing it from the
[Authorized Devices](https://app.plex.tv/desktop/#!/settings/devices/all) list,
or by executing another command:

```shell
cargo run -p plex-api --example signout
```

## Committing

Please follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) when writing commit
messages.

## Writing tests

I don't have much exprience with testing in Rust, so please write the tests the
way you see fit. At the same time, I created a few macros to reduce the
boilerplate amount:

* `#[plex_api_test_helper::offline_test]`
* `#[plex_api_test_helper::online_test_myplex]`
* `#[plex_api_test_helper::online_test_unclaimed_server]`
* `#[plex_api_test_helper::online_test_claimed_server]`
* `#[plex_api_test_helper::online_test_non_shared_server]`
* `#[plex_api_test_helper::online_test]`

I hope it's clear from their names when to use any of them.

[rstest](http://docs.rs/rstest) is used under the hood for handling the tests — this way
it's possible to use [fixtures](https://docs.rs/rstest/latest/rstest/attr.rstest.html#injecting-fixtures) and
[tests parametrization](https://docs.rs/rstest/latest/rstest/attr.rstest.html#test-parametrized-cases).

Please use those when writing integration tests.

Thanks,
Andrey Yantsen
