# How to contribute

I'm delighted you're reading this, and I hope you'll find this project easy to jump in.

## First steps

The project uses [cargo-husky](https://crates.io/crates/cargo-husky) for managing the git
hooks. After you cloned the repository you need to run `cargo test` at least once, to
install the correct hooks configuration.

## Testing

The project provides a sample Plex library for testing, feel free to extend it the way you
see fit.

There're multiple test types available:

* Offline tests, with mock data.
* Online tests using a claimed (i.e. assigned to a MyPlex account) local docker container.
* Online tests using an unclaimed local docker container.

The easiest way to run all the available tests is by executing the following command:

```
cargo xtask tests
```

## Committing

Please follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) when writing commit
messages.

## Writing tests

I don't have much exprience with testing in Rust, so please write the tests the way you see fit.
At the same time, I created a few macroses to reduce the boilerplate amount:
* `#[plex_api_test_helper::async_offline_test]`
* `#[plex_api_test_helper::online_test_unclaimed_server]`
* `#[plex_api_test_helper::online_test_claimed_server]`

[rstest](http://docs.rs/rstest) is used under the hood for handling the tests — this way
it's possible to use [fixtures](https://docs.rs/rstest/latest/rstest/attr.rstest.html#injecting-fixtures) and
[tests parametrization](https://docs.rs/rstest/latest/rstest/attr.rstest.html#test-parametrized-cases).

Please use those when writing integration tests.

Thanks,
Andrey Yantsen
