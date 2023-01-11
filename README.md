[![crates](https://img.shields.io/crates/v/plex-api.svg)](https://crates.io/crates/plex-api)
[![tests status](https://github.com/andrey-yantsen/plex-api.rs/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/andrey-yantsen/plex-api.rs/actions/workflows/test.yml?query=branch%3Amain)
[![codecov](https://codecov.io/gh/andrey-yantsen/plex-api.rs/branch/main/graph/badge.svg?token=hrpICjrn5q)](https://codecov.io/gh/andrey-yantsen/plex-api.rs)
[![Join the chat at https://gitter.im/plex-api-rs/community](https://badges.gitter.im/plex-api-rs/community.svg)](https://gitter.im/plex-api-rs/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

# Work in progress

Sorry, if you're looking for a ready-to-use API, it's definitely not this one.

My goal (not sure, when I would be able to achieve it, or even if I'll achieve it) is to create an API, similar to
[python-plexapi](https://github.com/pkkid/python-plexapi). Actually, to be honest, my final goal is to rewrite my
[plexiglas](https://github.com/andrey-yantsen/plexiglas) project into Rust from Python :) And to "simplify" the task
I've decided to have a separate project with the API.

Any help is welcome. And just in case: I'm far from being an expert in Rust, so if you know how to do things in proper,
Rust-way — feel free to create an issue (or, better, pull-request) to correct the code.

As the starting point I'd like to have an easy way to bootstrap test env. You can check out what I've done for
[python-plexapi](https://github.com/pkkid/python-plexapi/blob/master/tools/plex-bootstraptest.py) in terms
of the env: there is a script which creates a new Plex Server instance in docker and populates the library with some
stub media, and assigns this shiny new server to MyPlex account, if required.

# Supported plex versions

The codebase is regularly tested against three different PMS versions. The
exact list of the releases is generated automatically on a daily basis, to keep
up with all the recent PMS versions. We take every other minor release from
the latest 5 minor releases. At the moment the following versions are
tested:

<!-- plex releases list start -->
* 1.30.1.6562-915986d62
* 1.28.2.6151-914ddd2b3
* 1.26.2.5797-5bd057d2b
<!-- plex releases list end -->

# TODO

- [x] MyPlex access
- [ ] CLI command to bootstrap new Plex server
  - [x] Requesting claim-token from MyPlex
  - [ ] Change server's settings
  - [ ] Library section creation
  - [ ] Notifications handling, to be able to determine if the library initialisation complete
  - [ ] Granting access to a managed user for freshly created server
- [ ] MobileSync
  - [ ] Read
  - [ ] Download

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

You can find the contribution documentation in the [CONTRIBUTING.md](./CONTRIBUTING.md) file.
