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

I don't have much exprience with testing in Rust, so please write those the way you see fit.
At the same time, I created who macroses to simplify the ingeration testing:

- `#[plex_api_test_helper::async_offline_test]`
- `#[plex_api_test_helper::async_online_test]`

Please use those when writing integration tests.

Thanks,
Andrey Yantsen
