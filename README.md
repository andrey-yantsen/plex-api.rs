[![crates](https://img.shields.io/crates/v/plex-api.svg)](https://crates.io/crates/plex-api) [![dependency status](https://deps.rs/repo/github/andrey-yantsen/plex-api.rs/status.svg)](https://deps.rs/repo/github/andrey-yantsen/plex-api.rs) [![tests status](https://github.com/andrey-yantsen/plex-api.rs/workflows/Test%20everything/badge.svg)](https://github.com/andrey-yantsen/plex-api.rs/actions?query=workflow%3A%22Test+everything%22)

# Work in progress

Sorry, if you're looking for a ready-to-use API, it's definitely not this one.

My goal (not sure, when I would be able to achieve it, or even if I'll achieve it) is to create an API, similar to [python-plexapi](https://github.com/pkkid/python-plexapi). Actually, to be honest, my real goal is to rewrite my [plexiglas](https://github.com/andrey-yantsen/plexiglas) project into Rust from Python :) And to "simplify" the task I've decided to have a separate project with the API.

Any help is welcome.

As the starting point I'd like to have an easy way to bootstrap test env (especially in travis). You can check out what I've done for [python-plexapi](https://github.com/pkkid/python-plexapi/blob/master/tools/plex-bootstraptest.py) in terms of the env: there is a script which creates a new Plex Server instance in docker and populates the library with some stub media, and assigns this shiny new server to MyPlex account, if required.

# TODO

* [X] MyPlex access
* [ ] CLI command to bootstrap new Plex server
   * [X] Requesting claim-token from MyPlex
   * [ ] Accept EULA
   * [ ] Creating library-like files structure (probably it'd easier to use git-lfs for storing file stubs)
   * [ ] Library section creation
   * [ ] Notifications handling, to be able to determine if the library initialisation complete
   * [ ] Granting access to a managed user for freshly created server
* [ ] MobileSync
    * [ ] Read
    * [ ] Download
