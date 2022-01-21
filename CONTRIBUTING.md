# How to contribute

I'm delighted you're reading this, and I hope you'll find this project easy to jump in.

## Testing

The project provides a sample Plex library for testing, feel free to extend it the way you
see fit.

The easiest way to run all the available tests is by executing the following command:

```
cargo xtask tests
```

## Committing

Please follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) when writing commit
messages.

## Writing tests

I don't have much exprience with testing in Rust, so please write the tests the way you see fit.
At the same time, I created a macros to reduce the boilerplate amount: `#[plex_api_test_helper::async_offline_test]`. Under the hood, [rstest](http://docs.rs/rstest)
is used for handling the tests — this way it's possible to use [fixtures](https://docs.rs/rstest/latest/rstest/attr.rstest.html#injecting-fixtures) and [tests parametrization](https://docs.rs/rstest/latest/rstest/attr.rstest.html#test-parametrized-cases).

Please use those when writing integration tests.

Thanks,
Andrey Yantsen
